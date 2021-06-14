use crate::common::*;

/// A collection that multiplies values with [`Mul`](Mul) trait.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct MulVal<A>(Option<A>);

impl<A> FromIterator<A> for MulVal<A>
where
    A: Mul<A, Output = A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let prod = iter.into_iter().fold1(|lhs, rhs| lhs * rhs);
        Self(prod)
    }
}

impl<A> MulVal<A> {
    pub fn unwrap(self) -> A {
        self.0.unwrap()
    }

    pub fn get(&self) -> Option<&A> {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Option<A> {
        self.0
    }
}

impl<A> From<MulVal<A>> for Option<A> {
    fn from(collector: MulVal<A>) -> Self {
        collector.0
    }
}

impl<A> Extend<A> for MulVal<A>
where
    A: Mul<A, Output = A>,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let prod = self
            .0
            .take()
            .into_iter()
            .chain(iter)
            .fold1(|lhs, rhs| lhs * rhs);
        self.0 = prod;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_test() {
        let mut prod: MulVal<usize> = iter::repeat(2).take(10).collect();
        assert_eq!(prod.unwrap(), 1024);

        prod.extend(1..=5);
        assert_eq!(prod.unwrap(), 122880);
    }
}
