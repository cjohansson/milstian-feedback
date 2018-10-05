//! # Milstian Feedback
//! Used to handle feedback of various types of messages.

extern crate chrono;
use chrono::offset::Utc;

pub struct Feedback
{
    _error_file: Option<String>,
    _info_file: Option<String>,
}

impl Feedback {
    /// # Create new feedback object
    /// ```rust
    /// use milstian_feedback::Feedback;
    /// let feedback = Feedback::new(Option::None, Option::None);
    /// ```
    // TODO Verify error-file path and info_file path if specified
    pub fn new(_error_file: Option<String>, _info_file: Option<String>) -> Feedback {
        Feedback {
            _error_file,
            _info_file,
        }
    }
    /// # Output information
    /// ```rust,dont_run
    /// use milstian_feedback::Feedback;
    /// let feedback = Feedback::new(Option::None, Option::None);
    /// feedback.info("We have information".to_string());
    /// ```
    pub fn info(&self, message: String) {
        println!("{}", self.format_message(message));
        // TODO Output to info file here if specified
    }

    /// # Output error
    /// ```rust,dont_run
    /// use milstian_feedback::Feedback;
    /// let feedback = Feedback::new(Option::None, Option::None);
    /// feedback.error("We have a undefined error".to_string());
    /// ```
    pub fn error(&self, message: String) {
        eprintln!("{}", self.format_message(message));
    }

    /// # Format message with timestamp
    /// ```rust
    /// use milstian_feedback::Feedback;
    /// let feedback = Feedback::new(Option::None, Option::None);
    /// let message = feedback.format_message("Important stuff".to_string());
    /// assert!(message.contains("stuff"));
    /// ```
    pub fn format_message(&self, message: String) -> String {
        format!("{} - {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_message() {
        let feedback = Feedback::new(Option::None, Option::None);
        assert!(feedback.format_message("random".to_string()).contains("random"));
    }
}
