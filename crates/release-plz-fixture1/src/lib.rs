pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Greets the given name in English.
pub fn greet_en(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
