use crate::common::*;

/// A collection that takes unique values into a [BTreeSet](BTreeSet).
///
/// It maintains a [`BTreeSet`](BTreeSet) internally. When it is
/// built from iterator or extended, it expects unique input values.
/// Otherwise, it empties out the internal and ignore future values.
#[derive(Debug, Clone)]
pub struct UniqueBTreeSet<A>(Option<BTreeSet<A>>);

impl<A> Default for UniqueBTreeSet<A>
where
    A: Ord,
{
    fn default() -> Self {
        Self(Some(BTreeSet::new()))
    }
}

impl<A> FromIterator<A> for UniqueBTreeSet<A>
where
    A: Ord,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let set = BTreeSet::new();
        let set = iter.into_iter().try_fold(set, |mut set, item| {
            let ok = set.insert(item);
            ok.then(|| set)
        });
        Self(set)
    }
}

impl<A> Extend<A> for UniqueBTreeSet<A>
where
    A: Ord,
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

impl<A> UniqueBTreeSet<A> {
    pub fn unwrap(self) -> BTreeSet<A> {
        self.0.unwrap()
    }

    pub fn get(&self) -> Option<&BTreeSet<A>> {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Option<BTreeSet<A>> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::btreeset;

    #[test]
    fn unque_hash_set_test() {
        let mut set: UniqueBTreeSet<usize> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(set.get(), Some(&btreeset! {1, 2, 3}));

        set.extend(vec![4, 5]);
        assert_eq!(set.get(), Some(&btreeset! {1, 2, 3, 4, 5}));
        set.extend(vec![6, 6]);
        assert_eq!(set.get(), None);
    }
}
