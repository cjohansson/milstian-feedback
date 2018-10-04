extern crate chrono;
use chrono::offset::Utc;

//! # Milstian Feedback
//! Used to handle logging of various types of messages.

pub struct Logger;

impl Logger {
    /// # Output information
    /// ```rust,dont_run
    /// Logger::info("We have information".to_string());
    /// ```
    pub fn info(message: String) {
        println!("{}", Logger::format_message(message));
    }

    /// # Output error
    /// ```rust,dont_run
    /// Logger::error("We have a undefined error".to_string());
    /// ```
    pub fn error(message: String) {
        eprintln!("{}", Logger::format_message(message));
    }

    /// # Format message with timestamp
    /// ```rust
    /// let message = Logger::format("Important stuff".to_string());
    /// assert!(message.contains("stuff"));
    /// ```
    pub fn format_message(message: String) -> String {
        format!("{} - {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_message() {
        assert!(Logger::format_message("random".to_string()).contains("random"));
    }
}
