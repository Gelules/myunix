pub fn array_max_min(_v: Vec<i32>) -> (i32, i32) {
    (std::i32::MIN, std::i32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let v = vec![42];
        let result = array_max_min(v);
        let expected = (42, 42);
        assert_eq!(result, expected);
    }

    #[test]
    fn simple() {
        let v = vec![42, 0, 666, 777, -51, 12, 13];
        let result = array_max_min(v);
        let expected = (42, 42);
        assert_eq!(result, expected);
    }

    #[test]
    fn extrema() {
        let v = vec![-42, 0, 666, 777, -51, 12, 13, 999];
        let result = array_max_min(v);
        let expected = (-42, 999);
        assert_eq!(result, expected);
    }

}
