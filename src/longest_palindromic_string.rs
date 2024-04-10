use std::collections::HashSet;

#[derive(Debug)]
struct LongestPalindromicString<'a> {
    string: &'a str,
}

impl<'a> LongestPalindromicString<'a> {
    fn new(string: &'a str) -> Self {
        Self { string }
    }

    fn solve(&self) -> String {
        // Not using a hashmap as the key would continuously get reused. Instead, in Hashset with
        // (usize, usize), both values in (usize, usize) impl Hash, Therefore no collision.
        //
        let mut checked_set: HashSet<(usize, usize)> = HashSet::new();
        let mut string_collection: Vec<String> = Vec::new();
        let char_array = self.string.chars().enumerate().collect::<Vec<_>>();

        for (index, chars) in char_array.iter() {
            let mut checking_array = char_array.clone();
            checking_array.remove(*index);
            let chars_index = checking_array
                .iter()
                .filter_map(|(checking_index, checked_char)| {
                    // Skip already checked values => O(n^2) -> O(n)
                    (chars == checked_char && checked_set.get(&(*index, *checking_index)).is_none())
                        .then_some(*checking_index)
                })
                .collect::<Vec<_>>();

            for next_index in chars_index {
                // if checked_set.get(&(*index, next_index)).is_some() {
                //     continue;
                // }

                if next_index >= *index {
                    let cmp_chars = &char_array[*index..=next_index]
                        .iter()
                        .map(|(_, chars)| *chars)
                        .collect::<Vec<char>>();

                    let mut cmp_chars_rev = cmp_chars.clone();
                    cmp_chars_rev.reverse();

                    if cmp_chars_rev == *cmp_chars {
                        string_collection.push(cmp_chars.iter().collect::<String>())
                    }
                }
                checked_set.insert((*index, next_index));
                checked_set.insert((next_index, *index));
            }
        }

        string_collection.sort_by_key(|a| a.len());
        string_collection.last().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::LongestPalindromicString;

    #[test]
    fn longest_palindromic_string() {
        let longest_palindromic_string = LongestPalindromicString::new("babad");
        let solution = longest_palindromic_string.solve();
        assert!(solution == "bab" || solution == "aba")
    }

    #[test]
    fn longest_palindromic_string_cbbd() {
        let longest_palindromic_string = LongestPalindromicString::new("cbbd");
        let solution = longest_palindromic_string.solve();
        assert!(solution == "bb")
    }
}
