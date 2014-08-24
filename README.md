Armadillo.rs
============

A wrapper around [armadillo]('http://arma.sourceforge.net'), a high performance c++ matrix library.
Currently this is a small subset of the library, just enough to do
simple construction and math. In addition to this, the only supported
type is `f32` matrices.

As always, pull requests are welcome!

For more documentation, run `cargo doc` and look at target/doc/armadillo/index.html.

```rust
extern crate armadillo;
use armadillo::{Mat, MatrixFuncs};

pub fn main() {
  let zeros = Mat::zeros(4, 4);
  let twos = Mat::ones(4, 4) * 2f32;
  let ones = Mat::ones(4, 4);
  assert_eq!(ones, zeros + 1f32);
  assert_eq!(ones, zeros * 3f32 + ones);

  // use `*` is element wise multiplication
  assert_eq!(twos * twos, twos + 2f32);

  let eye = Mat::eye(4, 4);
  // use `dot` for matrix multiplication
  assert_eq!(twos.dot(&eye), twos);

  let mut mat = Mat::ones(4, 4);
  assert_eq!(mat.at((0,0)), 1.0);
  *mat.at_mut((0, 0)) = 2.0;
  assert_eq!(mat.at((0,0)), 2f32);
}
```
