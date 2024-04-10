
/// [TODO:INCOMPLETE]
///
/// * `array`: Values to sort
struct MedianSortedArrays {
    array: Vec<u32>,
}

impl MedianSortedArrays {
    fn new(first_array: Vec<u32>, second_array: Vec<u32>) -> Self {
        let mut array = first_array;
        array.extend(second_array);
        Self { array }
    }

    fn first_solution(&mut self) -> f32 {
        // This is actually a fail as the problem is in sorting the array.
        // TODO: Come back later as this is ~ leetcode max_diff
        self.array.sort();

        if self.array.len() % 2 == 0 {
            (self.array[self.array.len() / 2] as f32 + self.array[self.array.len() / 2 - 1] as f32)
                / 2.0
        } else {
            self.array[self.array.len() / 2] as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MedianSortedArrays;

    #[test]
    fn mediansortedarrays_odd() {
        let mut median_array = MedianSortedArrays::new([1, 3].into(), [2].into());

        assert_eq!(median_array.first_solution(), 2.0)
    }

    #[test]
    fn mediansortedarrays_even() {
        let mut median_array = MedianSortedArrays::new([1, 2].into(), [3, 4].into());

        assert_eq!(median_array.first_solution(), 2.5)
    }
}
