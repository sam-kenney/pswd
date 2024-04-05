//! Utility methods for the cli.

/// Convert a string slice into a Vec<char>.
pub fn to_vec_char(content: &str) -> Vec<char> {
    content.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::to_vec_char;

    /// Test to_vec_char accurately splits string slices.
    #[test]
    fn test_to_vec_char() {
        let result = vec!['h', 'e', 'l', 'l', 'o'];
        assert_eq!(to_vec_char("hello"), result)
    }

    /// Test that trailing spaces are not cleaned up.
    #[test]
    fn test_to_vec_char_trailing_spaces() {
        let result = vec!['h', 'e', 'l', 'l', 'o', ' '];
        assert_eq!(to_vec_char("hello "), result)
    }

    /// Test that leading spaces are not cleaned up.
    #[test]
    fn test_to_vec_char_leading_spaces() {
        let result = vec![' ', 'h', 'e', 'l', 'l', 'o'];
        assert_eq!(to_vec_char(" hello"), result)
    }
}
