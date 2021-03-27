use crate::common::*;

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
