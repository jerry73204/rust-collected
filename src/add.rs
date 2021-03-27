use crate::common::*;

pub struct AddCollector<A>(pub Option<A>);

impl<A> FromIterator<A> for AddCollector<A>
where
    A: Add<A, Output = A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let sum = iter.into_iter().fold1(|lhs, rhs| lhs + rhs);
        Self(sum)
    }
}

impl<A> AddCollector<A> {
    pub fn unwrap(self) -> A {
        self.0.unwrap()
    }

    pub fn get(self) -> Option<A> {
        self.0
    }
}

impl<A> From<AddCollector<A>> for Option<A> {
    fn from(collector: AddCollector<A>) -> Self {
        collector.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(
            iter::repeat(1)
                .take(100)
                .collect::<AddCollector<usize>>()
                .unwrap(),
            100
        );
    }
}
