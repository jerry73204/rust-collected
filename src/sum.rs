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

impl<A> Extend<A> for SumCollector<A>
where
    A: Sum<A>,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let default: A = iter::empty().sum();
        let sum = iter::once(mem::replace(&mut self.0, default))
            .chain(iter)
            .sum();
        self.0 = sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        let mut sum: SumCollector<usize> = iter::repeat(1).take(100).collect();
        assert_eq!(sum.get(), 100);

        sum.extend(1..=100);
        assert_eq!(sum.get(), 5150);
    }
}
