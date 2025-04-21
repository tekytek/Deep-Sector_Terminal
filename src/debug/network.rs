// Network diagnostics module for Space Trader game
// Provides tools for testing network connectivity and diagnosing issues

use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use std::io::{self, Read, Write};

// Use get-if-addrs crate for network interface discovery
use get_if_addrs::{get_if_addrs, IfAddr, Interface};

// Import our logging macros
use crate::{log_info, log_debug, log_error};

// Network diagnostics utilities
pub struct NetworkDiagnostics;

impl NetworkDiagnostics {
    // Get a list of network interfaces with their IP addresses
    pub fn get_interfaces() -> io::Result<Vec<Interface>> {
        get_if_addrs()
    }
    
    // Get a summary of network interfaces as a string
    pub fn get_interfaces_summary() -> String {
        match Self::get_interfaces() {
            Ok(interfaces) => {
                let mut result = String::new();
                
                for interface in interfaces {
                    let addr_str = match interface.addr {
                        IfAddr::V4(ref addr) => format!("{}/{}", addr.ip, addr.netmask),
                        IfAddr::V6(ref addr) => format!("{}/{}", addr.ip, addr.netmask),
                    };
                    
                    result.push_str(&format!("{}: {}\n", interface.name, addr_str));
                }
                
                result
            },
            Err(e) => {
                format!("Error getting network interfaces: {}", e)
            }
        }
    }
    
    // Check if a port is open on a host
    pub fn check_port(host: &str, port: u16, timeout_ms: u64) -> bool {
        let addr = format!("{}:{}", host, port);
        
        match addr.parse::<SocketAddr>() {
            Ok(socket_addr) => {
                match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(timeout_ms)) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            },
            Err(_) => false,
        }
    }
    
    // Check if a port is available (not in use) on the local machine
    pub fn check_port_available(port: u16) -> bool {
        // Try to bind to the port to see if it's available
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
        
        match std::net::TcpListener::bind(addr) {
            Ok(_) => true, // Binding succeeded, port is available
            Err(_) => false, // Binding failed, port is in use
        }
    }
    
    // Ping a host with a simple TCP ping, return the round-trip times
    pub fn ping_host(host: &str, count: u32) -> Vec<Duration> {
        let mut results = Vec::new();
        
        // Default to a common port (80) for the ping
        let port = 80;
        let addr = format!("{}:{}", host, port);
        
        log_info!("Pinging {} with TCP ping on port {}", host, port);
        
        for i in 0..count {
            log_debug!("Ping attempt {}/{}", i + 1, count);
            
            let start = Instant::now();
            
            match TcpStream::connect(&addr) {
                Ok(_) => {
                    let duration = start.elapsed();
                    log_debug!("Ping successful: {:?}", duration);
                    results.push(duration);
                },
                Err(e) => {
                    log_error!("Ping failed: {}", e);
                }
            }
            
            // Don't sleep after the last attempt
            if i < count - 1 {
                std::thread::sleep(Duration::from_millis(1000));
            }
        }
        
        results
    }
    
    // Generate a network environment report
    pub fn network_environment_report() -> String {
        let mut report = String::new();
        
        report.push_str("Network Environment Report\n");
        report.push_str("=========================\n\n");
        
        // Interface information
        report.push_str("Network Interfaces:\n");
        report.push_str(&Self::get_interfaces_summary());
        report.push_str("\n");
        
        // Common ports status
        report.push_str("Common Ports Status (localhost):\n");
        for port in [80, 443, 8080, 7878, 7890] {
            let status = if Self::check_port_available(port) {
                "available"
            } else {
                "in use"
            };
            report.push_str(&format!("- Port {}: {}\n", port, status));
        }
        
        // External connectivity
        report.push_str("\nExternal Connectivity:\n");
        
        let common_hosts = ["google.com", "github.com", "api.github.com"];
        for host in common_hosts {
            let port = 443; // HTTPS port
            let connectivity = Self::check_port(host, port, 5000);
            report.push_str(&format!("- {}: {}\n", host, if connectivity { "reachable" } else { "unreachable" }));
        }
        
        report
    }
    
    // Measure download bandwidth from a server
    pub fn measure_bandwidth(host: &str, port: u16, size_kb: u32) -> io::Result<f64> {
        let addr = format!("{}:{}", host, port);
        
        log_info!("Measuring bandwidth from {}:{} with {}KB", host, port, size_kb);
        
        let mut stream = TcpStream::connect(&addr)?;
        
        // Send a request for the specified size
        let request = format!("GET /bandwidth?size={} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", size_kb, host);
        stream.write_all(request.as_bytes())?;
        
        let start = Instant::now();
        
        // Read the response
        let mut buffer = [0; 8192];
        let mut total_bytes = 0;
        
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    total_bytes += n;
                },
                Err(e) => return Err(e),
            }
        }
        
        let duration = start.elapsed();
        let seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
        let bits = total_bytes as f64 * 8.0;
        let mbps = bits / seconds / 1_000_000.0;
        
        log_info!("Bandwidth test complete. Received {} bytes in {:.2} seconds ({:.2} Mbps)", 
                 total_bytes, seconds, mbps);
        
        Ok(mbps)
    }
    
    // Check DNS resolution
    pub fn check_dns(hostname: &str) -> io::Result<Vec<IpAddr>> {
        use std::net::ToSocketAddrs;
        
        log_info!("Resolving hostname: {}", hostname);
        
        let addrs: Vec<_> = (hostname, 0).to_socket_addrs()?
            .map(|socket_addr| socket_addr.ip())
            .collect();
            
        log_info!("Resolved {} to {} addresses", hostname, addrs.len());
        for (i, addr) in addrs.iter().enumerate() {
            log_debug!("  Address {}: {}", i+1, addr);
        }
        
        Ok(addrs)
    }
}