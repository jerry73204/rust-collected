use crate::common::*;

/// The collection type that collects values into an `[A; SIZE]` array from an iterator with exact size `SIZE`;
#[derive(Debug)]
pub struct ExactArray<A, const SIZE: usize> {
    array: [MaybeUninit<A>; SIZE],
    len: usize,
    overflow: bool,
}

impl<A, const SIZE: usize> ExactArray<A, SIZE> {
    /// Returns an array if it is collected or extended from exacty `SIZE` values.
    pub fn into_array(self) -> Option<[A; SIZE]> {
        let ok = !self.overflow && self.len == SIZE;
        ok.then(|| unsafe { MaybeUninit::array_assume_init(self.array) })
    }
}

impl<A, const SIZE: usize> FromIterator<A> for ExactArray<A, SIZE> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut array = MaybeUninit::<A>::uninit_array::<SIZE>();
        let mut len = 0;
        let mut overflow = false;

        for item in iter.into_iter() {
            if len == SIZE {
                overflow = true;
                break;
            }
            array[len].write(item);
            len += 1;
        }

        Self {
            array,
            len,
            overflow,
        }
    }
}

impl<A, const SIZE: usize> Default for ExactArray<A, SIZE> {
    fn default() -> Self {
        Self {
            array: MaybeUninit::uninit_array(),
            len: 0,
            overflow: false,
        }
    }
}

impl<A, const SIZE: usize> Extend<A> for ExactArray<A, SIZE> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        if !self.overflow {
            for item in iter {
                if self.len == SIZE {
                    self.overflow = true;
                    break;
                }

                self.array[self.len].write(item);
                self.len += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_array_test() {
        let array: ExactArray<_, 3> = vec![3, 5, 1].into_iter().collect();
        assert_eq!(array.into_array(), Some([3, 5, 1]));

        let array: ExactArray<_, 3> = vec![3, 5, 1, 4].into_iter().collect();
        assert_eq!(array.into_array(), None);

        let array: ExactArray<_, 3> = vec![3].into_iter().collect();
        assert_eq!(array.into_array(), None);

        let array: ExactArray<usize, 0> = vec![].into_iter().collect();
        assert_eq!(array.into_array(), Some([]));
    }
}
