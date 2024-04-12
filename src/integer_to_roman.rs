#[repr(u16)]
#[derive(Clone)]
enum RomanChar {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl ToString for RomanChar {
    fn to_string(&self) -> String {
        match self {
            RomanChar::I => "I",
            RomanChar::V => "V",
            RomanChar::X => "X",
            RomanChar::L => "L",
            RomanChar::C => "C",
            RomanChar::D => "D",
            RomanChar::M => "M",
        }
        .to_string()
    }
}

impl RomanChar {
    fn from_vec(input_vec: Vec<Self>) -> String {
        let mut return_string = String::new();

        for roman_char in input_vec {
            return_string.push_str(&roman_char.to_string())
        }

        return_string
    }

    fn get_reducing_roman(&self) -> Self {
        match self {
            RomanChar::I => RomanChar::I,
            RomanChar::V => RomanChar::I,
            RomanChar::X => RomanChar::I,
            RomanChar::L => RomanChar::X,
            RomanChar::C => RomanChar::X,
            RomanChar::D => RomanChar::C,
            RomanChar::M => RomanChar::C,
        }
    }
}

struct IntegerToRoman {
    num: u16,
}

impl IntegerToRoman {
    fn new(num: u16) -> Self {
        Self { num }
    }

    /// Logic: Iterate through the value and ensure that two conditions are met:
    /// You consistently chunk down the value until the number hits 0.
    /// You also ensure that after checking the result, you also check whether get_reducing_roman
    /// can help chunk down the number further (You must ensure that the remainder is less than the
    /// current value and the chunking value). You keep appending to the array.
    fn solve(&mut self) -> String {
        let mut roman_vec: Vec<RomanChar> = Vec::new();
        let roman_char = [
            RomanChar::M,
            RomanChar::D,
            RomanChar::C,
            RomanChar::L,
            RomanChar::X,
            RomanChar::V,
            RomanChar::I,
        ];

        while self.num > 0 {
            for next_char in roman_char.iter() {
                let char_val = next_char.clone() as u16;
                if self.num.ge(&char_val) {
                    self.num -= char_val;
                    roman_vec.push(next_char.clone());
                }
                if self.num.lt(&char_val)
                    && self
                        .num
                        .ge(&(char_val - next_char.get_reducing_roman() as u16))
                    && char_val != 1
                {
                    self.num -= char_val - next_char.get_reducing_roman() as u16;
                    roman_vec.push(next_char.get_reducing_roman());
                    roman_vec.push(next_char.clone());
                }
            }
        }

        RomanChar::from_vec(roman_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::IntegerToRoman;

    #[test]
    fn test_integer_to_roman_base() {
        let mut int_to_roman = IntegerToRoman::new(3);

        assert_eq!(int_to_roman.solve(), "III")
    }

    #[test]
    fn test_integer_to_roman_58() {
        let mut int_to_roman = IntegerToRoman::new(58);
        assert_eq!(int_to_roman.solve(), "LVIII")
    }

    #[test]
    fn test_integer_to_roman_1994() {
        let mut int_to_roman = IntegerToRoman::new(1994);
        assert_eq!(int_to_roman.solve(), "MCMXCIV")
    }
}
