use std::collections::HashSet;

struct TwoSum {
    nums: Vec<i32>,
    target: i32,
}

impl TwoSum {
    fn new(nums: Vec<i32>, target: i32) -> Self {
        Self { nums, target }
    }

    pub fn solution(&self) -> Option<Vec<i32>> {
        if let Some(soln) = Self::return_addition_improved(&self.nums, self.target) {
            return Some(soln);
        }

        None
    }

    fn return_addition(numbers: &[i32], target: i32) -> Option<Vec<i32>> {
        for (index, number) in numbers.iter().enumerate() {
            let new_vec = &numbers[(index + 1)..];

            if let Some(value) = new_vec.iter().find(|value| *value + number == target) {
                return Some(vec![*number, *value]);
            }
        }

        None
    }

    fn return_addition_improved(numbers: &[i32], target: i32) -> Option<Vec<i32>> {
        let mut hash_set: HashSet<&i32> = HashSet::new();
        for number in numbers.iter() {
            if let Some(val) = hash_set.get(&(target - number)) {
                return Some(vec![*number, **val]);
            } else {
                hash_set.insert(number);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::TwoSum;

    #[test]
    fn test_initial_state() {
        let two_sum = TwoSum::new(vec![2, 7, 11, 15], 9);

        assert_eq!(two_sum.solution(), Some(vec![7, 2]));
    }

    #[test]
    fn test_324() {
        let two_sum = TwoSum::new(vec![3, 2, 4], 6);

        assert_eq!(two_sum.solution(), Some(vec![4, 2]));
    }
}
