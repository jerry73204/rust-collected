use crate::common::*;

/// A collection that takes the last item when collected or extended from an iterator.
#[derive(Debug, Clone)]
pub struct Last<T> {
    inner: Option<T>,
}

impl<T> Last<T> {
    pub fn get(&self) -> Option<&T> {
        self.inner.as_ref()
    }

    pub fn into_inner(self) -> Option<T> {
        self.inner
    }
}

impl<T> Default for Last<T> {
    fn default() -> Self {
        Self { inner: None }
    }
}

impl<A> FromIterator<A> for Last<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().last(),
        }
    }
}

impl<A> Extend<A> for Last<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        self.inner = iter.into_iter().last().or(self.inner.take());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_test() {
        {
            let mut last: Last<_> = iter::empty().collect();
            assert!(last.get() == None);

            last.extend(0..10);
            assert!(last.get() == Some(&9));
        }

        {
            let mut last: Last<_> = (0..10).collect();
            assert!(last.get() == Some(&9));

            last.extend(10..20);
            assert!(last.get() == Some(&19));

            last.extend(iter::empty());
            assert!(last.get() == Some(&19));
        }
    }
}
