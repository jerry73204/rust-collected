use crate::common::*;

/// A collection that groups the tuples `(key, value)` by key.
#[derive(Debug, Clone)]
pub struct GroupHashMap<K, V>(HashMap<K, Vec<V>>);

impl<K, V> Default for GroupHashMap<K, V> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<K, V> FromIterator<(K, V)> for GroupHashMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let map = iter.into_iter().into_group_map();
        Self(map)
    }
}

impl<K, V> Extend<(K, V)> for GroupHashMap<K, V>
where
    K: Hash + Eq,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        let map = &mut self.0;

        iter.into_iter()
            .for_each(|(key, val)| match map.entry(key) {
                hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(val);
                }
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(vec![val]);
                }
            });
    }
}

impl<K, V> GroupHashMap<K, V> {
    pub fn get(&self) -> &HashMap<K, Vec<V>> {
        &self.0
    }

    pub fn into_inner(self) -> HashMap<K, Vec<V>> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn group_hash_map_test() {
        let mut map: GroupHashMap<char, usize> =
            vec![('a', 1), ('a', 2), ('b', 3)].into_iter().collect();
        assert_eq!(
            map.get(),
            &hashmap! {
                'a' => vec![1, 2],
                'b' => vec![3],
            }
        );

        map.extend(vec![('b', 4), ('c', 5)]);
        assert_eq!(
            map.get(),
            &hashmap! {
                'a' => vec![1, 2],
                'b' => vec![3, 4],
                'c' => vec![5],
            }
        );
    }
}
