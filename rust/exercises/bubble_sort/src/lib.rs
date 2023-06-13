pub fn bubble_sort(vec: &mut Vec<i32>) {
    if vec.len() < 2 {
        return;
    }

    for i in 0..(vec.len() - 1){
        for j in 0..(vec.len() - 1 - i) {
            if vec[j] > vec[j + 1] {
                vec[j] ^= vec[j + 1];
                vec[j + 1] ^= vec[j];
                vec[j] ^= vec[j + 1];
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn vec_is_sorted(vec: Vec<i32>) -> bool {
        for i in 1..vec.len() {
            if vec[i - 1] > vec[i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn simple() {
        let mut vec = vec![42, 11, 73, 66, 0, -8, 100, 99];
        bubble_sort(&mut vec);
        assert!(vec_is_sorted(vec));
    }

    #[test]
    fn multiple() {
        let mut vec = vec![42, 42, 42, 11, 11, 73, 73, 73, 66, 66, 66, 66, 66, 0, 0, -8, -8, 100, 100, 99, 99, 99, 99, 99, 99];
        bubble_sort(&mut vec);
        assert!(vec_is_sorted(vec));
    }

    #[test]
    fn reversed() {
        let mut vec = vec![100, 99, 73, 66, 42, 11, 0, -8];
        bubble_sort(&mut vec);
        assert!(vec_is_sorted(vec));
    }

    #[test]
    fn one() {
        let mut vec = vec![42];
        bubble_sort(&mut vec);
        assert!(vec_is_sorted(vec));
    }

    #[test]
    fn two_sorted() {
        let mut vec = vec![11, 42];
        bubble_sort(&mut vec);
        assert!(vec_is_sorted(vec));
    }

    #[test]
    fn two() {
        let mut vec = vec![42, 11];
        bubble_sort(&mut vec);
        assert!(vec_is_sorted(vec));
    }

    #[test]
    fn empty() {
        let mut vec = Vec::new();
        bubble_sort(&mut vec);
        assert!(vec_is_sorted(vec));
    }
}
