use std::collections::HashMap;

struct ContainerMostWater<'a> {
    array: &'a [u32],
}

impl<'a> ContainerMostWater<'a> {
    fn new(array: &'a [u32]) -> Self {
        Self { array }
    }

    // Logic:
    // Basically brute force method, short of filtering all previously compared indicies, but
    // placing value in hashmap. This brings down time complexity from O(n^2) => O(n).
    fn solve(&self) -> usize {
        let mut hash_set: HashMap<(usize, usize), usize> = HashMap::new();
        let checking_vec = self.array;

        for (index, height) in self.array.iter().enumerate() {
            // This is O(n)
            let new_checking_vec = checking_vec
                .iter()
                .enumerate()
                .filter(|(checking_index, _)| {
                    *checking_index != index
                        // This is O(1)
                        && hash_set.get(&(*checking_index, index)).is_none()
                        // This is O(1)
                        && hash_set.get(&(index, *checking_index)).is_none()
                })
                .collect::<Vec<_>>();

            // This is O(n)
            for (index2, height2) in new_checking_vec {
                let width = index2.abs_diff(index);
                let height = height.min(height2);
                hash_set.insert((index2, index), width * (*height as usize));
            }
        }

        // O(n)
        *hash_set.iter().max_by_key(|a| a.1).unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::ContainerMostWater;

    #[test]
    fn test_container_most_water_basecase() {
        let container = ContainerMostWater::new(&[1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(container.solve(), 49)
    }

    #[test]
    fn test_container_most_water_two_elements() {
        let container = ContainerMostWater::new(&[1, 1]);
        assert_eq!(container.solve(), 1)
    }
}
