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
