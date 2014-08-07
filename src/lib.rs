// TODO Add in a high level heres how things work docs
//
#![feature(unsafe_destructor)]
#![feature(globs)]
#![feature(macro_rules)]
extern crate debug;
extern crate libc;

use std::c_str::CString;
use std::fmt;
use libc::{c_uint, c_char, c_float, c_double};
use gen_ffi::*;

mod gen_ffi;

pub struct Mat {
  raw : *mut Matf32Raw,
  //TODO do some checking on shape
  shape : (uint, uint)
}

impl Mat {
  fn new_from_raw(raw : *mut Matf32Raw) -> Mat {
    unsafe {
      let shape = (arma_Mat_f32_n_rows(raw) as uint, arma_Mat_f32_n_cols(raw) as uint);
      Mat { raw: raw, shape : shape }
    }
  }

  /// Construct a new matrix filled with zeros
  pub fn zeros(rows : uint, cols : uint) -> Mat {
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_zeros(rows as c_uint, cols as c_uint))
    }
  }

  /// Construct a new matrix filled with ones
  pub fn ones(rows : uint, cols : uint) -> Mat {
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_ones(rows as c_uint, cols as c_uint))
    }
  }

  /// Construct a matrix with 1's on the diagonals
  pub fn eye(rows : uint, cols : uint) -> Mat {
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_eye(rows as c_uint, cols as c_uint))
    }
  }

  /// Construct a matrix from random values sampled from a
  /// normal distribution centered at 0 with std of 1.
  pub fn randn(rows : uint, cols : uint) -> Mat {
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_randn(rows as c_uint, cols as c_uint))
    }
  }

  /// Construct a matrix with from random values sampled from a
  /// uniform distribution of [0, 1)
  pub fn randu(rows : uint, cols : uint) -> Mat {
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_randu(rows as c_uint, cols as c_uint))
    }
  }

  /// Index the matrix by row and column
  pub fn at(&self, row : uint, col : uint) -> f32 {
    unsafe {
      arma_Mat_f32_at(self.raw, row as c_uint, col as c_uint) as f32
    }
  }
}

/// Trait to enable operations with matrix
pub trait MathWithMat {
  fn do_add(&self, mat : &Mat) -> Mat;
  fn do_sub(&self, mat : &Mat) -> Mat;
  fn do_mul(&self, mat : &Mat) -> Mat;
  fn do_div(&self, mat : &Mat) -> Mat;
}

impl MathWithMat for f32 {
  fn do_add(&self, mat : &Mat) -> Mat{
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_add_f32(mat.raw, *self))
    }
  }

  fn do_sub(&self, mat : &Mat) -> Mat{
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_sub_f32(mat.raw, *self))
    }
  }

  fn do_mul(&self, mat : &Mat) -> Mat{
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_mul_f32(mat.raw, *self))
    }
  }

  fn do_div(&self, mat : &Mat) -> Mat{
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_div_f32(mat.raw, *self))
    }
  }
}

impl MathWithMat for Mat {
  fn do_add(&self, mat : &Mat) -> Mat{
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_add_Mat_f32(mat.raw, self.raw))
    }
  }

  fn do_sub(&self, mat : &Mat) -> Mat{
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_sub_Mat_f32(mat.raw, self.raw))
    }
  }

  fn do_mul(&self, mat : &Mat) -> Mat{
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_mul_Mat_f32(mat.raw, self.raw))
    }
  }

  fn do_div(&self, mat : &Mat) -> Mat{
    unsafe {
      println!("Doing div {}", self)
      println!("Mat2 {}", mat);
      Mat::new_from_raw(arma_Mat_f32_div_Mat_f32(mat.raw, self.raw))
    }
  }
}

impl<T : MathWithMat> Add<T, Mat> for Mat {
  fn add(&self, rhs : &T) -> Mat {
    rhs.do_add(self)
  }
}

impl<T : MathWithMat> Sub<T, Mat> for Mat {
  fn sub(&self, rhs : &T) -> Mat {
    rhs.do_sub(self)
  }
}

impl<T : MathWithMat> Mul<T, Mat> for Mat {
  fn mul(&self, rhs : &T) -> Mat {
    rhs.do_mul(self)
  }
}

impl<T : MathWithMat> Div<T, Mat> for Mat {
  fn div(&self, rhs : &T) -> Mat {
    rhs.do_div(self)
  }
}

#[unsafe_destructor]
impl Drop for Mat {
  fn drop(&mut self) {
    unsafe {
      arma_Mat_f32_free(self.raw);
    }
  }
}

// TODO finish this implementation
impl fmt::Show for Mat {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
    unsafe {
      let c_str = arma_Mat_f32_print(self.raw);
      println!("{:?}", c_str);
      let string = String::from_str(CString::new(c_str, false).as_str().unwrap());
      write!(formatter, "Mat{{shape:({}, {})\n {}\n}}", 0i, 0i, string)
    }
  }
}

#[cfg(test)]
mod test {
  use super::Mat;
  #[test]
  fn zeros() {
    let zeros = Mat::zeros(10, 20);
    assert_eq!(0f32, zeros.at(0,0));
    assert_eq!(0f32, zeros.at(9,15));
  }

  #[test]
  fn ones() {
    let ones = Mat::ones(10, 20);
    assert_eq!(1f32, ones.at(0,0));
    assert_eq!(1f32, ones.at(9,15));
  }

  #[test]
  fn eye() {
    let eye = Mat::eye(10, 10);
    assert_eq!(1f32, eye.at(0,0));
    assert_eq!(1f32, eye.at(3,3));
    assert_eq!(0f32, eye.at(9,3));
  }

  #[test]
  fn rands() {
    let randu = Mat::randu(10, 10);
    let randn = Mat::randn(10, 10);
    // Spot check, no understanding of distribution
    assert!(randn.at(0,0) != randu.at(0,0));
    assert!(randu.at(2,1) != randu.at(2,0));
    assert!(randn.at(2,1) != randn.at(2,0));
    assert!(randn.at(3,2) != randn.at(7,3));
    assert!(randu.at(3,2) != randu.at(7,3));
  }

  #[test]
  fn mat_add_f32() {
    let ones = Mat::ones(10, 20);
    let new = ones + 1f32;
    assert_eq!(2f32, new.at(9,15));
  }

  #[test]
  fn mat_sub_f32() {
    let ones = Mat::ones(10, 20);
    let new = ones - 1f32;
    assert_eq!(0f32, new.at(9,15));
  }

  #[test]
  fn mat_div_f32() {
    let ones = Mat::ones(10, 20);
    let new = ones / 4f32;
    assert_eq!(0.25f32, new.at(9,15));
  }

  #[test]
  fn mat_mul_f32() {
    let ones = Mat::ones(10, 20);
    let new = ones * 4f32;
    assert_eq!(4f32, new.at(9,15));
  }

  #[test]
  fn mat_add_mat() {
    let ones = Mat::ones(10, 20);
    let other = Mat::ones(10, 20);
    let new = ones + other;
    assert_eq!(2f32, new.at(9,15));
  }

  #[test]
  fn mat_sub_mat() {
    let ones = Mat::ones(10, 10);
    let other = Mat::ones(10, 10);
    let new = ones - other;
    assert_eq!(0f32, new.at(9,15));
  }

  #[test]
  fn mat_div_mat() {
    let ones = Mat::ones(10, 10)+1f32;
    let other = Mat::ones(10, 10)+100f32;
    println!("{}", ones);
    println!("{}", other);
    let new = ones / other;
    assert_eq!(0.5f32, new.at(9,15));
  }

  #[test]
  fn mat_mul_mat() {
    let ones = Mat::ones(10, 10);
    let other = Mat::eye(10, 10) + 1f32;
    let new = ones * other;
    assert_eq!(2f32, new.at(9,15));
  }
}
