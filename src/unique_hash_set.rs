use crate::common::*;

/// A collection that takes unique values into a set.
///
/// It maintains a [`HashSet`](HashSet) internally. When it is
/// built from iterator or extended, it expects unique input values.
/// Otherwise, it empties out the internal and ignore future values.
#[derive(Debug, Clone)]
pub struct UniqueHashSet<A>(Option<HashSet<A>>);

impl<A> Default for UniqueHashSet<A> {
    fn default() -> Self {
        Self(Some(HashSet::new()))
    }
}

impl<A> FromIterator<A> for UniqueHashSet<A>
where
    A: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let set = HashSet::new();
        let set = iter.into_iter().try_fold(set, |mut set, item| {
            let ok = set.insert(item);
            ok.then(|| set)
        });
        Self(set)
    }
}

impl<A> Extend<A> for UniqueHashSet<A>
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

impl<A> UniqueHashSet<A> {
    pub fn unwrap(self) -> HashSet<A> {
        self.0.unwrap()
    }

    pub fn get(&self) -> Option<&HashSet<A>> {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Option<HashSet<A>> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;

    #[test]
    fn unque_hash_set_test() {
        let mut set: UniqueHashSet<usize> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(set.get(), Some(&hashset! {1, 2, 3}));

        set.extend(vec![4, 5]);
        assert_eq!(set.get(), Some(&hashset! {1, 2, 3, 4, 5}));
        set.extend(vec![6, 6]);
        assert_eq!(set.get(), None);
    }
}
