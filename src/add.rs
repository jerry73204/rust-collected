use crate::common::*;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct AddVal<A>(Option<A>);

impl<A> FromIterator<A> for AddVal<A>
where
    A: Add<A, Output = A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let sum = iter.into_iter().fold1(|lhs, rhs| lhs + rhs);
        Self(sum)
    }
}

impl<A> AddVal<A> {
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

impl<A> From<AddVal<A>> for Option<A> {
    fn from(collector: AddVal<A>) -> Self {
        collector.0
    }
}

impl<A> Extend<A> for AddVal<A>
where
    A: Add<A, Output = A>,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let sum = self
            .0
            .take()
            .into_iter()
            .chain(iter)
            .fold1(|lhs, rhs| lhs + rhs);
        self.0 = sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let mut sum: AddVal<usize> = iter::repeat(1).take(100).collect();
        assert_eq!(sum.unwrap(), 100);

        sum.extend(1..=100);
        assert_eq!(sum.unwrap(), 5150);
    }
}
