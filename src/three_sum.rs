use std::collections::HashSet;

struct ThreeSum {
    internal_vec: Vec<i32>,
}

impl ThreeSum {
    fn new(internal_vec: Vec<i32>) -> Self {
        Self { internal_vec }
    }

    /// Note to self: This is the definition of a logically sound solution...that is likely horrible. While the
    /// solution does attempt to make an optimization by removing values out of respective
    /// iterations, and also skips some work by ensuring that tuples inserted into the Hashset are
    /// always ordered and don't need checks for 3! times, there is still a lot of work to do.
    /// The solution iterates for O(n) followed by O(n-1) followed by O(n-2) times. Given the
    /// constrains of the question, this is at best O(n!) and at worst ~ O(n^3), making it useless
    /// for larger arrays.
    fn solve(&self) -> Vec<Vec<i32>> {
        // We use a hashset to ensure that there are no duplicates. For example, the first test,
        // could allow for two instances of [-1,0,1]
        let mut return_vec: HashSet<(i32, i32, i32)> = HashSet::new();
        let binding = self.internal_vec.iter().enumerate().collect::<Vec<_>>();
        let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

        for (index1, value1) in binding.iter() {
            let mut second_vec = binding.clone();
            second_vec.remove(*index1);
            for (index2, value2) in second_vec.iter() {
                let mut third_vec = second_vec.clone();
                if *index2 > 0 {
                    third_vec.remove(*index2 - 1);
                } else {
                    third_vec.remove(*index2);
                }
                for (index3, value3) in third_vec.iter() {
                    let mut indexes = [index1, index2, index3];
                    indexes.sort();
                    if **value1 + **value2 + **value3 == 0
                        && index1 != index2
                        && index2 != index3
                        && index1 != index3
                        && visited
                            .get(&(*indexes[0], *indexes[1], *indexes[2]))
                            .is_none()
                    {
                        visited.insert((*indexes[0], *indexes[1], *indexes[2]));
                        let mut values = [**value1, **value2, **value3];
                        values.sort();
                        return_vec.insert((values[0], values[1], values[2]));
                    }
                }
            }
        }

        return_vec
            .into_iter()
            .map(|(v1, v2, v3)| vec![v1, v2, v3])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ThreeSum;

    #[test]
    fn test_three_sum_base_case() {
        let three_sum = ThreeSum::new([-1, 0, 1, 2, -1, -4].into());
        assert_eq!(three_sum.solve(), vec![vec![-1, 0, 1], vec![-1, -1, 2]]);
    }

    #[test]
    fn test_three_sum_none_case() {
        let three_sum = ThreeSum::new([0, 1, 1].into());
        assert_eq!(three_sum.solve(), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_zeros() {
        let three_sum = ThreeSum::new([0, 0, 0].into());
        assert_eq!(three_sum.solve(), vec![vec![0, 0, 0]]);
    }
}
