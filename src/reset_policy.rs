pub fn linear_reset_policy(confirmation_depth: i64, max_tolerable_height_difference: i64) -> Vec<i64> {
    (0..=confirmation_depth).map(|height_a| {
        max_tolerable_height_difference * height_a / confirmation_depth
    }).collect()
}

#[derive(Clone)]
pub struct PolicyIterator {
    size: usize,
    max_value: i64,
    current_array: Vec<i64>,
    done: bool,
}

impl PolicyIterator {
    pub fn new(size: usize, max_value: i64) -> Self {
        Self {
            size,
            max_value,
            current_array: vec![0; size],
            done: false,
        }
    }
}

impl Iterator for PolicyIterator {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current_array.clone();

        // Generate the next non-decreasing array
        for i in (1..self.size).rev() {
            if self.current_array[i] < self.max_value {
                self.current_array[i] += 1;
                for j in (i + 1)..self.size {
                    self.current_array[j] = self.current_array[i];
                }
                return Some(result);
            }
        }

        self.done = true;
        Some(result)
    }
}
