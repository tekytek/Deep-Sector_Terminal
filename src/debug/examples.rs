// This file contains examples of how to use the debugging system

use crate::debug::{self, LogLevel};
use crate::debug::error_analysis;
use crate::debug::network::NetworkDiagnostics;
use crate::debug::client_server::{ConnectionHealth, NetworkSimulator};
use std::collections::HashMap;
use std::net::SocketAddr;

// Import the logging macros
use crate::{log_trace, log_debug, log_info, log_warn, log_error, time_function, end_timing};

/// Initialize the debug system
pub fn init_debug_system() {
    // Initialize the debug logger with default settings
    debug::init(
        Some("logs/space_trader.log"), // Log to a file
        true,                         // Also print to console
        debug::get_log_level_from_env(), // Get log level from env var or default to INFO
    );
    
    // Set module-specific log levels
    debug::set_module_level("network", LogLevel::Debug);
    debug::set_module_level("game", LogLevel::Info);
    
    // Register custom panic handler (already done in debug::init)
    
    // Log initialization
    log_info!("Debug system initialized");
    log_debug!("System information: {}", debug::system_info());
}

/// Example of basic logging
pub fn demo_basic_logging() {
    log_trace!("This is a TRACE message");
    log_debug!("This is a DEBUG message");
    log_info!("This is an INFO message");
    log_warn!("This is a WARNING message");
    log_error!("This is an ERROR message");
    
    // Timing a function
    time_function!("demo_operation");
    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(50));
}

/// Example of error recording and analysis
pub fn demo_error_analysis() {
    // Record some errors
    error_analysis::record_error(
        "database", 
        "connection_error", 
        "Failed to connect to database", 
        None
    );
    
    // Record with context
    let mut context = HashMap::new();
    context.insert("user_id".to_string(), "12345".to_string());
    context.insert("operation".to_string(), "save_game".to_string());
    
    error_analysis::record_error(
        "game_state", 
        "save_error", 
        "Failed to save game state", 
        Some(context)
    );
    
    // Generate an error report
    let report = error_analysis::generate_error_report();
    log_debug!("Error Report:\n{}", report);
    
    // Analyze error patterns
    let analysis = error_analysis::analyze_error_patterns();
    log_debug!("Error Pattern Analysis:\n{}", analysis);
}

/// Example of network diagnostics
pub fn demo_network_diagnostics() {
    // Check connectivity to a server
    let server_port = 7878;  // Default server port
    let server_host = "localhost";
    
    let is_server_accessible = NetworkDiagnostics::check_port(server_host, server_port, 3000);
    log_info!("Server accessibility check: {}", if is_server_accessible { "PASSED" } else { "FAILED" });
    
    // Ping test
    if is_server_accessible {
        let ping_results = NetworkDiagnostics::ping_host(server_host, 3);
        
        if !ping_results.is_empty() {
            let avg_ms = ping_results.iter().map(|d| d.as_millis()).sum::<u128>() as f64 / ping_results.len() as f64;
            log_info!("Average ping to {}: {:.2}ms", server_host, avg_ms);
        } else {
            log_warn!("All pings to {} failed", server_host);
        }
    }
    
    // Generate network environment report
    let env_report = NetworkDiagnostics::network_environment_report();
    log_debug!("Network Environment Report:\n{}", env_report);
}

/// Example of client-server diagnostics
pub fn demo_client_server_diagnostics() {
    // Create a connection health tracker
    let connection_id = "client_1";
    let health_tracker = std::sync::Arc::new(std::sync::Mutex::new(ConnectionHealth::new(connection_id)));
    
    // Record some events
    if let Ok(mut health) = health_tracker.lock() {
        health.record_send();
        health.record_receive();
        
        // Simulate a connection error
        health.record_send_failure("Timeout waiting for acknowledgment");
        
        // Generate health report
        let report = health.generate_report();
        log_debug!("Connection Health Report:\n{}", report);
        
        // Check health status
        let is_healthy = health.is_healthy();
        log_info!("Connection health status: {}", if is_healthy { "HEALTHY" } else { "UNHEALTHY" });
    }
    
    // Network condition simulator
    let mut simulator = NetworkSimulator::new();
    simulator.set_latency(250);       // 250ms latency
    simulator.set_packet_loss(5);     // 5% packet loss
    
    // Enable/disable the simulator
    simulator.enable();
    log_info!("Network simulator enabled");
    
    // Process a message through the simulator
    if simulator.process_outgoing() {
        log_debug!("Message passed through network simulator");
    } else {
        log_debug!("Message dropped by network simulator");
    }
    
    // Disable the simulator
    simulator.disable();
}

/// Example of running a connection test
pub fn run_connection_test(addr_str: &str) {
    log_info!("Starting connection test to {}", addr_str);
    
    match addr_str.parse::<SocketAddr>() {
        Ok(addr) => {
            match crate::debug::client_server::test_connection(addr, 5, 5000) {
                Ok(report) => {
                    log_info!("Connection test completed successfully");
                    log_info!("Test Results:\n{}", report);
                },
                Err(e) => {
                    log_error!("Connection test failed: {}", e);
                }
            }
            
            // Also run a bandwidth test
            match crate::debug::client_server::test_bandwidth(addr, 64, 10000) {
                Ok(report) => {
                    log_info!("Bandwidth test completed successfully");
                    log_info!("Test Results:\n{}", report);
                },
                Err(e) => {
                    log_error!("Bandwidth test failed: {}", e);
                }
            }
        },
        Err(e) => {
            log_error!("Invalid socket address format: {}", e);
        }
    }
}

/// How to analyze a stack trace
pub fn demonstrate_stack_trace() {
    // Get a simple stack trace
    let trace = debug::simple_stack_trace();
    log_debug!("Stack trace from current location:\n{}", trace);
}

/// Full demo showing usage of all debugging features
pub fn run_full_demo() {
    init_debug_system();
    
    log_info!("Starting debugging system demonstration");
    
    demo_basic_logging();
    demo_error_analysis();
    demo_network_diagnostics();
    demo_client_server_diagnostics();
    
    // Connection test requires a running server
    // Uncomment to test when server is running:
    // run_connection_test("127.0.0.1:7878");
    
    demonstrate_stack_trace();
    
    log_info!("Debugging system demonstration completed");
}