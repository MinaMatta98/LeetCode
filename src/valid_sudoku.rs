struct ValidSudoku {
    board: Vec<Vec<char>>,
}

impl ValidSudoku {
    fn new(board: Vec<Vec<char>>) -> Self {
        Self { board }
    }

    fn solve(&mut self) -> bool {
        if !self.solve_squares() {
            return false;
        }

        self.solve_row_column_pairs()
    }

    fn solve_squares(&self) -> bool {
        let board_square_num_x = self.board.len() / 3;
        let board_square_num_y = board_square_num_x;

        // Row iteration
        for i in 0..board_square_num_x {
            for j in 0..board_square_num_y {
                // Index into rows
                let rows = self.board.get((j * 3)..=(j * 3 + 2)).unwrap();

                let mut view = rows
                    .iter()
                    // Return only column indexes within the rows
                    .flat_map(|vec| &vec[i * 3..=(i * 3 + 2)])
                    .filter(|val| **val != '.')
                    .collect::<Vec<_>>();

                view.sort();
                let view_next = view.iter().skip(1);
                if view
                    .iter()
                    .zip(view_next)
                    .any(|(char1, char2)| char1 == char2)
                {
                    return false;
                }
            }
        }

        true
    }

    fn solve_row_column_pairs(&mut self) -> bool {
        if self.board[0].len() == 1 {
            return true;
        }

        // This will intersect for both rows and columns and must therefore be reserved
        let index_0 = self.board[0].remove(0);

        let mut row = self.board.remove(0);
        row.push(index_0);
        row.sort();
        row.retain(|chars| *chars != '.');

        let mut column = self
            .board
            .iter_mut()
            .filter_map(|items| {
                let char = items.remove(0);
                char.ne(&'.').then_some(char)
            })
            .collect::<Vec<_>>();

        column.push(index_0);
        column.sort();

        if !Self::row_column_unique_chec(&column) || !Self::row_column_unique_chec(&row) {
            return false;
        }

        self.solve_row_column_pairs()
    }

    /// This will iterate over all chars within the array and ensure that they unique. Note that
    /// all iteration requires cmp (index), (index + 1). Therefore, the char array **MUST** be sorted
    ///
    /// * `row_or_column`: Sorted char array
    fn row_column_unique_chec(row_or_column: &[char]) -> bool {
        let iter = row_or_column.iter();
        let iter_skip = row_or_column.iter().skip(1);
        iter.zip(iter_skip).all(|(val1, val2)| val1 != val2)
    }
}

#[cfg(test)]
mod tests {
    use super::ValidSudoku;

    #[test]
    fn test_valid_sudoku_base_case() {
        let board = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]
        .iter_mut()
        .map(|char_arr| char_arr.to_vec())
        .collect();

        let mut valid_sudoku = ValidSudoku::new(board);

        assert!(valid_sudoku.solve());
    }

    #[test]
    fn test_valid_sudoku_duplicates_in_squares() {
        let board = [
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]
        .iter_mut()
        .map(|char_arr| char_arr.to_vec())
        .collect();

        let mut valid_sudoku = ValidSudoku::new(board);

        assert!(!valid_sudoku.solve());
    }
}
