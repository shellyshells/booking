use chrono::Local;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::fs::OpenOptions;
use std::io::Write;

/// Singleton Logger - Thread-safe global logging instance
pub struct Logger {
    log_file: Mutex<Option<std::fs::File>>,
}

static LOGGER: Lazy<Logger> = Lazy::new(|| {
    Logger {
        log_file: Mutex::new(None),
    }
});

impl Logger {
    /// Get the global logger instance
    pub fn instance() -> &'static Logger {
        &LOGGER
    }

    /// Initialize the logger with a file path
    pub fn init(&self, file_path: &str) {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Failed to open log file");
        
        *self.log_file.lock() = Some(file);
        self.log("INFO", "Logger initialized");
    }

    /// Log a message with a specific level
    pub fn log(&self, level: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_message = format!("[{}] [{}] {}\n", timestamp, level, message);

        // Print to console
        print!("{}", log_message);

        // Write to file if initialized
        if let Some(file) = self.log_file.lock().as_mut() {
            let _ = file.write_all(log_message.as_bytes());
            let _ = file.flush();
        }
    }

    pub fn info(&self, message: &str) {
        self.log("INFO", message);
    }

    pub fn warn(&self, message: &str) {
        self.log("WARN", message);
    }

    pub fn error(&self, message: &str) {
        self.log("ERROR", message);
    }

    pub fn debug(&self, message: &str) {
        self.log("DEBUG", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_logger() {
        let logger1 = Logger::instance();
        let logger2 = Logger::instance();
        
        // Verify same instance
        assert!(std::ptr::eq(logger1, logger2));
    }
}
