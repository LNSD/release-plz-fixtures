/// Greets the given name in English.
pub fn greet_en(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Greets the given name in Dutch.
pub fn greet_nl(name: &str) -> String {
    format!("Hallo, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_en() {
        //* Given
        let name = "Alice";

        //* When
        let result = greet_en(name);

        //* Then
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn test_greet_nl() {
        //* Given
        let name = "Alice";

        //* When
        let result = greet_nl(name);

        //* Then
        assert_eq!(result, "Hallo, Alice!");
    }
}
