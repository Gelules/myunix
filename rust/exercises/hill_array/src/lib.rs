pub fn top_of_the_hill(hill: &Vec<i32>) -> i32 {
    let mut max = 0;
    let mut max_index = 0;
    let mut previous = 0;
    let mut max_reached = false;

    for i in 0..hill.len() {
        if hill[i] < 0 || (max_reached && hill[i] > previous) {
            return -1;
        }

        if max_reached == false && hill[i] < previous {
            max_reached = true;
        }

        if hill[i] > max {
            max = hill[i];
            max_index = i;
        }

        previous = hill[i];
    }
    return max_index.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index_of_max(hill: &Vec<i32>) -> i32 {
        let mut max_index = 0;
        let mut max = hill[0];

        for (index, &element) in hill.iter().enumerate() {
            if element > max {
                max = element;
                max_index = index;
            }
        }

        return max_index.try_into().unwrap();
    }

    #[test]
    fn simple() {
        let hill = vec![0, 1, 3, 5, 7, 5, 1, 0];
        let result = top_of_the_hill(&hill);
        let expected = index_of_max(&hill);
        assert_eq!(result, expected);
    }

    #[test]
    fn plateau() {
        let hill = vec![0, 1, 3, 5, 7, 7, 7, 7, 5, 1, 0];
        let result = top_of_the_hill(&hill);
        let expected = index_of_max(&hill);
        assert_eq!(result, expected);
    }

    #[test]
    fn wrong() {
        let hill = vec![0, 1, 3, 5, 7, 5, 7, 0];
        let result = top_of_the_hill(&hill);
        let expected = -1;
        assert_eq!(result, expected);
    }

    #[test]
    fn steep() {
        let hill = vec![0, 1, 3, 5, 7];
        let result = top_of_the_hill(&hill);
        let expected = index_of_max(&hill);
        assert_eq!(result, expected);
    }

    #[test]
    fn reverse_steep() {
        let hill = vec![0, 1, 3, 5, 7];
        let result = top_of_the_hill(&hill);
        let expected = index_of_max(&hill);
        assert_eq!(result, expected);
    }

    #[test]
    fn negative() {
        let hill = vec![0, 1, 3, 5, 7, 5, 1, 0, -2];
        let result = top_of_the_hill(&hill);
        let expected = -1;
        assert_eq!(result, expected);
    }

    #[test]
    fn one() {
        let hill = vec![42];
        let result = top_of_the_hill(&hill);
        let expected = index_of_max(&hill);
        assert_eq!(result, expected);
    }

    #[test]
    fn zeroes() {
        let hill = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = top_of_the_hill(&hill);
        let expected = index_of_max(&hill);
        assert_eq!(result, expected);
    }

}
