struct ReverseInteger {
    num: i32,
}

impl ReverseInteger {
    fn new(num: i32) -> Self {
        Self { num }
    }

    /// This should be leetcode easy, but there's likely a bit level optimization.
    /// Change to chars then iterate through chars in reversed order, checking the first
    /// position to ensure that there are no 0's. As you iterate through chars, push to string.
    /// Then parse the string into an i32, while ensuring to check index 0 for '-' char and append.
    /// You can also parse each individual char to digit via a 10 radix, but this seems more
    /// complex.
    fn solve(&self) -> i32 {
        let num_string = self.num.to_string();
        let mut final_string = String::new();

        for (index, char) in num_string.chars().rev().enumerate() {
            if index == 0 && ('0' == char || char == '-') {
                continue;
            } else {
                final_string.push(char);
            }
        }

        let mut final_num = final_string.parse::<i32>().unwrap();

        if num_string.starts_with('-') {
            final_num = -final_num;
        }

        final_num
    }
}

#[cfg(test)]
mod tests {
    use super::ReverseInteger;

    #[test]
    fn test_reverse_integer_standard() {
        let reverse_int = ReverseInteger::new(123);
        assert_eq!(reverse_int.solve(), 321);
    }

    #[test]
    fn test_reverse_integer_negative() {
        let reverse_int = ReverseInteger::new(-123);
        assert_eq!(reverse_int.solve(), -321);
    }

    #[test]
    fn test_reverse_integer_zero_start() {
        let reverse_int = ReverseInteger::new(120);
        assert_eq!(reverse_int.solve(), 21);
    }
}
