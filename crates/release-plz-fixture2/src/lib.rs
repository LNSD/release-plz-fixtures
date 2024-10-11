/// Greets the given name in Spanish.
pub fn greet_es(name: &str) -> String {
    format!("¡Hola, {}!", name)
}

/// Greets the given name in French.
pub fn greet_fr(name: &str) -> String {
    format!("Bonjour, {}!", name)
}

/// Greets the given name in Italian.
pub fn greet_it(name: &str) -> String {
    format!("Ciao, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_es() {
        //* Given
        let name = "Alice";

        //* When
        let result = greet_es(name);

        //* Then
        assert_eq!(result, "¡Hola, Alice!");
    }

    #[test]
    fn test_greet_fr() {
        //* Given
        let name = "Alice";

        //* When
        let result = greet_fr(name);

        //* Then
        assert_eq!(result, "Bonjour, Alice!");
    }

    #[test]
    fn test_greet_it() {
        //* Given
        let name = "Alice";

        //* When
        let result = greet_it(name);

        //* Then
        assert_eq!(result, "Ciao, Alice!");
    }
}
