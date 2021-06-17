use crate::common::*;

/// A wrapper around a collection `U` that can only be collected or extended from an unique-valued iterator.
///
/// The wrapper maintains a [HashSet](HashSet) internally. If an incoming value `A` is duplicated,
/// it clears the internal collections and ignores future values.
pub struct FromUniqueHash<U, A>(Option<State<U, A>>)
where
    A: Hash + Eq + Clone;

struct State<U, A>
where
    A: Hash + Eq + Clone,
{
    inner: U,
    set: HashSet<A>,
}

impl<U, A> FromUniqueHash<U, A>
where
    A: Hash + Eq + Clone,
{
    pub fn get(&self) -> Option<&U> {
        self.0.as_ref().map(|state| &state.inner)
    }

    pub fn into_inner(self) -> Option<U> {
        self.0.map(|state| state.inner)
    }
}

impl<U, A> FromIterator<A> for FromUniqueHash<U, A>
where
    U: FromIterator<A>,
    A: Hash + Eq + Clone,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut set = Some(HashSet::new());
        let inner: U = iter
            .into_iter()
            .map(|item| {
                let ok = set.as_mut().unwrap().insert(item.clone());

                if ok {
                    Some(item)
                } else {
                    set = None;
                    None
                }
            })
            .fuse()
            .flatten()
            .collect();

        let state = set.map(|set| State { inner, set });

        Self(state)
    }
}

impl<U, A> Extend<A> for FromUniqueHash<U, A>
where
    U: Extend<A>,
    A: Hash + Eq + Clone,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        if let Some(state) = &mut self.0 {
            let State { inner, set } = state;
            let mut ok = true;

            let iter = iter
                .into_iter()
                .map(|item| {
                    ok &= set.insert(item.clone());
                    ok.then(|| item)
                })
                .fuse()
                .flatten();
            inner.extend(iter);

            if !ok {
                self.0 = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_hash_test() {
        let unique: FromUniqueHash<Vec<_>, _> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(unique.into_inner(), Some(vec![1, 2, 3]));

        let unique: FromUniqueHash<Vec<_>, _> = vec![1, 2, 3, 3].into_iter().collect();
        assert_eq!(unique.into_inner(), None);

        let mut unique: FromUniqueHash<Vec<_>, _> = vec![1, 2, 3].into_iter().collect();
        unique.extend(vec![4]);
        assert_eq!(unique.get(), Some(&vec![1, 2, 3, 4]));
        unique.extend(vec![4]);
        assert_eq!(unique.get(), None);
    }
}
