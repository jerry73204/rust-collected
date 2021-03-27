use crate::common::*;

pub struct MulCollector<A>(pub Option<A>);

impl<A> FromIterator<A> for MulCollector<A>
where
    A: Mul<A, Output = A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let prod = iter.into_iter().fold1(|lhs, rhs| lhs * rhs);
        Self(prod)
    }
}

impl<A> MulCollector<A> {
    pub fn unwrap(self) -> A {
        self.0.unwrap()
    }

    pub fn get(self) -> Option<A> {
        self.0
    }
}

impl<A> From<MulCollector<A>> for Option<A> {
    fn from(collector: MulCollector<A>) -> Self {
        collector.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_test() {
        assert_eq!(
            iter::repeat(2)
                .take(10)
                .collect::<MulCollector<usize>>()
                .unwrap(),
            1024
        );
    }
}
