use crate::common::*;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct ProductCollector<A>(pub A);

impl<A> FromIterator<A> for ProductCollector<A>
where
    A: Product<A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let product: A = iter.into_iter().product();
        Self(product)
    }
}

impl<A> ProductCollector<A> {
    pub fn get(self) -> A {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_test() {
        assert_eq!(
            iter::repeat(2)
                .take(10)
                .collect::<ProductCollector<usize>>()
                .get(),
            1024
        );
    }
}
