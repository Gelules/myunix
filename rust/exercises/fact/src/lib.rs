pub fn fact(n: u32) -> u32 {
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let result = fact(0);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn one() {
        let result = fact(1);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn two() {
        let result = fact(2);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn three() {
        let result = fact(3);
        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    fn seven() {
        let result = fact(7);
        let expected = 5040;
        assert_eq!(result, expected);
    }

    #[test]
    fn twelve() {
        let result = fact(12);
        let expected = 479007600;
        assert_eq!(result, expected);
    }
}
