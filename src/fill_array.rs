use crate::common::*;

/// The collection type that collects values into an `[A; SIZE]` array from an iterator with minimum size `SIZE`;
#[derive(Debug)]
pub struct FillArray<A, const SIZE: usize> {
    array: [MaybeUninit<A>; SIZE],
    len: usize,
}

impl<A, const SIZE: usize> FillArray<A, SIZE> {
    /// Returns an array if it is collected or extended from at least `SIZE` values.
    pub fn into_array(self) -> Option<[A; SIZE]> {
        let ok = self.len == SIZE;
        ok.then(|| unsafe { MaybeUninit::array_assume_init(self.array) })
    }
}

impl<A, const SIZE: usize> FromIterator<A> for FillArray<A, SIZE> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut array = MaybeUninit::<A>::uninit_array::<SIZE>();
        let mut len = 0;

        for item in iter.into_iter() {
            if len == SIZE {
                break;
            }
            array[len].write(item);
            len += 1;
        }

        Self { array, len }
    }
}

impl<A, const SIZE: usize> Default for FillArray<A, SIZE> {
    fn default() -> Self {
        Self {
            array: MaybeUninit::uninit_array(),
            len: 0,
        }
    }
}

impl<A, const SIZE: usize> Extend<A> for FillArray<A, SIZE> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for item in iter {
            if self.len == SIZE {
                break;
            }

            self.array[self.len].write(item);
            self.len += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_array_test() {
        let array: FillArray<_, 3> = vec![3, 5, 1].into_iter().collect();
        assert_eq!(array.into_array(), Some([3, 5, 1]));

        let array: FillArray<_, 3> = vec![3, 5, 1, 4].into_iter().collect();
        assert_eq!(array.into_array(), Some([3, 5, 1]));

        let array: FillArray<_, 3> = vec![3].into_iter().collect();
        assert_eq!(array.into_array(), None);

        let array: FillArray<usize, 0> = vec![].into_iter().collect();
        assert_eq!(array.into_array(), Some([]));
    }
}
