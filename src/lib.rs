//! Collection types that takes the maximum, the summation and more from iterators.
//!
//! Every collection type in the crate implements [`FromIterator`](core::iter::FromIterator),
//! [`Extend`](core::iter::Extend) and [`Default`](Default) traits. They can be built from
//! [`collect()`](Iterator::collect), and can be updated by [`extend()`](Extend::extend).
//!
//! For example, it makes it easy to compute maximum and minimum value from an iterator
//! using [`unzip()`](Iterator::unzip) in single step.
//!
//! ```rust
//! use collected::{MaxVal, MinVal};
//! let (min, max): (MinVal<_>, MaxVal<_>) = vec![3, 1, 5, 2, 4, 3, 6]
//!     .into_iter()
//!     .map(|val| (val, val))
//!     .unzip();
//! assert_eq!(min.unwrap(), 1);
//! assert_eq!(max.unwrap(), 6);
//! ```

mod add;
mod common;
mod group_hash_map;
mod max;
mod min;
mod mul;
mod product;
mod sum;
mod unique_hash_set;

pub use add::*;
pub use group_hash_map::*;
pub use max::*;
pub use min::*;
pub use mul::*;
pub use product::*;
pub use sum::*;
pub use unique_hash_set::*;
