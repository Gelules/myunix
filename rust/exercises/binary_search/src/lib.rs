//! Of course the best way to search for an item is to use vec.get() and match the result. This is
//! just an exercise.

use std::cmp::Ordering;

pub fn binary_search(vec: Vec<i32>, elt: i32) -> i32 {
    let mut left = 0;
    let mut right = vec.len();
    let mut middle;
    let mut picked;

    while left < right {
        middle = left + (right - left) / 2;
        picked = vec.get(middle);

        match picked.cmp(&Some(&elt)) {
            Ordering::Equal => return middle.try_into().unwrap(),
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle,
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent() {
        let v = vec![-19, -17, -13, -11, -7, -5, -3, -2, 2, 3, 5, 7, 11, 13, 17, 19];
        let expected = -1;
        let result = binary_search(v, 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn first() {
        let v = vec![-19, -17, -13, -11, -7, -5, -3, -2, 2, 3, 5, 7, 11, 13, 17, 19];
        let expected = 0;
        let result = binary_search(v, -19);
        assert_eq!(result, expected);
    }

    #[test]
    fn middle() {
        let v = vec![-19, -17, -13, -11, -7, -5, -3, -2, 0, 2, 3, 5, 7, 11, 13, 17, 19];
        let expected = 8;
        let result = binary_search(v, 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn last() {
        let v = vec![-19, -17, -13, -11, -7, -5, -3, -2, 2, 3, 5, 7, 11, 13, 17, 19];
        let expected = 15;
        let result = binary_search(v, 19);
        assert_eq!(result, expected);
    }

    #[test]
    fn empty() {
        let v = Vec::new(); 
        let expected = -1;
        let result = binary_search(v, 19);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_element_absent() {
        let v = vec![-19];
        let expected = -1;
        let result = binary_search(v, 19);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_element_present() {
        let v = vec![-19];
        let expected = 0;
        let result = binary_search(v, -19);
        assert_eq!(result, expected);
    }
}
