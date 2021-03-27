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

impl<A> Extend<A> for ProductCollector<A>
where
    A: Product<A>,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let default: A = iter::empty().product();
        let prod = iter::once(mem::replace(&mut self.0, default))
            .chain(iter)
            .product();
        self.0 = prod;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_test() {
        let mut prod: ProductCollector<usize> = iter::repeat(2).take(10).collect();
        assert_eq!(prod.get(), 1024);

        prod.extend(1..=5);
        assert_eq!(prod.get(), 122880);
    }
}
