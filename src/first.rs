use crate::common::*;

/// A collection that takes the first item when collected or extended from an iterator.
///
/// By collecting/extending an iterator into `First`, each item is consumed and dropped.
#[derive(Debug, Clone)]
pub struct First<T> {
    inner: Option<T>,
}

impl<T> First<T> {
    pub fn get(&self) -> Option<&T> {
        self.inner.as_ref()
    }

    pub fn into_inner(self) -> Option<T> {
        self.inner
    }
}

impl<T> Default for First<T> {
    fn default() -> Self {
        Self { inner: None }
    }
}

impl<A> FromIterator<A> for First<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let first = iter.next();
        iter.for_each(|_| ());

        Self { inner: first }
    }
}

impl<A> Extend<A> for First<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();
        let first = iter.next();
        iter.for_each(|_| ());

        self.inner = self.inner.take().or(first);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        {
            let mut first: First<_> = iter::empty().collect();
            assert!(first.get() == None);

            first.extend(0..10);
            assert!(first.get() == Some(&0));
        }

        {
            let mut first: First<_> = (0..10).collect();
            assert!(first.get() == Some(&0));

            first.extend(10..20);
            assert!(first.get() == Some(&0));

            first.extend(iter::empty());
            assert!(first.get() == Some(&0));
        }
    }
}
