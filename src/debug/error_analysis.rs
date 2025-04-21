// Error analysis module for Space Trader game
// This module provides tools for tracking, analyzing, and fixing errors

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use std::fmt;

// Import our logging macros
use crate::{log_info, log_debug, log_error};

// Define an error record structure
#[derive(Debug, Clone)]
pub struct ErrorRecord {
    pub timestamp: SystemTime,
    pub module: String,
    pub error_type: String,
    pub message: String,
    pub context: Option<HashMap<String, String>>,
}

impl fmt::Display for ErrorRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timestamp = chrono::DateTime::<chrono::Local>::from(self.timestamp)
            .format("%Y-%m-%d %H:%M:%S%.3f");
            
        write!(f, "[{}] {}::{}: {}", timestamp, self.module, self.error_type, self.message)?;
        
        if let Some(context) = &self.context {
            write!(f, "\nContext:")?;
            for (key, value) in context {
                write!(f, "\n  {}: {}", key, value)?;
            }
        }
        
        Ok(())
    }
}

// Maximum number of errors to store
const MAX_ERRORS: usize = 1000;

// Global error queue
lazy_static! {
    static ref ERROR_QUEUE: Mutex<VecDeque<ErrorRecord>> = Mutex::new(VecDeque::with_capacity(MAX_ERRORS));
}

// Record an error
pub fn record_error(
    module: &str,
    error_type: &str,
    message: &str,
    context: Option<HashMap<String, String>>
) {
    let error = ErrorRecord {
        timestamp: SystemTime::now(),
        module: module.to_string(),
        error_type: error_type.to_string(),
        message: message.to_string(),
        context,
    };
    
    log_error!("{}", error);
    
    let mut queue = ERROR_QUEUE.lock().unwrap();
    
    // Add to the queue, removing older entries if needed
    queue.push_back(error);
    if queue.len() > MAX_ERRORS {
        queue.pop_front();
    }
}

// Get all recorded errors
pub fn get_errors() -> Vec<ErrorRecord> {
    let queue = ERROR_QUEUE.lock().unwrap();
    queue.iter().cloned().collect()
}

// Filter errors by module
pub fn get_errors_by_module(module: &str) -> Vec<ErrorRecord> {
    let queue = ERROR_QUEUE.lock().unwrap();
    queue.iter()
        .filter(|e| e.module == module)
        .cloned()
        .collect()
}

// Filter errors by type
pub fn get_errors_by_type(error_type: &str) -> Vec<ErrorRecord> {
    let queue = ERROR_QUEUE.lock().unwrap();
    queue.iter()
        .filter(|e| e.error_type == error_type)
        .cloned()
        .collect()
}

// Get recent errors
pub fn get_recent_errors(duration: Duration) -> Vec<ErrorRecord> {
    let now = SystemTime::now();
    let queue = ERROR_QUEUE.lock().unwrap();
    
    queue.iter()
        .filter(|e| {
            if let Ok(elapsed) = now.duration_since(e.timestamp) {
                elapsed <= duration
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

// Clear all errors
pub fn clear_errors() {
    let mut queue = ERROR_QUEUE.lock().unwrap();
    queue.clear();
}

// Generate an error report
pub fn generate_error_report() -> String {
    let queue = ERROR_QUEUE.lock().unwrap();
    
    if queue.is_empty() {
        return "No errors recorded.".to_string();
    }
    
    let mut report = String::new();
    report.push_str(&format!("Error Report ({} errors)\n", queue.len()));
    report.push_str("=======================\n\n");
    
    // Get error counts by module
    let mut module_counts: HashMap<String, usize> = HashMap::new();
    for error in queue.iter() {
        *module_counts.entry(error.module.clone()).or_insert(0) += 1;
    }
    
    // Get error counts by type
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    for error in queue.iter() {
        *type_counts.entry(error.error_type.clone()).or_insert(0) += 1;
    }
    
    // Report summary by module
    report.push_str("Errors by Module:\n");
    for (module, count) in module_counts.iter() {
        report.push_str(&format!("  {}: {} errors\n", module, count));
    }
    report.push_str("\n");
    
    // Report summary by type
    report.push_str("Errors by Type:\n");
    for (error_type, count) in type_counts.iter() {
        report.push_str(&format!("  {}: {} errors\n", error_type, count));
    }
    report.push_str("\n");
    
    // Recent errors (last 15 minutes)
    let now = SystemTime::now();
    let mut recent_errors = 0;
    for error in queue.iter() {
        if let Ok(elapsed) = now.duration_since(error.timestamp) {
            if elapsed <= Duration::from_secs(15 * 60) {
                recent_errors += 1;
            }
        }
    }
    report.push_str(&format!("Recent errors (last 15 min): {}\n\n", recent_errors));
    
    // Most recent 10 errors
    report.push_str("Most Recent Errors:\n");
    for (i, error) in queue.iter().rev().take(10).enumerate() {
        report.push_str(&format!("\n--- Error {} ---\n{}\n", i + 1, error));
    }
    
    report
}

// Analyze error patterns for potential solutions
pub fn analyze_error_patterns() -> String {
    let queue = ERROR_QUEUE.lock().unwrap();
    
    if queue.is_empty() {
        return "No errors to analyze.".to_string();
    }
    
    let mut report = String::new();
    report.push_str("Error Pattern Analysis\n");
    report.push_str("=====================\n\n");
    
    // Analyze frequency of error types
    let mut type_frequency: HashMap<String, usize> = HashMap::new();
    for error in queue.iter() {
        *type_frequency.entry(format!("{}::{}", error.module, error.error_type)).or_insert(0) += 1;
    }
    
    // Sort by frequency
    let mut sorted_types: Vec<_> = type_frequency.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));
    
    report.push_str("Common Error Patterns:\n");
    for (error_key, count) in sorted_types.iter().take(5) {
        report.push_str(&format!("  {}: {} occurrences\n", error_key, count));
    }
    report.push_str("\n");
    
    // Group errors by modules for correlation analysis
    let modules: Vec<String> = queue.iter()
        .map(|e| e.module.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
        
    // Check if there are correlated errors across modules
    for i in 0..modules.len() {
        for j in i+1..modules.len() {
            let module_a = &modules[i];
            let module_b = &modules[j];
            
            let errors_a = queue.iter().filter(|e| &e.module == module_a).count();
            let errors_b = queue.iter().filter(|e| &e.module == module_b).count();
            
            // If both modules have errors, check for timeframe correlation
            if errors_a > 0 && errors_b > 0 {
                let mut correlated_errors = 0;
                
                for error_a in queue.iter().filter(|e| &e.module == module_a) {
                    for error_b in queue.iter().filter(|e| &e.module == module_b) {
                        if let (Ok(time_a), Ok(time_b)) = (
                            error_a.timestamp.duration_since(SystemTime::UNIX_EPOCH),
                            error_b.timestamp.duration_since(SystemTime::UNIX_EPOCH)
                        ) {
                            // Check if errors occurred within 5 seconds of each other
                            let time_diff = if time_a > time_b {
                                time_a - time_b
                            } else {
                                time_b - time_a
                            };
                            
                            if time_diff <= Duration::from_secs(5) {
                                correlated_errors += 1;
                            }
                        }
                    }
                }
                
                if correlated_errors > 0 {
                    report.push_str(&format!(
                        "Possible correlation between {} and {} modules: {} related errors\n",
                        module_a, module_b, correlated_errors
                    ));
                }
            }
        }
    }
    
    // Add potential solutions for common error types
    report.push_str("\nPotential Solutions:\n");
    
    for (error_key, _) in sorted_types.iter().take(3) {
        let (module, error_type) = error_key.split_once("::").unwrap_or((error_key, ""));
        
        // Customize solutions based on error types
        let solution = match (module, error_type) {
            ("network", "connection_error") => 
                "Check network connectivity, firewall settings, and ensure the server is running.",
                
            ("network", "timeout") => 
                "Increase timeout values or check for network congestion.",
                
            ("game", "save_error") => 
                "Check file permissions and available disk space.",
                
            ("database", _) => 
                "Verify database connection settings and schema integrity.",
                
            (_, "permission_denied") => 
                "Check file and resource permissions.",
                
            (_, _) => 
                "Review logs for more details about this error type."
        };
        
        report.push_str(&format!("  For {}: {}\n", error_key, solution));
    }
    
    report
}

// Convenience method to register a simple error
pub fn register_simple_error(module: &str, message: &str) {
    record_error(module, "error", message, None);
}

// Helper function to get error count
pub fn get_error_count() -> usize {
    ERROR_QUEUE.lock().unwrap().len()
}