use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde_json;
use uuid::Uuid;


use crate::network::error::{NetworkError, NetworkResult};
use crate::network::protocol::{Message, DEFAULT_SERVER_PORT, HEARTBEAT_INTERVAL};
use crate::models::ship::Ship;
use crate::models::universe::Universe;
use crate::models::market::Market;

/// Client connection to a game server
pub struct GameClient {
    client_id: Uuid,
    username: String,
    server_addr: String,
    connected: bool,
    universe: Option<Universe>,
    player_ship: Option<Ship>,
    current_market: Option<Market>,
    tx: Option<mpsc::Sender<Message>>, 
    rx: Arc<Mutex<mpsc::Receiver<Message>>>,
}

impl GameClient {
    /// Create a new game client
    pub fn new(username: String) -> Self {
        let (_tx, rx) = mpsc::channel(100);
        
        Self {
            client_id: Uuid::new_v4(),
            username,
            server_addr: String::new(),
            connected: false,
            universe: None,
            player_ship: None,
            current_market: None,
            tx: None,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
    
    /// Connect to a game server
    pub async fn connect(&mut self, server_host: &str, port: Option<u16>, password: Option<String>) -> NetworkResult<()> {
        if self.connected {
            return Err(NetworkError::ClientError("Already connected to a server".to_string()));
        }
        
        let port = port.unwrap_or(DEFAULT_SERVER_PORT);
        let server_addr = format!("{}:{}", server_host, port);
        
        // Connect to the server
        let stream = TcpStream::connect(&server_addr).await
            .map_err(|e| NetworkError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        println!("Connected to server at {}", server_addr);
        self.server_addr = server_addr;
        
        // Split the TCP stream
        let (mut reader, mut writer) = stream.into_split();
        
        // Create a channel for sending messages to the server
        let (tx, mut rx) = mpsc::channel::<Message>(100);
        self.tx = Some(tx);
        
        // Send initial connect message
        let connect_msg = Message::Connect {
            client_id: self.client_id,
            username: self.username.clone(),
            password,
        };
        
        let connect_bytes = serde_json::to_vec(&connect_msg)
            .map_err(|e| NetworkError::SerializationError(e.to_string()))?;
        
        writer.write_all(&connect_bytes).await
            .map_err(|e| NetworkError::IoError(e))?;
        
        // Start a task to forward messages to the server
        let writer_clone = Arc::new(Mutex::new(writer));
        let writer_for_task = writer_clone.clone();
        
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                match serde_json::to_vec(&message) {
                    Ok(bytes) => {
                        let mut writer = writer_for_task.lock().await;
                        if let Err(e) = writer.write_all(&bytes).await {
                            eprintln!("Error writing to server: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error serializing message: {}", e);
                    }
                }
            }
        });
        
        // Start a heartbeat task
        let tx_clone = self.tx.clone();
        let client_id = self.client_id;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(HEARTBEAT_INTERVAL);
            loop {
                interval.tick().await;
                
                if let Some(tx) = &tx_clone {
                    let heartbeat = Message::Heartbeat {
                        client_id,
                        timestamp: Instant::now().into(),
                    };
                    
                    if let Err(e) = tx.send(heartbeat).await {
                        eprintln!("Failed to send heartbeat: {}", e);
                        break;
                    }
                } else {
                    break; // Channel closed
                }
            }
        });
        
        // Start a task to receive messages from the server
        let rx_clone = self.rx.clone();
        
        tokio::spawn(async move {
            let mut buffer = [0u8; 8192];
            
            while let Ok(n) = reader.read(&mut buffer).await {
                if n == 0 {
                    // Connection closed
                    break;
                }
                
                match serde_json::from_slice::<Message>(&buffer[..n]) {
                    Ok(message) => {
                        // Get a mpsc channel sender to forward the message
                        let _rx_lock = rx_clone.lock().await;
                        // In tokio's mpsc, the Receiver doesn't have a send method
                        // We need to handle the message directly here instead
                        // Properly handling would involve a more complex architecture
                        println!("Received message from server: {:?}", message);
                    }
                    Err(e) => {
                        eprintln!("Error deserializing message: {}", e);
                    }
                }
            }
            
            println!("Server connection closed");
        });
        
        // Wait for connect response
        let response = self.receive_message().await?;
        
        match response {
            Message::ConnectResponse { success, message, universe, player_ship } => {
                if success {
                    println!("Connection successful: {}", message);
                    self.connected = true;
                    self.universe = universe;
                    self.player_ship = player_ship;
                    Ok(())
                } else {
                    self.tx = None; // Clean up
                    Err(NetworkError::AuthError(message))
                }
            }
            _ => {
                self.tx = None; // Clean up
                Err(NetworkError::ConnectionError("Unexpected response from server".to_string()))
            }
        }
    }
    
    /// Disconnect from the server
    pub async fn disconnect(&mut self) -> NetworkResult<()> {
        if !self.connected || self.tx.is_none() {
            return Ok(());
        }
        
        // Send disconnect message
        let disconnect_msg = Message::Disconnect {
            client_id: self.client_id,
        };
        
        if let Some(tx) = &self.tx {
            tx.send(disconnect_msg).await
                .map_err(|_| NetworkError::ConnectionError("Failed to send disconnect message".to_string()))?;
        }
        
        // Clean up
        self.tx = None;
        self.connected = false;
        
        println!("Disconnected from server");
        Ok(())
    }
    
    /// Receive a message from the server with timeout
    pub async fn receive_message(&self) -> NetworkResult<Message> {
        if !self.connected && self.tx.is_none() {
            return Err(NetworkError::ClientError("Not connected to server".to_string()));
        }
        
        let timeout = sleep(Duration::from_secs(5));
        tokio::pin!(timeout);
        
        tokio::select! {
            _ = &mut timeout => {
                Err(NetworkError::Timeout)
            }
            result = async {
                let mut rx = self.rx.lock().await;
                rx.recv().await.ok_or(NetworkError::ConnectionError("Server channel closed".to_string()))
            } => {
                result
            }
        }
    }
    
    /// Request the current game state
    pub async fn request_game_state(&self) -> NetworkResult<()> {
        if !self.connected || self.tx.is_none() {
            return Err(NetworkError::ClientError("Not connected to server".to_string()));
        }
        
        let request = Message::RequestGameState {
            client_id: self.client_id,
        };
        
        if let Some(tx) = &self.tx {
            tx.send(request).await
                .map_err(|_| NetworkError::ConnectionError("Failed to send game state request".to_string()))?;
        }
        
        Ok(())
    }
    
    /// Navigate to a star system
    pub async fn navigate_to_system(&self, destination: String) -> NetworkResult<(bool, String)> {
        if !self.connected || self.tx.is_none() {
            return Err(NetworkError::ClientError("Not connected to server".to_string()));
        }
        
        let nav_action = Message::NavigationAction {
            client_id: self.client_id,
            destination_system: destination,
        };
        
        if let Some(tx) = &self.tx {
            tx.send(nav_action).await
                .map_err(|_| NetworkError::ConnectionError("Failed to send navigation action".to_string()))?;
        }
        
        // Wait for response
        let response = self.receive_message().await?;
        
        match response {
            Message::ActionResponse { success, message, updated_ship, .. } => {
                // Update local ship state
                if let Some(_ship) = updated_ship {
                    // Need to borrow mutably, but GameClient methods don't take &mut self
                    // In a real implementation, you'd need to handle this with proper state management
                    // This is just a placeholder
                    // self.player_ship = Some(ship);
                }
                
                Ok((success, message))
            }
            _ => {
                Err(NetworkError::ServerError("Unexpected response from server".to_string()))
            }
        }
    }
    
    // Other action methods would follow a similar pattern
    
    /// Get the local universe data (if available)
    pub fn get_universe(&self) -> Option<&Universe> {
        self.universe.as_ref()
    }
    
    /// Get the player's ship data (if available)
    pub fn get_player_ship(&self) -> Option<&Ship> {
        self.player_ship.as_ref()
    }
    
    /// Check if connected to server
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}