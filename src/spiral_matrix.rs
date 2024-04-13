#[derive(Default)]
enum Direction {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    // Solution is predicated on a consuming iterator. The consumed_vec is consumed through
    // iteration. Therefore, the consumed_vec[0] for the first iteration is not the same as the
    // consumed_vec[0] after a full round through right, down, left and up.
    fn consume(&mut self, consumed_vec: &mut Vec<Vec<u8>>, insertion_vec: &mut Vec<u8>) {
        match self {
            Direction::Right => self.right(consumed_vec, insertion_vec),
            Direction::Down => self.down(consumed_vec, insertion_vec),
            Direction::Left => self.left(consumed_vec, insertion_vec),
            Direction::Up => self.up(consumed_vec, insertion_vec),
        }
    }

    // Assume a spiral. Next direction is terminated by not initiating a call where the vector no
    // longer yields elements
    fn next_direction(&mut self) {
        match self {
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
            Direction::Up => *self = Direction::Right,
        }
    }

    // Go right untill you can no longer go right.
    fn right(&mut self, consumer_vec: &mut Vec<Vec<u8>>, insertion_vec: &mut Vec<u8>) {
        if consumer_vec.len().gt(&0) {
            let top_vec = consumer_vec.remove(0);
            if top_vec.len().gt(&0) {
                let top_vec_iter = top_vec.into_iter();
                for value in top_vec_iter {
                    insertion_vec.push(value)
                }
            }
            self.next_direction();
            self.consume(consumer_vec, insertion_vec);
        }
    }

    // Go down untill you can no longer go down.
    fn down(&mut self, consumer_vec: &mut Vec<Vec<u8>>, insertion_vec: &mut Vec<u8>) {
        if consumer_vec.len().gt(&0) {
            let right_vec = consumer_vec
                .iter_mut()
                .map(|vec| vec.pop())
                .collect::<Vec<_>>();

            if right_vec.len().gt(&0) {
                right_vec.into_iter().for_each(|value| {
                    if let Some(value) = value {
                        insertion_vec.push(value)
                    }
                })
            }
            self.next_direction();
            self.consume(consumer_vec, insertion_vec);
        }
    }

    // Go left until you can no longer go left
    fn left(&mut self, consumer_vec: &mut Vec<Vec<u8>>, insertion_vec: &mut Vec<u8>) {
        let bottom_vec = consumer_vec.pop();

        if let Some(mut bottom_vec) = bottom_vec {
            while let Some(value) = bottom_vec.pop() {
                insertion_vec.push(value)
            }
            self.next_direction();
            self.consume(consumer_vec, insertion_vec);
        }
    }

    // Go up until there is no more up left
    fn up(&mut self, consumer_vec: &mut Vec<Vec<u8>>, insertion_vec: &mut Vec<u8>) {
        if consumer_vec.len().gt(&0) {
            let right_vec = consumer_vec
                .iter_mut()
                .map(|vec| vec.remove(0))
                .collect::<Vec<_>>();

            if right_vec.first().is_some() {
                right_vec
                    .into_iter()
                    .for_each(|value| insertion_vec.push(value));
                self.next_direction();
                self.consume(consumer_vec, insertion_vec);
            }
        }
    }
}

struct SpiralMatrix {
    matrix: Vec<Vec<u8>>,
    direction: Direction,
}

impl SpiralMatrix {
    fn new(matrix: Vec<Vec<u8>>) -> Self {
        Self {
            matrix,
            direction: Default::default(),
        }
    }

    /// Requirements state to solve for an m x n matrix. Thus this solution has been optimized for all internal
    /// vectors being of length n. Iteration through rows ends once a last index for all rows has
    /// been consumed.
    fn solve(&mut self) -> Vec<u8> {
        let mut return_vec = Vec::new();
        self.direction.consume(&mut self.matrix, &mut return_vec);
        return_vec
    }
}

#[cfg(test)]
mod tests {
    use super::SpiralMatrix;

    #[test]
    fn test_spiral_matrix() {
        let ref_vec = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]
            .map(|arr| arr.to_vec())
            .to_vec();
        let mut spiral_matrix = SpiralMatrix::new(ref_vec);
        assert_eq!(
            spiral_matrix.solve(),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        )
    }

    #[test]
    fn test_spiral_matrix_secondary() {
        let ref_vec = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .map(|arr| arr.to_vec())
            .to_vec();
        let mut spiral_matrix = SpiralMatrix::new(ref_vec);
        assert_eq!(spiral_matrix.solve(), vec![1, 2, 3, 6, 9, 8, 7, 4, 5])
    }

    #[test]
    fn test_complex() {
        let ref_vec = [
            [1, 2, 3, 4, 5, 6],
            [7, 8, 9, 10, 11, 12],
            [13, 14, 15, 16, 17, 18],
        ]
        .map(|arr| arr.to_vec())
        .to_vec();
        let mut spiral_matrix = SpiralMatrix::new(ref_vec);
        assert_eq!(
            spiral_matrix.solve(),
            vec![1, 2, 3, 4, 5, 6, 12, 18, 17, 16, 15, 14, 13, 7, 8, 9, 10, 11]
        )
    }
}
