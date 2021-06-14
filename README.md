# Collected

Collection types that takes the maximum, the summation and more from iterators in Rust.

## Usage
Every collection type in the crate implements `FromIterator`, `Extend` and `Default` traits.
They can be built from `collect()`, and can be updated by `extend()`.

For example, it makes it easy to compute maximum and minimum value from an iterator
using `unzip()` in single step.

```rust
use collected::{MaxVal, MinVal};
let (min, max): (MinVal<_>, MaxVal<_>) = vec![3, 1, 5, 2, 4, 3, 6]
    .into_iter()
    .map(|val| (val, val))
    .unzip();
assert_eq!(min.unwrap(), 1);
assert_eq!(max.unwrap(), 6);
```


## License

MIT License. See [license file](LICENSE.txt).
