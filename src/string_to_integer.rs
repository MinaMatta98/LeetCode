struct Atoi<'a> {
    ref_string: &'a str,
}

impl<'a> Atoi<'a> {
    fn new(ref_string: &'a str) -> Self {
        Self { ref_string }
    }

    // This should be leetcode easy. Basically follow exactly what the test says.
    fn solve(&mut self) -> i32 {
        // Basically iterates through a byte string untill it reaches a val != ' '
        self.ref_string = self.ref_string.trim_start();
        let mut negative = false;
        let mut final_string = String::new();

        for (index, chars) in self.ref_string.chars().enumerate() {
            if index == 0 && chars == '-' {
                negative = true;
                continue;
            }

            if chars.is_ascii_digit() {
                final_string.push(chars)
            } else {
                break;
            }
        }

        let mut final_int = final_string.parse::<i32>().unwrap();

        if negative {
            final_int = -final_int;
        }

        final_int = final_int.clamp(i32::MIN, i32::MAX);

        final_int
    }
}

#[cfg(test)]
mod tests {
    use super::Atoi;

    #[test]
    fn test_atoi_base() {
        let mut atoi = Atoi::new("42");
        assert_eq!(atoi.solve(), 42);
    }

    #[test]
    fn test_atoi_negative() {
        let mut atoi = Atoi::new("-42");
        assert_eq!(atoi.solve(), -42);
    }

    #[test]
    fn test_atoi_words() {
        let mut atoi = Atoi::new("4193 with words");
        assert_eq!(atoi.solve(), 4193);
    }
}
