struct ZigZagConversion<'a> {
    input_string: &'a str,
    row_count: u32,
}

impl<'a> ZigZagConversion<'a> {
    fn new(input_string: &'a str, row_count: u32) -> Self {
        Self {
            input_string,
            row_count,
        }
    }

    /// This is one of the harder questions. The whole logic is predicated in constructing a vector
    /// of vectors, where the internal vector count == row_count of the struct.
    /// The question is then when to select each vector. The rule to form an iterator over the
    /// vectors until you cannot iterate next anymore. Then you reallocate the iterator, but switch
    /// the direction. Given that you already poppulate the last iterator before switching, you
    /// must then move the iterative pointer, such that you skip the first vector and poppulate the
    /// next vec. This is done in O(n) duration, as you need to iterate through the elements. You
    /// also only allocate for n num chars and the space complexity is also O(n).
    fn solve(&self) -> String {
        let char_array = self.input_string.chars();
        let num_rows = self.row_count;
        let mut reversed = 0;

        let mut char_insertion_vec: Vec<Vec<char>> = Vec::new();

        for _ in 0..num_rows {
            char_insertion_vec.push(Vec::new())
        }

        let mut insertion_vec = char_insertion_vec.iter_mut();

        for char in char_array {
            if reversed % 2 == 0 {
                if let Some(insertion_vec) = insertion_vec.next() {
                    insertion_vec.push(char)
                } else {
                    insertion_vec = char_insertion_vec.iter_mut();
                    if let Some(insertion_vec) = insertion_vec.nth_back(1) {
                        insertion_vec.push(char);
                    }
                    reversed += 1;
                }
            } else if let Some(insertion_vec) = insertion_vec.next_back() {
                insertion_vec.push(char)
            } else {
                insertion_vec = char_insertion_vec.iter_mut();
                if let Some(insertion_vec) = insertion_vec.nth(1) {
                    insertion_vec.push(char);
                }
                reversed += 1;
            }
        }

        char_insertion_vec
            .iter()
            .flat_map(|chars| {
                println!("{chars:?}");
                chars.to_vec()
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::ZigZagConversion;

    #[test]
    fn zig_zag_conversion_test_3() {
        let zig_zag_construct = ZigZagConversion::new("PAYPALISHIRING", 3);

        assert_eq!(zig_zag_construct.solve(), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn zig_zag_conversion_test_4() {
        let zig_zag_construct = ZigZagConversion::new("PAYPALISHIRING", 4);

        assert_eq!(zig_zag_construct.solve(), "PINALSIGYAHRPI");
    }
}
