pub fn fibo(n: u32) -> u32 {
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let result = fibo(0);
        let expected = 0;
        assert_eq!(result, expected);
    }


    #[test]
    fn one() {
        let result = fibo(1);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn two() {
        let result = fibo(2);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn five() {
        let result = fibo(5);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn eight() {
        let result = fibo(8);
        let expected = 21;
        assert_eq!(result, expected);
    }

    #[test]
    fn ten() {
        let result = fibo(10);
        let expected = 55;
        assert_eq!(result, expected);
    }

    #[test]
    fn twenty() {
        let result = fibo(20);
        let expected = 6765;
        assert_eq!(result, expected);
    }

    #[test]
    fn thirty() {
        let result = fibo(30);
        let expected = 832040;
        assert_eq!(result, expected);
    }
}
