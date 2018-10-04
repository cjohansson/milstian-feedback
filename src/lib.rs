extern crate chrono;
use chrono::offset::Utc;

pub struct Logger;

impl Logger {
    pub fn info(message: String) {
        println!("{}", Logger::format_message(message));
    }

    pub fn error(message: String) {
        eprintln!("{}", Logger::format_message(message));
    }

    fn format_message(message: String) -> String {
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
