// Debug module for Space Trader game
// This module provides tools for logging, diagnostics, and debugging

use std::env;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::backtrace::Backtrace;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::fmt;
use std::panic;
use std::thread;

pub mod network;
pub mod error_analysis;
pub mod client_server;
pub mod examples;

// Define log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

// Global logger configuration
struct Logger {
    file_path: Option<String>,
    print_to_console: bool,
    global_level: LogLevel,
    module_levels: HashMap<String, LogLevel>,
}

// Create a global logger instance
lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger {
        file_path: None,
        print_to_console: true,
        global_level: LogLevel::Info,
        module_levels: HashMap::new(),
    });
    
    // Global counter for tracking unique operation IDs
    pub static ref OPERATION_COUNTER: AtomicUsize = AtomicUsize::new(0);
    
    // Timing information
    static ref TIMINGS: Mutex<HashMap<String, Vec<Duration>>> = Mutex::new(HashMap::new());
}

// Initialize the debug system
pub fn init(file_path: Option<&str>, print_to_console: bool, global_level: LogLevel) {
    let mut logger = LOGGER.lock().unwrap();
    
    // Set logger configuration
    logger.file_path = file_path.map(String::from);
    logger.print_to_console = print_to_console;
    logger.global_level = global_level;
    
    // Create log directory if needed
    if let Some(path) = &logger.file_path {
        if let Some(dir) = Path::new(path).parent() {
            if !dir.exists() {
                if let Err(e) = create_dir_all(dir) {
                    eprintln!("Failed to create log directory: {}", e);
                }
            }
        }
    }
    
    // Set custom panic handler
    panic::set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::capture();
        let thread = thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");
        
        let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic payload"
        };
        
        let location = if let Some(loc) = panic_info.location() {
            format!("{}:{}", loc.file(), loc.line())
        } else {
            "unknown location".to_string()
        };
        
        let panic_message = format!(
            "PANIC in thread '{}' at {}: {}\nBacktrace:\n{:?}",
            thread_name, location, payload, backtrace
        );
        
        // Log the panic through our logging system
        log_internal(LogLevel::Error, "panic", &panic_message);
        
        // Also print to stderr
        eprintln!("{}", panic_message);
    }));
}

// Set logging level for a specific module
pub fn set_module_level(module: &str, level: LogLevel) {
    let mut logger = LOGGER.lock().unwrap();
    logger.module_levels.insert(module.to_string(), level);
}

// Get the current level for a module
pub fn get_module_level(module: &str) -> LogLevel {
    let logger = LOGGER.lock().unwrap();
    *logger.module_levels.get(module).unwrap_or(&logger.global_level)
}

// Get log level from environment variable DEBUG_LEVEL
pub fn get_log_level_from_env() -> LogLevel {
    match env::var("DEBUG_LEVEL").ok().as_deref() {
        Some("trace") => LogLevel::Trace,
        Some("debug") => LogLevel::Debug,
        Some("info") => LogLevel::Info,
        Some("warn") | Some("warning") => LogLevel::Warning,
        Some("error") => LogLevel::Error,
        _ => LogLevel::Info, // Default to Info level
    }
}

// Internal logging function (made public for macro access)
pub fn log_internal(level: LogLevel, module: &str, message: &str) {
    let logger = LOGGER.lock().unwrap();
    
    // Check if this log should be processed based on the module's level
    let module_level = logger.module_levels.get(module).unwrap_or(&logger.global_level);
    if level < *module_level {
        return;
    }
    
    // Format the log message
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let formatted_message = format!("[{}] [{}] [{}] {}", timestamp, level, module, message);
    
    // Write to console if enabled
    if logger.print_to_console {
        match level {
            LogLevel::Error => eprintln!("\x1B[31m{}\x1B[0m", formatted_message),   // Red
            LogLevel::Warning => eprintln!("\x1B[33m{}\x1B[0m", formatted_message), // Yellow
            LogLevel::Info => println!("\x1B[32m{}\x1B[0m", formatted_message),     // Green
            LogLevel::Debug => println!("\x1B[36m{}\x1B[0m", formatted_message),    // Cyan
            LogLevel::Trace => println!("\x1B[90m{}\x1B[0m", formatted_message),    // Gray
        }
    }
    
    // Write to file if configured
    if let Some(file_path) = &logger.file_path {
        let file_result = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path);
            
        match file_result {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", formatted_message) {
                    eprintln!("Failed to write to log file: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
            }
        }
    }
}

// Record timing information
pub fn record_timing(operation: &str, duration: Duration) {
    let mut timings = TIMINGS.lock().unwrap();
    timings.entry(operation.to_string())
        .or_insert_with(Vec::new)
        .push(duration);
}

// Get statistics about timing information
pub fn get_timing_stats(operation: &str) -> Option<(Duration, Duration, Duration)> {
    let timings = TIMINGS.lock().unwrap();
    
    if let Some(durations) = timings.get(operation) {
        if durations.is_empty() {
            return None;
        }
        
        let total: Duration = durations.iter().sum();
        let avg = total / durations.len() as u32;
        
        let min = durations.iter().min().cloned().unwrap_or(Duration::from_secs(0));
        let max = durations.iter().max().cloned().unwrap_or(Duration::from_secs(0));
        
        Some((min, avg, max))
    } else {
        None
    }
}

// Generate a system information report
pub fn system_info() -> String {
    let hostname = if let Ok(name) = hostname::get() {
        name.to_string_lossy().into_owned()
    } else {
        "unknown".to_string()
    };
    
    let num_cpus = num_cpus::get();
    
    format!(
        "System Information:\n\
         - Hostname: {}\n\
         - CPU Cores: {}\n\
         - OS: {}\n\
         - Memory: [Feature not implemented]\n\
         - Network Interfaces: {}\n",
        hostname,
        num_cpus,
        std::env::consts::OS,
        network::NetworkDiagnostics::get_interfaces_summary()
    )
}

// Get a simple stack trace
pub fn simple_stack_trace() -> String {
    format!("{:?}", Backtrace::capture())
}

// Macros for logging
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {{
        let module = module_path!().split("::").last().unwrap_or("unknown");
        $crate::debug::log_internal($crate::debug::LogLevel::Trace, module, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        let module = module_path!().split("::").last().unwrap_or("unknown");
        $crate::debug::log_internal($crate::debug::LogLevel::Debug, module, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        let module = module_path!().split("::").last().unwrap_or("unknown");
        $crate::debug::log_internal($crate::debug::LogLevel::Info, module, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        let module = module_path!().split("::").last().unwrap_or("unknown");
        $crate::debug::log_internal($crate::debug::LogLevel::Warning, module, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        let module = module_path!().split("::").last().unwrap_or("unknown");
        $crate::debug::log_internal($crate::debug::LogLevel::Error, module, &format!($($arg)*));
    }};
}

// Timing macro - measures execution time of a code block
#[macro_export]
macro_rules! time_function {
    ($operation:expr) => {{
        use std::sync::atomic::Ordering;
        
        let start = std::time::Instant::now();
        // Use a static counter directly inside the macro
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let op_id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let op_unique = format!("{}_{}", $operation, op_id);
        
        $crate::log_trace!("Starting operation: {}", op_unique);
        
        // Return the operation ID and start time for later use
        (op_unique, start)
    }};
}

// End timing macro - completes measurement and records the timing
#[macro_export]
macro_rules! end_timing {
    (($op_unique:expr, $start:expr)) => {{
        let duration = $start.elapsed();
        $crate::debug::record_timing(&$op_unique, duration);
        $crate::log_trace!("Completed operation: {} in {:?}", $op_unique, duration);
        duration
    }};
}

// Simple logging functions that don't rely on macros
pub fn info(msg: &str) {
    log_internal(LogLevel::Info, "main", msg);
}

pub fn debug(msg: &str) {
    log_internal(LogLevel::Debug, "main", msg);
}

pub fn error(msg: &str) {
    log_internal(LogLevel::Error, "main", msg);
}

pub fn warning(msg: &str) {
    log_internal(LogLevel::Warning, "main", msg);
}

pub fn trace(msg: &str) {
    log_internal(LogLevel::Trace, "main", msg);
}