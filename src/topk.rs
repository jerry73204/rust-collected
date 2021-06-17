use crate::common::*;

/// The collection that retains maximum `K` values.
#[derive(Debug, Clone)]
pub struct TopK<A, const K: usize>
where
    A: Ord,
{
    topk: MinMaxHeap<A>,
}

impl<A, const K: usize> TopK<A, K>
where
    A: Ord,
{
    pub fn get_heap(&self) -> &MinMaxHeap<A> {
        &self.topk
    }

    pub fn into_heap(self) -> MinMaxHeap<A> {
        self.topk
    }

    pub fn into_sorted_vec(self) -> Vec<A> {
        self.topk.into_vec_desc()
    }
}

impl<A, const K: usize> FromIterator<A> for TopK<A, K>
where
    A: Ord,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut topk = MinMaxHeap::new();
        let mut iter = iter.into_iter();

        for item in &mut iter {
            topk.push(item);
            if topk.len() > K {
                topk.pop_min();
                break;
            }
        }

        for item in iter {
            topk.push(item);
            topk.pop_min();
        }

        Self { topk }
    }
}

impl<A, const K: usize> Default for TopK<A, K>
where
    A: Ord,
{
    fn default() -> Self {
        Self {
            topk: MinMaxHeap::new(),
        }
    }
}

impl<A, const K: usize> Extend<A> for TopK<A, K>
where
    A: Ord,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();

        for item in &mut iter {
            self.topk.push(item);
            if self.topk.len() > K {
                self.topk.pop_min();
                break;
            }
        }

        for item in iter {
            self.topk.push(item);
            self.topk.pop_min();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_test() {
        let topk: TopK<_, 3> = vec![3, 1, 8, 2, 7, 7, 0, 2].into_iter().collect();
        assert_eq!(topk.into_sorted_vec(), vec![8, 7, 7]);
    }
}
