use crate::common::*;

pub struct SumCollector<A>(pub A);

impl<A> FromIterator<A> for SumCollector<A>
where
    A: Sum<A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let sum: A = iter.into_iter().sum();
        Self(sum)
    }
}

impl<A> SumCollector<A> {
    pub fn get(self) -> A {
        self.0
    }
}
