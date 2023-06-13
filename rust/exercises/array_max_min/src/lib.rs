pub fn array_max_min(v: Vec<i32>) -> (i32, i32) {
    (*v.iter().min().unwrap(), *v.iter().max().unwrap())
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
        let expected = (-51, 777);
        assert_eq!(result, expected);
    }

    #[test]
    fn extrema() {
        let v = vec![-51, 0, 666, 777, -42, 12, 13, 999];
        let result = array_max_min(v);
        let expected = (-51, 999);
        assert_eq!(result, expected);
    }

}
