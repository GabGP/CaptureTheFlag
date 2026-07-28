// ============================================================================
// FILE STORE & LOG ROTATION
// ============================================================================

use chrono::Local;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// Constants for log management
const MAX_LOG_LINES_PER_FILE: usize = 2000;
const LOG_DIRECTORY: &str = "logs";

// ============================================================================
// LOGGER STATE MANAGEMENT
// ============================================================================

/// Thread-safe handle to the file logger state
#[derive(Clone)]
pub struct LoggerHandle {
    inner: Arc<Mutex<LoggerState>>,
}

/// Internal state tracking active files and line counts for rotation
struct LoggerState {
    client_file: Option<PathBuf>,
    server_file: Option<PathBuf>,
    client_lines: usize,
    server_lines: usize,
}

impl LoggerHandle {
    /// Initializes a new logger handle and ensures the log directory exists
    pub fn new() -> io::Result<Self> {
        fs::create_dir_all(LOG_DIRECTORY)?;
        Ok(Self {
            inner: Arc::new(Mutex::new(LoggerState {
                client_file: None,
                server_file: None,
                client_lines: 0,
                server_lines: 0,
            })),
        })
    }

    /// Appends a new line to the log file, handling rotation if limits are reached
    pub fn append(&self, side: &str, line: &str) -> io::Result<()> {
        let path = self.ensure_file(side)?;
        let mut state = self.inner.lock().unwrap();
        let lines_count = if side == "client" {
            &mut state.client_lines
        } else {
            &mut state.server_lines
        };

        // Trigger log rotation if we exceed the maximum allowed lines
        if *lines_count >= MAX_LOG_LINES_PER_FILE {
            drop(state); // Drop lock before rotating
            let new_path = self.rotate_file(side)?;
            let mut state = self.inner.lock().unwrap();
            let lines_count = if side == "client" {
                &mut state.client_lines
            } else {
                &mut state.server_lines
            };

            // Reset counter and assign new file path
            *lines_count = 0;
            if side == "client" {
                state.client_file = Some(new_path.clone());
            } else {
                state.server_file = Some(new_path.clone());
            }
            let path = new_path;
            drop(state);
            self.write_to_path(&path, line)?;
            return Ok(());
        }

        self.write_to_path(&path, line)?;
        *lines_count += 1;
        Ok(())
    }

    /// Ensures a target log file exists and returns its path
    fn ensure_file(&self, side: &str) -> io::Result<PathBuf> {
        let mut state = self.inner.lock().unwrap();
        let target = if side == "client" {
            &mut state.client_file
        } else {
            &mut state.server_file
        };

        if let Some(path) = target {
            return Ok(path.clone());
        }

        let timestamp = current_timestamp();
        let path = PathBuf::from(LOG_DIRECTORY).join(format!("{}-{}.log", side, timestamp));
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        drop(file);
        *target = Some(path.clone());
        Ok(path)
    }

    /// Creates a new log file with a fresh timestamp for rotation
    fn rotate_file(&self, side: &str) -> io::Result<PathBuf> {
        let timestamp = current_timestamp();
        let path = PathBuf::from(LOG_DIRECTORY).join(format!("{}-{}.log", side, timestamp));
        let _ = File::create(&path)?;
        Ok(path)
    }

    /// Helper to open and write directly to a file path
    fn write_to_path(&self, path: &PathBuf, line: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", line)?;
        file.flush()?;
        Ok(())
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Retrieves the current system time as a UNIX timestamp string
pub fn current_timestamp() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", now)
}

/// Retrieves and formats the current local time for human-readable logging
pub fn format_timestamp() -> String {
    Local::now().format("%d/%m/%Y %H:%M:%S").to_string()
}
