/// Utility functions for the frontend application

/// Format error messages for display
pub fn format_error(error: &str) -> String {
    // Remove common prefixes from error messages
    error
        .replace("Request error: ", "")
        .replace("Parse error: ", "")
        .replace("Serialization error: ", "")
}

/// Truncate text to a specified length
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length])
    }
}

/// Format date string to display format
pub fn format_date(date_str: &str) -> String {
    // Extract just the date part (YYYY-MM-DD) from ISO 8601 format
    if date_str.len() >= 10 {
        date_str[..10].to_string()
    } else {
        date_str.to_string()
    }
}
