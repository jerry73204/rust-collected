use crate::common::*;

/// A collection doing nothing when collected or extended from an iterator.
///
/// By collecting/extending an iterator into `Noop`, each item is consumed and dropped.
#[derive(Debug, Clone)]
pub struct Noop<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for Noop<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<A> FromIterator<A> for Noop<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        iter.into_iter().for_each(|_| ());
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<A> Extend<A> for Noop<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        iter.into_iter().for_each(|_| ());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_test() {
        let mut noop: Noop<_> = iter::empty().collect();
        noop.extend(0..10);
    }
}
