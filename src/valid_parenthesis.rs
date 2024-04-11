struct ValidParenthesis<'a> {
    ref_string: &'a str,
}

impl<'a> ValidParenthesis<'a> {
    fn new(ref_string: &'a str) -> Self {
        Self { ref_string }
    }

    fn matching_closing_parenthesis(ref_char: char) -> char {
        if ref_char == '(' {
            return ')';
        }

        if ref_char == '[' {
            return ']';
        }

        '}'
    }

    /// No new allocations. Simply indexing a string slice and checking one char against another.
    /// In order to avoid checking the same element twice, we can simply advance the iterator, such
    /// that when we first retrieve a value, it is consumed. The comparison value then advances the
    /// iterator to the next index. The loop will break once ref_chars.next() is None, therefore
    /// there will not be any out of bounds access. This will also not panic as there is a check
    /// for a failure to advance the iterator. If the iterator advances and there is No value, this
    /// means an odd number of characters exist and output should then return false.
    pub fn solve(&mut self) -> bool {
        let mut ref_chars = self.ref_string.chars();

        while let Some(char) = ref_chars.next() {
            let matching_char = Self::matching_closing_parenthesis(char);
            match ref_chars.next() {
                Some(char) => {
                    if matching_char != char {
                        return false;
                    }
                }
                // This implies a non-even indexed iterator
                None => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::ValidParenthesis;

    #[test]
    fn test_base_case() {
        let mut paren_test = ValidParenthesis::new("()");
        assert!(paren_test.solve())
    }

    #[test]
    fn test_all_brackets() {
        let mut paren_test = ValidParenthesis::new("()[]{}");
        assert!(paren_test.solve())
    }

    #[test]
    fn test_false_case() {
        let mut paren_test = ValidParenthesis::new("(]");
        assert!(!paren_test.solve())
    }

    #[test]
    fn test_odd_chars() {
        let mut paren_test = ValidParenthesis::new("()[");
        assert!(!paren_test.solve())
    }
}
