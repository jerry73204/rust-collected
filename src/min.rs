use crate::common::*;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct MinVal<A>(Option<A>);

impl<A> FromIterator<A> for MinVal<A>
where
    A: Ord,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let min = iter.into_iter().fold1(|lhs, rhs| lhs.min(rhs));
        Self(min)
    }
}

impl<A> MinVal<A> {
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

impl<A> From<MinVal<A>> for Option<A> {
    fn from(collector: MinVal<A>) -> Self {
        collector.0
    }
}

impl<A> Extend<A> for MinVal<A>
where
    A: Ord,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let min = self
            .0
            .take()
            .into_iter()
            .chain(iter)
            .fold1(|lhs, rhs| lhs.min(rhs));
        self.0 = min;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_test() {
        let mut min: MinVal<isize> = (1..100).collect();
        assert_eq!(min.unwrap(), 1);

        min.extend((-200)..(-100));
        assert_eq!(min.unwrap(), -200);
    }
}
