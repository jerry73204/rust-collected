use crate::common::*;

/// A wrapper around collection `U` that retains only last `N` values.
#[derive(Debug, Clone)]
pub struct LastN<U, A, const N: usize>
where
    U: FromIterator<A>,
{
    buffer: VecDeque<A>,
    _phantom: PhantomData<U>,
}

impl<U, A, const N: usize> LastN<U, A, N>
where
    U: FromIterator<A>,
{
    /// Collects last `N` values into the collection `U`.
    pub fn into_inner(self) -> U {
        self.buffer.into_iter().collect()
    }
}

impl<U, A, const N: usize> FromIterator<A> for LastN<U, A, N>
where
    U: FromIterator<A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut buffer = VecDeque::new();
        let mut iter = iter.into_iter();

        for item in &mut iter {
            buffer.push_back(item);
            if buffer.len() > N {
                buffer.pop_front();
                break;
            }
        }

        for item in iter {
            buffer.push_back(item);
            buffer.pop_front();
        }

        Self {
            buffer,
            _phantom: PhantomData,
        }
    }
}

impl<U, A, const N: usize> Default for LastN<U, A, N>
where
    U: FromIterator<A>,
{
    fn default() -> Self {
        Self {
            buffer: VecDeque::new(),
            _phantom: PhantomData,
        }
    }
}

impl<U, A, const N: usize> Extend<A> for LastN<U, A, N>
where
    U: FromIterator<A>,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();

        for item in &mut iter {
            self.buffer.push_back(item);
            if self.buffer.len() > N {
                self.buffer.pop_front();
                break;
            }
        }

        for item in iter {
            self.buffer.push_back(item);
            self.buffer.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_n_test() {
        let last_n: LastN<Vec<_>, _, 3> = vec![3, 1, 5, 2].into_iter().collect();
        assert_eq!(last_n.into_inner(), vec![1, 5, 2]);

        let last_n: LastN<Vec<_>, usize, 3> = vec![].into_iter().collect();
        assert_eq!(last_n.into_inner(), vec![]);

        let last_n: LastN<Vec<_>, _, 0> = vec![2, 4, 1].into_iter().collect();
        assert_eq!(last_n.into_inner(), vec![]);
    }
}
