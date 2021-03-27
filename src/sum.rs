use crate::common::*;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(
            iter::repeat(1)
                .take(100)
                .collect::<SumCollector<usize>>()
                .get(),
            100
        );
    }
}
