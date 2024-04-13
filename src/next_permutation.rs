#[derive(Debug)]
struct NextPermutation {
    array: Vec<u8>,
}

impl NextPermutation {
    fn new(array: Vec<u8>) -> Self {
        Self { array }
    }

    // Logic: If you want to make the smallest change possible and therefore yield the next
    // lexicographically ordered permutation, you should start from the end of the vector where
    // changes will yeild the smallest value. Therefore, let's reverse the vector, skip the first
    // iteration as there is nothing to compare to, and iterate.
    //
    // We compare the value in the current index with all values after it. We filter all the
    // comparisons, such that we only yeild greater values. However, we must then swap the current
    // index, such that the least of the greater values is used. In order to yeild the next
    // permutation, all elements after the array index must then be sorted.
    //
    // There is one edge condition where if the index == 0 and there is no greater number than
    // index 0, that means we have reached the end of lexicographical sorting. Reverse the iterator
    // and start again.
    fn solve(self) -> Vec<u8> {
        let mut array = self.array;

        for (index, value) in array.iter().enumerate().rev().skip(1) {
            // index + 1 cannot fail due to iterator skip(1).
            let comparison_indecies_min = array[(index + 1)..]
                .iter()
                .enumerate()
                .filter(|(_, comparison_val)| *comparison_val > value)
                // Note that enumeration means that the new index will have to be
                // incremented by + index + 1 to return the true index of the returned
                // parts
                .min_by(|(_, val1), (_, val2)| val1.cmp(val2))
                .map(|(current_index, _)| current_index + index + 1);

            if let Some(new_index) = comparison_indecies_min {
                array.swap(new_index, index);
                // When elements are swapped, this means that a new next value is chosen,
                // therefore the next lexicographically greater value will require a sort
                // for all elements after the index that is swapped
                //
                // Since the elements are already at their max order beyond this point, we can
                // simply reverse them to get the minimum lexicographical permutation.
                //
                // In comparison to sort(), this will reduce time complexity from O(nlog(n)) => O(n)
                array[(index + 1)..].reverse();
                return array;
            }
            // Once this pathway is taken, it means that the iterator has come to the first index
            // and has deemed that there are no greater values. This means that there is no next,
            // therefore restart by sorting the vector and yielding the first lexicographically
            // ordered value.
            else if index == 0 {
                // Since the elements are already at their max order beyond this point, we can
                // simply reverse them to get the minimum lexicographical permutation.
                //
                // In comparison to sort(), this will reduce time complexity from O(nlog(n)) => O(n)
                array.reverse();
                return array;
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::NextPermutation;

    #[test]
    fn test_next_permuation() {
        let next_permutation = NextPermutation::new(vec![1, 2, 3]);
        assert_eq!(next_permutation.solve(), vec![1, 3, 2]);
    }

    #[test]
    fn test_non_trivial() {
        let next_permutation = NextPermutation::new(vec![1, 1, 5]);
        assert_eq!(next_permutation.solve(), vec![1, 5, 1]);
    }

    #[test]
    fn test_next_permuation_reverse() {
        let next_permutation = NextPermutation::new(vec![3, 2, 1]);
        assert_eq!(next_permutation.solve(), vec![1, 2, 3]);
    }

    #[test]
    fn test_more_than_three() {
        let next_permutation = NextPermutation::new(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(next_permutation.solve(), vec![1, 2, 3, 4, 6, 5]);
    }

    #[test]
    fn test_more_deep() {
        let next_permutation = NextPermutation::new(vec![1, 2, 3, 6, 5, 4]);
        assert_eq!(next_permutation.solve(), vec![1, 2, 4, 3, 5, 6]);
    }
}
