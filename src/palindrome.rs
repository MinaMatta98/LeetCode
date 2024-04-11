struct Palindrome {
    num: i32,
}

impl Palindrome {
    fn new(num: i32) -> Self {
        Self { num }
    }

    // LeetCode joke check two iterators and consume them at both times. Time complexity O(n),
    // space complexity ~ O(2n)
    fn solve(&self) -> bool {
        let chars = self.num.to_string();
        let chars = chars.chars();
        let mut rev_chars = chars.clone().rev();

        for char in chars {
            if char != rev_chars.next().unwrap() {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Palindrome;

    #[test]
    fn test_palindrom_basecase() {
        let palindrome = Palindrome::new(121);
        assert!(palindrome.solve())
    }

    #[test]
    fn test_palindrom_negative() {
        let palindrome = Palindrome::new(-121);
        assert!(!palindrome.solve())
    }

    #[test]
    fn test_palindrom_false() {
        let palindrome = Palindrome::new(10);
        assert!(!palindrome.solve())
    }
}
