use crate::common::*;
use indexmap::IndexSet;

/// A collection that takes unique values into a [IndexSet](IndexSet).
///
/// It maintains a [`IndexSet`](IndexSet) internally. When it is
/// built from iterator or extended, it expects unique input values.
/// Otherwise, it empties out the internal and ignore future values.
#[derive(Debug, Clone)]
pub struct UniqueIndexSet<A>(Option<IndexSet<A>>);

impl<A> Default for UniqueIndexSet<A> {
    fn default() -> Self {
        Self(Some(IndexSet::new()))
    }
}

impl<A> FromIterator<A> for UniqueIndexSet<A>
where
    A: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let set = IndexSet::new();
        let set = iter.into_iter().try_fold(set, |mut set, item| {
            let ok = set.insert(item);
            ok.then(|| set)
        });
        Self(set)
    }
}

impl<A> Extend<A> for UniqueIndexSet<A>
where
    A: Hash + Eq,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        if let Some(set) = &mut self.0 {
            for item in iter {
                if !set.insert(item) {
                    self.0 = None;
                    return;
                }
            }
        }
    }
}

impl<A> UniqueIndexSet<A> {
    pub fn unwrap(self) -> IndexSet<A> {
        self.0.unwrap()
    }

    pub fn get(&self) -> Option<&IndexSet<A>> {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Option<IndexSet<A>> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unque_hash_set_test() {
        let mut set: UniqueIndexSet<usize> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(set.get(), Some(&IndexSet::from_iter(vec![1, 2, 3])));

        set.extend(vec![4, 5]);
        assert_eq!(set.get(), Some(&IndexSet::from_iter(vec![1, 2, 3, 4, 5])));
        set.extend(vec![6, 6]);
        assert_eq!(set.get(), None);
    }
}
