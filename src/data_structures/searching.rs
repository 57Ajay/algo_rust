pub trait BinarySearchExt<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearchExt<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if &self[mid] < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if &self[mid] <= x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
