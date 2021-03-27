use crate::common::*;

pub struct MinCollector<A>(pub Option<A>);

impl<A> FromIterator<A> for MinCollector<A>
where
    A: Ord,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let sum = iter.into_iter().fold1(|lhs, rhs| lhs.min(rhs));
        Self(sum)
    }
}

impl<A> MinCollector<A> {
    pub fn unwrap(self) -> A {
        self.0.unwrap()
    }

    pub fn get(self) -> Option<A> {
        self.0
    }
}

impl<A> From<MinCollector<A>> for Option<A> {
    fn from(collector: MinCollector<A>) -> Self {
        collector.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_test() {
        assert_eq!((1..100).collect::<MinCollector<usize>>().unwrap(), 1);
    }
}
