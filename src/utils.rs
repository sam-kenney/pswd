//! Utility methods for the cli.

pub trait Chars {
    fn as_chars(&self) -> Vec<char>;
}

impl Chars for &str {
    fn as_chars(&self) -> Vec<char> {
        self.chars().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Chars;

    /// Test to_vec_char accurately splits string slices.
    #[test]
    fn test_to_vec_char() {
        let result = vec!['h', 'e', 'l', 'l', 'o'];
        assert_eq!("hello".as_chars(), result)
    }

    /// Test that trailing spaces are not cleaned up.
    #[test]
    fn test_to_vec_char_trailing_spaces() {
        let result = vec!['h', 'e', 'l', 'l', 'o', ' '];
        assert_eq!("hello ".as_chars(), result)
    }

    /// Test that leading spaces are not cleaned up.
    #[test]
    fn test_to_vec_char_leading_spaces() {
        let result = vec![' ', 'h', 'e', 'l', 'l', 'o'];
        assert_eq!(" hello".as_chars(), result)
    }
}
