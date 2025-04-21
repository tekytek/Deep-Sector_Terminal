// Client-server diagnostics module for Space Trader game
// Provides tools for testing and debugging client-server communications

use std::net::{TcpStream, SocketAddr};
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::collections::VecDeque;
use std::thread;
use std::error::Error;
use rand::Rng;

// Import our logging macros
use crate::{log_info, log_debug, log_error};

// ConnectionHealth tracks the health of a client-server connection
#[derive(Debug)]
pub struct ConnectionHealth {
    // Unique identifier for this connection
    connection_id: String,
    
    // Statistics
    send_count: AtomicU32,
    receive_count: AtomicU32,
    send_failures: AtomicU32,
    receive_failures: AtomicU32,
    
    // Last send/receive timestamps
    last_send: Mutex<Option<Instant>>,
    last_receive: Mutex<Option<Instant>>,
    
    // Recent send/receive latencies (in milliseconds)
    send_latencies: Mutex<VecDeque<u64>>,
    receive_latencies: Mutex<VecDeque<u64>>,
    
    // Recent errors
    recent_errors: Mutex<VecDeque<(Instant, String)>>,
    
    // Maximum number of items to store
    max_history: usize,
}

impl ConnectionHealth {
    // Create a new connection health tracker
    pub fn new(connection_id: &str) -> Self {
        const DEFAULT_HISTORY_SIZE: usize = 100;
        
        Self {
            connection_id: connection_id.to_string(),
            send_count: AtomicU32::new(0),
            receive_count: AtomicU32::new(0),
            send_failures: AtomicU32::new(0),
            receive_failures: AtomicU32::new(0),
            last_send: Mutex::new(None),
            last_receive: Mutex::new(None),
            send_latencies: Mutex::new(VecDeque::with_capacity(DEFAULT_HISTORY_SIZE)),
            receive_latencies: Mutex::new(VecDeque::with_capacity(DEFAULT_HISTORY_SIZE)),
            recent_errors: Mutex::new(VecDeque::with_capacity(DEFAULT_HISTORY_SIZE)),
            max_history: DEFAULT_HISTORY_SIZE,
        }
    }
    
    // Record a successful send
    pub fn record_send(&self) {
        self.send_count.fetch_add(1, Ordering::SeqCst);
        *self.last_send.lock().unwrap() = Some(Instant::now());
    }
    
    // Record a successful receive
    pub fn record_receive(&self) {
        self.receive_count.fetch_add(1, Ordering::SeqCst);
        *self.last_receive.lock().unwrap() = Some(Instant::now());
    }
    
    // Record a send failure
    pub fn record_send_failure(&self, error: &str) {
        self.send_failures.fetch_add(1, Ordering::SeqCst);
        
        let mut errors = self.recent_errors.lock().unwrap();
        errors.push_back((Instant::now(), format!("Send error: {}", error)));
        
        if errors.len() > self.max_history {
            errors.pop_front();
        }
    }
    
    // Record a receive failure
    pub fn record_receive_failure(&self, error: &str) {
        self.receive_failures.fetch_add(1, Ordering::SeqCst);
        
        let mut errors = self.recent_errors.lock().unwrap();
        errors.push_back((Instant::now(), format!("Receive error: {}", error)));
        
        if errors.len() > self.max_history {
            errors.pop_front();
        }
    }
    
    // Record send latency
    pub fn record_send_latency(&self, latency_ms: u64) {
        let mut latencies = self.send_latencies.lock().unwrap();
        latencies.push_back(latency_ms);
        
        if latencies.len() > self.max_history {
            latencies.pop_front();
        }
    }
    
    // Record receive latency
    pub fn record_receive_latency(&self, latency_ms: u64) {
        let mut latencies = self.receive_latencies.lock().unwrap();
        latencies.push_back(latency_ms);
        
        if latencies.len() > self.max_history {
            latencies.pop_front();
        }
    }
    
    // Check if the connection is healthy
    pub fn is_healthy(&self) -> bool {
        // Connection is considered unhealthy if:
        // 1. There's been recent failures (>10% of operations)
        // 2. No activity in the past 60 seconds (if there's been any activity at all)
        
        let total_ops = self.send_count.load(Ordering::SeqCst) + self.receive_count.load(Ordering::SeqCst);
        let total_failures = self.send_failures.load(Ordering::SeqCst) + self.receive_failures.load(Ordering::SeqCst);
        
        // No operations yet, consider it healthy
        if total_ops == 0 {
            return true;
        }
        
        // Failure rate too high
        if total_failures > 0 && (total_failures as f64 / total_ops as f64) > 0.1 {
            return false;
        }
        
        // Check for recent activity
        let last_send = self.last_send.lock().unwrap();
        let last_receive = self.last_receive.lock().unwrap();
        
        let now = Instant::now();
        
        if let Some(last) = *last_send {
            if now.duration_since(last) > Duration::from_secs(60) {
                return false;
            }
        }
        
        if let Some(last) = *last_receive {
            if now.duration_since(last) > Duration::from_secs(60) {
                return false;
            }
        }
        
        true
    }
    
    // Get average latencies
    pub fn get_average_latencies(&self) -> (Option<f64>, Option<f64>) {
        let send_latencies = self.send_latencies.lock().unwrap();
        let receive_latencies = self.receive_latencies.lock().unwrap();
        
        let avg_send = if !send_latencies.is_empty() {
            Some(send_latencies.iter().sum::<u64>() as f64 / send_latencies.len() as f64)
        } else {
            None
        };
        
        let avg_receive = if !receive_latencies.is_empty() {
            Some(receive_latencies.iter().sum::<u64>() as f64 / receive_latencies.len() as f64)
        } else {
            None
        };
        
        (avg_send, avg_receive)
    }
    
    // Generate a health report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("Connection Health Report for {}\n", self.connection_id));
        report.push_str("===============================\n\n");
        
        let send_count = self.send_count.load(Ordering::SeqCst);
        let receive_count = self.receive_count.load(Ordering::SeqCst);
        let send_failures = self.send_failures.load(Ordering::SeqCst);
        let receive_failures = self.receive_failures.load(Ordering::SeqCst);
        
        report.push_str(&format!("Status: {}\n\n", if self.is_healthy() { "HEALTHY" } else { "UNHEALTHY" }));
        
        report.push_str("Activity:\n");
        report.push_str(&format!("  Send operations:    {}\n", send_count));
        report.push_str(&format!("  Receive operations: {}\n", receive_count));
        report.push_str(&format!("  Send failures:      {}\n", send_failures));
        report.push_str(&format!("  Receive failures:   {}\n", receive_failures));
        
        if send_count > 0 {
            report.push_str(&format!("  Send success rate:   {:.1}%\n", 
                                    (send_count - send_failures) as f64 / send_count as f64 * 100.0));
        }
        
        if receive_count > 0 {
            report.push_str(&format!("  Receive success rate: {:.1}%\n", 
                                    (receive_count - receive_failures) as f64 / receive_count as f64 * 100.0));
        }
        
        report.push_str("\nLatency:\n");
        
        let (avg_send, avg_receive) = self.get_average_latencies();
        
        if let Some(avg) = avg_send {
            report.push_str(&format!("  Average send latency:    {:.2} ms\n", avg));
        } else {
            report.push_str("  Average send latency:    No data\n");
        }
        
        if let Some(avg) = avg_receive {
            report.push_str(&format!("  Average receive latency: {:.2} ms\n", avg));
        } else {
            report.push_str("  Average receive latency: No data\n");
        }
        
        // Recent errors
        let errors = self.recent_errors.lock().unwrap();
        
        if !errors.is_empty() {
            report.push_str("\nRecent Errors:\n");
            
            for (i, (timestamp, error)) in errors.iter().enumerate().take(5) {
                let elapsed = timestamp.elapsed();
                report.push_str(&format!("  {}. ({} ago) {}\n", 
                                       i + 1,
                                       format_duration(elapsed),
                                       error));
            }
        }
        
        report
    }
}

// Helper function to format a duration in a human-readable way
fn format_duration(duration: Duration) -> String {
    if duration.as_secs() < 60 {
        format!("{:.1} sec", duration.as_secs_f64())
    } else if duration.as_secs() < 3600 {
        format!("{:.1} min", duration.as_secs_f64() / 60.0)
    } else {
        format!("{:.1} hr", duration.as_secs_f64() / 3600.0)
    }
}

// Network condition simulator for testing
#[derive(Debug)]
pub struct NetworkSimulator {
    // Is the simulator enabled
    enabled: AtomicBool,
    
    // Simulated latency in milliseconds
    latency_ms: AtomicU32,
    
    // Packet loss percentage (0-100)
    packet_loss_percent: AtomicU32,
    
    // Packet corruption percentage (0-100)
    corruption_percent: AtomicU32,
    
    // Packet duplication percentage (0-100)
    duplication_percent: AtomicU32,
    
    // Packet reordering percentage (0-100)
    reordering_percent: AtomicU32,
    
    // Random number generator
    rng: Mutex<rand::rngs::ThreadRng>,
}

impl NetworkSimulator {
    // Create a new simulator with default settings
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            latency_ms: AtomicU32::new(0),
            packet_loss_percent: AtomicU32::new(0),
            corruption_percent: AtomicU32::new(0),
            duplication_percent: AtomicU32::new(0),
            reordering_percent: AtomicU32::new(0),
            rng: Mutex::new(rand::thread_rng()),
        }
    }
    
    // Enable the simulator
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }
    
    // Disable the simulator
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }
    
    // Check if simulator is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }
    
    // Set simulated latency in milliseconds
    pub fn set_latency(&self, latency_ms: u32) {
        self.latency_ms.store(latency_ms, Ordering::SeqCst);
    }
    
    // Set packet loss percentage (0-100)
    pub fn set_packet_loss(&self, percent: u32) {
        let clamped = percent.min(100);
        self.packet_loss_percent.store(clamped, Ordering::SeqCst);
    }
    
    // Set packet corruption percentage (0-100)
    pub fn set_corruption(&self, percent: u32) {
        let clamped = percent.min(100);
        self.corruption_percent.store(clamped, Ordering::SeqCst);
    }
    
    // Set packet duplication percentage (0-100)
    pub fn set_duplication(&self, percent: u32) {
        let clamped = percent.min(100);
        self.duplication_percent.store(clamped, Ordering::SeqCst);
    }
    
    // Set packet reordering percentage (0-100)
    pub fn set_reordering(&self, percent: u32) {
        let clamped = percent.min(100);
        self.reordering_percent.store(clamped, Ordering::SeqCst);
    }
    
    // Process an outgoing message
    pub fn process_outgoing(&self) -> bool {
        if !self.is_enabled() {
            return true;
        }
        
        let mut rng = self.rng.lock().unwrap();
        
        // Check for packet loss
        let loss_pct = self.packet_loss_percent.load(Ordering::SeqCst);
        if loss_pct > 0 && rng.gen_range(0..100) < loss_pct {
            // Packet lost
            return false;
        }
        
        // Simulate latency
        let latency = self.latency_ms.load(Ordering::SeqCst);
        if latency > 0 {
            thread::sleep(Duration::from_millis(latency as u64));
        }
        
        // For now, we're not actually modifying the message for corruption
        // but just returning whether it passed through the simulator
        
        true
    }
    
    // Get the current settings as a string
    pub fn get_settings(&self) -> String {
        format!(
            "Network Simulator Settings:\n\
             Enabled: {}\n\
             Latency: {} ms\n\
             Packet Loss: {}%\n\
             Corruption: {}%\n\
             Duplication: {}%\n\
             Reordering: {}%",
            self.is_enabled(),
            self.latency_ms.load(Ordering::SeqCst),
            self.packet_loss_percent.load(Ordering::SeqCst),
            self.corruption_percent.load(Ordering::SeqCst),
            self.duplication_percent.load(Ordering::SeqCst),
            self.reordering_percent.load(Ordering::SeqCst)
        )
    }
}

// Test connection to a server
pub fn test_connection(addr: SocketAddr, repeat: usize, timeout_ms: u64) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    report.push_str(&format!("Connection Test to {}:{}\n", addr.ip(), addr.port()));
    report.push_str("==========================\n\n");
    
    log_info!("Testing connection to {}:{}", addr.ip(), addr.port());
    
    // Test basic connectivity
    report.push_str("Basic Connectivity Test:\n");
    
    let start = Instant::now();
    match TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)) {
        Ok(_) => {
            let elapsed = start.elapsed();
            report.push_str(&format!("  Connection successful ({:.2} ms)\n", elapsed.as_millis()));
        },
        Err(e) => {
            report.push_str(&format!("  Connection failed: {}\n", e));
            return Ok(report);  // Return early because other tests will fail
        }
    }
    
    // Test multiple connections
    report.push_str("\nMultiple Connection Test:\n");
    
    let mut successful = 0;
    let mut total_ms = 0;
    let mut min_ms = u128::MAX;
    let mut max_ms = 0;
    
    for i in 1..=repeat {
        report.push_str(&format!("  Attempt {}: ", i));
        
        let start = Instant::now();
        match TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)) {
            Ok(_) => {
                let elapsed_ms = start.elapsed().as_millis();
                report.push_str(&format!("Success ({} ms)\n", elapsed_ms));
                
                successful += 1;
                total_ms += elapsed_ms;
                min_ms = min_ms.min(elapsed_ms);
                max_ms = max_ms.max(elapsed_ms);
            },
            Err(e) => {
                report.push_str(&format!("Failed: {}\n", e));
            }
        }
        
        // Small delay between attempts
        thread::sleep(Duration::from_millis(100));
    }
    
    // Connection summary
    report.push_str("\nConnection Summary:\n");
    report.push_str(&format!("  Success rate: {}/{} ({:.1}%)\n", 
                           successful, repeat, 
                           (successful as f64 / repeat as f64) * 100.0));
    
    if successful > 0 {
        let avg_ms = total_ms as f64 / successful as f64;
        report.push_str(&format!("  Average connect time: {:.2} ms\n", avg_ms));
        report.push_str(&format!("  Min connect time: {} ms\n", min_ms));
        report.push_str(&format!("  Max connect time: {} ms\n", max_ms));
    }
    
    Ok(report)
}

// Test bandwidth to a server
pub fn test_bandwidth(addr: SocketAddr, test_size_kb: u32, timeout_ms: u64) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    report.push_str(&format!("Bandwidth Test to {}:{}\n", addr.ip(), addr.port()));
    report.push_str("========================\n\n");
    
    log_info!("Testing bandwidth to {}:{}", addr.ip(), addr.port());
    
    // Connect to the server
    let mut stream = match TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)) {
        Ok(s) => s,
        Err(e) => {
            report.push_str(&format!("Connection failed: {}\n", e));
            return Ok(report);
        }
    };
    
    // Set read/write timeouts
    stream.set_read_timeout(Some(Duration::from_millis(timeout_ms)))?;
    stream.set_write_timeout(Some(Duration::from_millis(timeout_ms)))?;
    
    // Test upload bandwidth
    report.push_str("Upload Test:\n");
    
    // Create test data
    let data_size = test_size_kb * 1024;
    let data = vec![b'X'; data_size as usize];
    
    let start = Instant::now();
    match stream.write_all(&data) {
        Ok(_) => {
            let elapsed = start.elapsed();
            let bits = (data_size * 8) as f64;  // bits sent
            let seconds = elapsed.as_secs_f64();
            let mbps = bits / seconds / 1_000_000.0;
            
            report.push_str(&format!("  Sent {} KB in {:.2} seconds ({:.2} Mbps)\n", 
                                   test_size_kb, seconds, mbps));
        },
        Err(e) => {
            report.push_str(&format!("  Upload failed: {}\n", e));
        }
    }
    
    // Assume the server echoes the data back for a download test
    report.push_str("\nNote: Download test requires server echo support.\n");
    
    Ok(report)
}