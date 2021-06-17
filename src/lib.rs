#![cfg_attr(
    feature = "unstable",
    feature(
        maybe_uninit_uninit_array,
        maybe_uninit_extra,
        maybe_uninit_array_assume_init
    )
)]

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
#[cfg(feature = "unstable")]
mod exact_array;
#[cfg(feature = "unstable")]
mod fill_array;
mod from_unique_hash;
mod from_unique_ord;
mod group_hash_map;
mod last_n;
mod max;
mod min;
mod mul;
mod product;
mod sum;
mod topk;
mod unique_btree_set;
mod unique_hash_set;
#[cfg(feature = "indexmap")]
mod unique_index_set;
mod uniquify_hash;
mod uniquify_ord;

pub use add::*;
#[cfg(feature = "unstable")]
pub use exact_array::*;
#[cfg(feature = "unstable")]
pub use fill_array::*;
pub use from_unique_hash::*;
pub use from_unique_ord::*;
pub use group_hash_map::*;
pub use last_n::*;
pub use max::*;
pub use min::*;
pub use mul::*;
pub use product::*;
pub use sum::*;
pub use topk::*;
pub use unique_btree_set::*;
pub use unique_hash_set::*;
#[cfg(feature = "indexmap")]
pub use unique_index_set::*;
pub use uniquify_hash::*;
pub use uniquify_ord::*;
