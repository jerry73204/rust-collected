use crate::common::*;

/// A counter that counts witnessed items when collected or extended from an iterator.
#[derive(Debug, Clone)]
pub struct Count<T> {
    count: usize,
    _phantom: PhantomData<T>,
}

impl<T> Count<T> {
    pub fn get(&self) -> usize {
        self.count
    }
}

impl<T> Default for Count<T> {
    fn default() -> Self {
        Self {
            count: 0,
            _phantom: PhantomData,
        }
    }
}

impl<A> FromIterator<A> for Count<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self {
            count: iter.into_iter().count(),
            _phantom: PhantomData,
        }
    }
}

impl<A> Extend<A> for Count<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        self.count += iter.into_iter().count();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_test() {
        {
            let mut count: Count<_> = iter::empty().collect();
            count.extend(0..10);
            assert!(count.get() == 10);
        }

        {
            let mut count: Count<_> = (0..10).collect();
            assert!(count.get() == 10);

            count.extend(10..20);
            assert!(count.get() == 20);

            count.extend(iter::empty());
            assert!(count.get() == 20);
        }
    }
}
