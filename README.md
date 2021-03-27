# Rust Collected

Summation, product, maximum and more special collectors for Rust iterators.

## Example

```rust
use collected::MaxCollector;

fn main() {
    let max: MaxCollector<usize> = (1..100).collect();
    assert_eq!(max.unwrap(), 99);
}
```

## License

MIT License. See [license file](LICENSE.txt).
