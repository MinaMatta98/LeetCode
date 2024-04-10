struct LongestSubstring<'a> {
    string: &'a str,
}

impl<'a> LongestSubstring<'a> {
    fn new(string: &'a str) -> Self {
        Self { string }
    }

    fn solution(&self) -> String {
        let mut return_string: Vec<String> = vec![];

        for chars in self.string.chars() {
            if let Some(string) = return_string.last_mut() {
                if string.contains(chars) {
                    return_string.push(String::from(chars));
                } else {
                    string.push(chars);
                }
            } else {
                return_string.push(String::from(chars));
            }
        }

        return_string.sort_by_key(|a| a.len());
        return_string.last().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::LongestSubstring;

    #[test]
    fn test_longest_substring() {
        let longest_substring = LongestSubstring::new("bbbbbb");

        assert_eq!(longest_substring.solution(), "b");
    }
}
