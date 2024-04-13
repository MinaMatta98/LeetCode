struct FindPosition {
    sorted_array: Vec<(usize, i64)>,
    target: i64,
}

impl FindPosition {
    fn new(sorted_array: Vec<i64>, target: i64) -> Self {
        // Ensure that vector does not duplicate by calling a consuming iterator.
        let sorted_array = sorted_array.into_iter().enumerate().collect();
        Self {
            sorted_array,
            target,
        }
    }

    // Will yield either two indicies if elements are present, or -1,-1 if not present
    fn solve(&self) -> Result<(usize, usize), (i8, i8)> {
        if let Ok(index) = Self::chunk_array_scope(&self.sorted_array, self.target) {
            // Else this means that index == last and there is no second element
            #[allow(clippy::collapsible_if)]
            if (index + 1) < self.sorted_array.len() {
                // We can check index value with guaranteed indexing safety
                if self.sorted_array[index + 1].1 == self.target {
                    return Ok((index, index + 1));
                }
            }
        }
        // Not ok means immediate return
        Err((-1, -1))
    }

    // Question requires a solution with O(log(n)) time complexity to search for a given value
    // within an array. This solution delivers a O(log2(n)) time complexity as required.
    //
    // Basically check the start of an array by consistently chunking arrays and testing the last
    // value of the array left of center or minimum size of the array value left of center.
    // Eventually there will be a vector of size [1] to check against. This is the base case that
    // ends the recursion. If the value within this vector does not yield the target, then return
    // -1.
    fn chunk_array_scope(sorted_array: &[(usize, i64)], target: i64) -> Result<usize, i64> {
        // Check for empty value
        if sorted_array.is_empty() {
            return Err(-1);
        }

        // Eventually, the array will be chunked to a single size array.
        if sorted_array[0].1 == target {
            return Ok(sorted_array[0].0);
        } else if sorted_array.len() == 1 {
            return Err(-1);
        }

        let binary_split_length = sorted_array.len() / 2;
        let lhs_array = &sorted_array[..binary_split_length];
        let rhs_array = &sorted_array[binary_split_length..];

        // Check the last element of left hand side array and subject to whether this is larger,
        // either chunk the lhs_array or rhs_array further.
        if lhs_array.last().unwrap().1 > target {
            Self::chunk_array_scope(lhs_array, target)
        } else {
            Self::chunk_array_scope(rhs_array, target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FindPosition;

    #[test]
    fn find_first_and_last_test() {
        let find_position = FindPosition::new(vec![5, 7, 7, 8, 8, 10], 8);
        assert_eq!(find_position.solve(), Ok((3, 4)))
    }

    #[test]
    fn find_first_and_last_none_case() {
        let find_position = FindPosition::new(vec![5, 7, 7, 8, 8, 10], 6);
        assert_eq!(find_position.solve(), Err((-1, -1)))
    }

    #[test]
    fn find_first_and_last_odd_none() {
        let find_position = FindPosition::new(vec![5, 7, 8, 8, 10], 6);
        assert_eq!(find_position.solve(), Err((-1, -1)))
    }

    #[test]
    fn find_first_and_last_even_none() {
        let find_position = FindPosition::new(vec![5, 7, 8, 8, 10], 8);
        assert_eq!(find_position.solve(), Ok((2, 3)))
    }

    #[test]
    fn find_first_and_last_single() {
        let find_position = FindPosition::new(vec![5, 7, 8, 8, 10], 10);
        assert_eq!(find_position.solve(), Err((-1, -1)))
    }

    #[test]
    fn find_first_and_last_empty() {
        let find_position = FindPosition::new(vec![], 0);
        assert_eq!(find_position.solve(), Err((-1, -1)))
    }
}
