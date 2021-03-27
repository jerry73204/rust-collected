use crate::common::*;

pub struct MaxCollector<A>(pub Option<A>);

impl<A> FromIterator<A> for MaxCollector<A>
where
    A: Ord,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let sum = iter.into_iter().fold1(|lhs, rhs| lhs.max(rhs));
        Self(sum)
    }
}

impl<A> MaxCollector<A> {
    pub fn unwrap(self) -> A {
        self.0.unwrap()
    }

    pub fn get(self) -> Option<A> {
        self.0
    }
}

impl<A> From<MaxCollector<A>> for Option<A> {
    fn from(collector: MaxCollector<A>) -> Self {
        collector.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_test() {
        assert_eq!((1..100).collect::<MaxCollector<usize>>().unwrap(), 99);
    }
}
