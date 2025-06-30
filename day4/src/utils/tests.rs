fn private_helper() -> u32 {
    42
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| "Invalid number".to_string())
}

#[cfg(test)]
mod tests {
    use super::super::calculate::{add, divide};

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_private_helper() {
        assert_eq!(super::private_helper(), 42);
    }

    #[test]
    #[should_panic(expected = "Divide by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }

    #[test]
    fn test_parse() -> Result<(), String> {
        // return Result
        let n = super::parse_number("42")?;
        assert_eq!(n, 42);
        Ok(())
    }
}
