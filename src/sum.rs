use crate::common::*;

/// A collection taking the summation from values with [`Sum`](Sum) trait.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct SumVal<A>(pub A);

impl<A> FromIterator<A> for SumVal<A>
where
    A: Sum<A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let sum: A = iter.into_iter().sum();
        Self(sum)
    }
}

impl<A> SumVal<A> {
    pub fn get(&self) -> &A {
        &self.0
    }

    pub fn into_inner(self) -> A {
        self.0
    }
}

impl<A> Extend<A> for SumVal<A>
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
        let mut sum: SumVal<usize> = iter::repeat(1).take(100).collect();
        assert_eq!(*sum.get(), 100);

        sum.extend(1..=100);
        assert_eq!(*sum.get(), 5150);
    }
}
