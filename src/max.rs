use crate::common::*;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct MaxVal<A>(Option<A>);

impl<A> FromIterator<A> for MaxVal<A>
where
    A: Ord,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let max = iter.into_iter().fold1(|lhs, rhs| lhs.max(rhs));
        Self(max)
    }
}

impl<A> MaxVal<A> {
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

impl<A> From<MaxVal<A>> for Option<A> {
    fn from(collector: MaxVal<A>) -> Self {
        collector.0
    }
}

impl<A> Extend<A> for MaxVal<A>
where
    A: Ord,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let max = self
            .0
            .take()
            .into_iter()
            .chain(iter)
            .fold1(|lhs, rhs| lhs.max(rhs));
        self.0 = max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_test() {
        let mut max: MaxVal<usize> = (1..100).collect();
        assert_eq!(max.unwrap(), 99);

        max.extend(100..200);
        assert_eq!(max.unwrap(), 199);
    }
}
