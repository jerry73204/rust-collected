use crate::common::*;

/// A collection taking the product from values with [`Product`](Product) trait.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct ProductVal<A>(pub A);

impl<A> FromIterator<A> for ProductVal<A>
where
    A: Product<A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let product: A = iter.into_iter().product();
        Self(product)
    }
}

impl<A> ProductVal<A> {
    pub fn get(&self) -> &A {
        &self.0
    }

    pub fn into_inner(self) -> A {
        self.0
    }
}

impl<A> Extend<A> for ProductVal<A>
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
        let mut prod: ProductVal<usize> = iter::repeat(2).take(10).collect();
        assert_eq!(*prod.get(), 1024);

        prod.extend(1..=5);
        assert_eq!(*prod.get(), 122880);
    }
}
