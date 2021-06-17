use crate::common::*;

/// A wrapper around a collection `U` that ignores duplicated values from iterator.
///
/// The wrapper maintains a [BTreeSet](BTreeSet) internally. It inserts incoming value `A`
/// to collection `U` at most once, and ignores duplicated future values.
pub struct UniquifyOrd<U, A>
where
    A: Ord + Clone,
{
    inner: U,
    set: BTreeSet<A>,
}

impl<U, A> UniquifyOrd<U, A>
where
    A: Ord + Clone,
{
    pub fn get(&self) -> &U {
        &self.inner
    }

    pub fn into_inner(self) -> U {
        self.inner
    }
}

impl<U, A> FromIterator<A> for UniquifyOrd<U, A>
where
    U: FromIterator<A>,
    A: Ord + Clone,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut set = BTreeSet::new();
        let inner: U = iter
            .into_iter()
            .filter_map(|item| set.insert(item.clone()).then(|| item))
            .collect();

        Self { inner, set }
    }
}

impl<U, A> Extend<A> for UniquifyOrd<U, A>
where
    U: Extend<A>,
    A: Ord + Clone,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let Self { inner, set } = self;

        let iter = iter
            .into_iter()
            .filter_map(|item| set.insert(item.clone()).then(|| item));
        inner.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniquify_hash_test() {
        let unique: UniquifyOrd<Vec<_>, _> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(unique.into_inner(), vec![1, 2, 3]);

        let unique: UniquifyOrd<Vec<_>, _> = vec![1, 2, 2, 3, 3].into_iter().collect();
        assert_eq!(unique.into_inner(), vec![1, 2, 3]);

        let mut unique: UniquifyOrd<Vec<_>, _> = vec![1, 2, 3].into_iter().collect();
        unique.extend(vec![4]);
        assert_eq!(unique.get(), &vec![1, 2, 3, 4]);
        unique.extend(vec![4]);
        assert_eq!(unique.get(), &vec![1, 2, 3, 4]);
    }
}
