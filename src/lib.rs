//! # Milstian Feedback
//! Used to handle feedback of various types of messages.

extern crate chrono;
use chrono::offset::Utc;

pub struct Feedback;

impl Feedback {
    /// # Output information
    /// ```rust,dont_run
    /// use milstian_feedback::Feedback;
    /// Feedback::info("We have information".to_string());
    /// ```
    pub fn info(message: String) {
        println!("{}", Feedback::format_message(message));
    }

    /// # Output error
    /// ```rust,dont_run
    /// use milstian_feedback::Feedback;
    /// Feedback::error("We have a undefined error".to_string());
    /// ```
    pub fn error(message: String) {
        eprintln!("{}", Feedback::format_message(message));
    }

    /// # Format message with timestamp
    /// ```rust
    /// use milstian_feedback::Feedback;
    /// let message = Feedback::format_message("Important stuff".to_string());
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
        assert!(Feedback::format_message("random".to_string()).contains("random"));
    }
}
