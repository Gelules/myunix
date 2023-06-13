pub fn digit(mut n: i32, mut k: i32) -> u32 {
    let mut result = 0;

    if n <= 0 || k <= 0 {
        return 0;
    }

    while k != 0 {
        result = n % 10;
        n /= 10;
        k -= 1;
    }

    return result.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left() {
        let n = 123456;
        let k = 1;
        let expected = 6;
        let result = digit(n, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn right() {
        let n = 123456;
        let k = 6;
        let expected = 1;
        let result = digit(n, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn middle() {
        let n = 123456;
        let k = 4;
        let expected = 3;
        let result = digit(n, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn n_zero() {
        let n = 0;
        let k = 6;
        let expected = 0;
        let result = digit(n, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn n_negative() {
        let n = -123456;
        let k = 6;
        let expected = 0;
        let result = digit(n, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn k_zero() {
        let n = 123456;
        let k = 0;
        let expected = 0;
        let result = digit(n, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn k_negative() {
        let n = 123456;
        let k = -6;
        let expected = 0;
        let result = digit(n, k);
        assert_eq!(result, expected);
    }
}
