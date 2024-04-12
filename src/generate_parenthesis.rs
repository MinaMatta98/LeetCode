use std::collections::HashSet;

struct GenerateParenthesis {
    num: u8,
    parens: HashSet<String>,
}

impl GenerateParenthesis {
    fn new(num: u8) -> Self {
        Self {
            num,
            parens: HashSet::new(),
        }
    }

    // Logic: First pass => i == 1 => always yeild ()
    // Logic: All other passes do this: either add () to the start or end of last pass values or
    // add '(' last_value ')'. Trying this for n == 2 will demonstrate that we get n == 1 ["()"]
    // therefore n == 2 == [(()), ()(), ()()]. The duplication problem is solved by using a HashSet
    // where string implements hash and only unique values are stored. This is a more efficient way
    // of dealing with duplicates rather than consistently adding parenthesis onto duplicates which
    // only grow the operations with time. Even if filteration is done at every step, this still
    // involves and additional n! number of passes.
    fn solve(mut self) -> Vec<String> {
        for i in 0..self.num {
            if i == 0 {
                let value = "()".into();
                self.parens.insert(value);
            } else {
                let mut string_hashset: HashSet<String> = HashSet::new();
                // As this is a consuming iterator, it does not lead to a growth in size as one
                // value is taken out of the hashset and placed into the new hashset. Therefore
                // space complexity is not increased by new insertions
                for ref_string in self.parens.into_iter() {
                    string_hashset.insert(format!("{ref_string}()"));
                    string_hashset.insert(format!("(){ref_string}"));
                    string_hashset.insert(format!("({ref_string})"));
                }

                self.parens = string_hashset;
            }
        }

        self.parens.into_iter().collect()
    }

    // Recursive Version
    fn solve_recursively(mut self, value: u8) -> Vec<String> {
        if self.num == value {
            let value = "()".into();
            self.parens.insert(value);
        } else {
            let mut string_hashset: HashSet<String> = HashSet::new();
            // As this is a consuming iterator, it does not lead to a growth in size as one
            // value is taken out of the hashset and placed into the new hashset.
            for ref_string in self.parens.into_iter() {
                string_hashset.insert(format!("{ref_string}()"));
                string_hashset.insert(format!("(){ref_string}"));
                string_hashset.insert(format!("({ref_string})"));
            }

            self.parens = string_hashset;
        }

        if value == 1 {
            return self.parens.into_iter().collect();
        }

        Self::solve_recursively(self, value - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::GenerateParenthesis;

    #[test]
    fn test_parens() {
        let gen_parens = GenerateParenthesis::new(3);

        let solution = gen_parens.solve_recursively(3);

        // Since there is no suggestion in leetCode that order matters
        let expected_solutions = ["((()))", "(()())", "(())()", "()(())", "()()()"];

        for ref_string in expected_solutions {
            assert!(solution.contains(&ref_string.to_owned()))
        }

        // Ensure no less or more (duplicates)
        assert!(solution.len() == expected_solutions.len())
    }
    #[test]
    fn test_parens_double() {
        let gen_parens = GenerateParenthesis::new(2);

        let solution = gen_parens.solve();

        println!("{solution:?}");

        // Since there is no suggestion in leetCode that order matters
        let expected_solutions = ["(())", "()()"];

        for ref_string in expected_solutions {
            assert!(solution.contains(&ref_string.to_owned()))
        }

        // Ensure no less or more (duplicates)
        assert!(solution.len() == expected_solutions.len())
    }

    #[test]
    fn test_parens_single() {
        let gen_parens = GenerateParenthesis::new(1);

        let solution = gen_parens.solve();

        // Since there is no suggestion in leetCode that order matters
        let expected_solutions = ["()"];

        for ref_string in expected_solutions {
            assert!(solution.contains(&ref_string.to_owned()))
        }

        // Ensure no less or more (duplicates)
        assert!(solution.len() == expected_solutions.len())
    }
}
