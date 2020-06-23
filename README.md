# Bareiss algorithm

Bareiss algorithm calculates the determinant of an integer square matrix only using integer arithmetics. 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.7"
```

## Example

```rust

use bareiss::bareiss_determinant;

let mut matrix = vec![1, 2, 3, 10];
assert_eq!(bareiss_determinant(&mut matrix, 2).unwrap(), 4);

```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.