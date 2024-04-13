struct MergeIntervals {
    intervals: Vec<[u8; 2]>,
}

impl MergeIntervals {
    fn new(intervals: Vec<[u8; 2]>) -> Self {
        Self { intervals }
    }

    // Solution is really simple. If range[1][1] and range[2][2] encapsulate both range[1,2] and
    // range[2,1], this means that a new range of indexes [[1,1][2,2]] is yielded.
    fn solve(&mut self) {
        // Sort the array, such that all min ranges are arranged.
        self.intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut first_set = self.intervals.iter_mut().peekable();

        while let Some(set1) = first_set.next() {
            while let Some(set2) = first_set.peek_mut() {
                let set1_min = set1[0];
                let set1_max = set1[1];
                let set2_min = set2[0];
                let set2_max = set2[1];

                // In the solution of this question, it is not stated that all ranges are somewhat valid, i.e
                // range[1][1] < range[2][2], otherwise all of range 2 would be ignored. This
                // basically marks set 2 as redundant
                if set1_min >= set2_max {
                    **set2 = [0, 0];
                    // first_set.next_back();
                    // break;
                } else if set1_max >= set2_min {
                    *set1 = [set1_min, set2_max];
                    // Since set2 != index 0, this means that both index0 and index1 of set2 >> 0.
                    // Therefore we can filter all [0,0] arrays later
                    **set2 = [0, 0];
                    // Since we are only peeking, we need to compare to next
                    first_set.next();
                    continue;
                    // first_set.next_back();
                }

                // If it's none of these conditions, that means that the next set is a range
                // completely outside of the current bounds and first_set.next() should be
                // triggered.
                break;
            }
        }
        self.intervals.retain(|val| val[0] != 0 && val[1] != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::MergeIntervals;

    #[test]
    fn test_merge_intervals_base() {
        let mut intervals = MergeIntervals::new(vec![[1, 3], [2, 6], [8, 10], [15, 18]]);
        intervals.solve();
        assert_eq!(intervals.intervals, vec![[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn test_merge_intervals_complex() {
        let mut intervals = MergeIntervals::new(vec![[1, 3], [2, 6], [8, 16], [15, 18]]);
        intervals.solve();
        assert_eq!(intervals.intervals, vec![[1, 6], [8, 18]]);
    }

    #[test]
    fn test_merge_intervals_long() {
        let mut intervals = MergeIntervals::new(vec![[1, 3], [2, 6], [8, 16], [15, 18], [16, 19]]);
        intervals.solve();
        assert_eq!(intervals.intervals, vec![[1, 6], [8, 19]]);
    }

    #[test]
    fn test_merge_intervals_single() {
        let mut intervals = MergeIntervals::new(vec![[1, 4], [4, 5]]);
        intervals.solve();
        assert_eq!(intervals.intervals, vec![[1, 5]]);
    }
}
