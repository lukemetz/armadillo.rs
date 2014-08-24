use ffi::*;
use std::c_str::CString;
use libc::c_uint;
use std::{fmt, mem, cmp};

pub struct Mat {
  pub raw : *mut Matf32Raw,
  pub shape : (uint, uint)
}


impl Mat {
  /// Mostly internal function to cunstruct a matrix off of
  /// a c pointer
  pub fn new_from_raw(raw : *mut Matf32Raw) -> Mat {
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
}

// This needs to be for base type specific functions
impl Mat {
  fn fail_at_bad_index(&self, index : &(uint, uint)) {
    let (r, c) = self.shape;
    let &(row, col) = index;
    if row >= r || col >= c {
      fail!(format!("Index out of bounds - shape: {} bad index: {}", self.shape, index));
    }
  }

  /// Index the matrix by row and column
  ///
  /// # Failure
  ///
  /// Fails if index is invalid.
  pub fn at(&self, index : (uint, uint)) -> f32 {
    self.fail_at_bad_index(&index);
    let (row, col) = index;
    unsafe {
      arma_Mat_f32_at(self.raw, row as c_uint, col as c_uint) as f32
    }
  }

  /// Index the matrix by row and column returning a mutable reference
  ///
  /// # Failure
  ///
  /// Fails if index is invalid.
  pub fn at_mut(&mut self, index : (uint, uint)) -> &mut f32 {
    self.fail_at_bad_index(&index);
    let (row, col) = index;
    unsafe {
      let ptr = arma_Mat_f32_at_ptr(self.raw, row as c_uint, col as c_uint) as *mut f32;
      mem::transmute(ptr)
    }
  }

  /// Get the data pointer
  pub unsafe fn data_ptr(&self) -> *mut f32 {
    arma_Mat_f32_data(self.raw)
  }
}

pub trait MatrixFuncs {
  /// Perform traditional matrix multiplication, or a bunch of dot products.
  ///
  /// # Failure
  ///
  /// Fails if matrix sizes are incompatible
  fn dot(&self, mat : &Self) -> Self;
}

impl MatrixFuncs for Mat {
  /// # Failure
  fn dot(&self, mat : &Mat) -> Mat{
    let (_, ca) = self.shape;
    let (rb, _) = mat.shape;
    if ca != rb {
        fail!(format!("Cannot multiply matrices of shape: {} and {}", self.shape, mat.shape));
    }
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_dot_Mat_f32(self.raw, mat.raw))
    }
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
      let string = String::from_str(CString::new(c_str, false).as_str().unwrap());
      write!(formatter, "Mat{{shape:({}, {})\n {}\n}}", 0i, 0i, string)
    }
  }
}

impl cmp::PartialEq for Mat {
  fn eq(&self, other: &Mat) -> bool {
    if self.shape != other.shape {
      fail!(format!("Invalid equality check. Matrix different shapes: {} vs {}", self.shape, other.shape));
    }
    unsafe {
      arma_Mat_f32_eq(self.raw, other.raw) != 0
    }
  }
}

#[cfg(test)]
mod test_mat {
  use super::{Mat};

  #[test]
  fn zeros() {
    let zeros = Mat::zeros(10, 20);
    assert_eq!(0f32, zeros.at((0,0)));
    assert_eq!(0f32, zeros.at((9,15)));
  }

  #[test]
  fn ones() {
    let ones = Mat::ones(10, 20);
    assert_eq!(1f32, ones.at((0,0)));
    assert_eq!(1f32, ones.at((9,15)));
  }

  #[test]
  fn eye() {
    let eye = Mat::eye(10, 10);
    assert_eq!(1f32, eye.at((0,0)));
    assert_eq!(1f32, eye.at((3,3)));
    assert_eq!(0f32, eye.at((9,3)));
  }

  #[test]
  fn rands() {
    let randu = Mat::randu(10, 10);
    let randn = Mat::randn(10, 10);
    // Spot check, no understanding of distribution
    assert!(randn.at((0,0)) != randu.at((0,0)));
    assert!(randu.at((2,1)) != randu.at((2,0)));
    assert!(randn.at((2,1)) != randn.at((2,0)));
    assert!(randn.at((3,2)) != randn.at((7,3)));
    assert!(randu.at((3,2)) != randu.at((7,3)));
  }
}

#[cfg(test)]
mod test_mat_funcs {
  use super::{Mat, MatrixFuncs};
  use std::mem;

  #[test]
  fn mat_dot() {
    let eye = Mat::eye(3, 5) * 3.0f32;
    let other = Mat::ones(5, 1);
    let result = eye.dot(&other);
    assert_eq!(result.shape, (3u, 1u));
    assert_eq!(result.at((0,0)), 3.0f32);
  }

  #[test]
  #[should_fail]
  fn mat_dot_invalid_sizes() {
    let eye = Mat::eye(3, 5) * 3.0f32;
    let other = Mat::ones(6, 1);
    let _ = eye.dot(&other);
  }

  #[test]
  #[should_fail]
  fn mat_at_col_out_of_bounds() {
    let mat = Mat::ones(4, 9);
    let _ = mat.at((2, 10));
  }

  #[test]
  #[should_fail]
  fn mat_at_row_out_of_bounds() {
    let mat = Mat::ones(4, 9);
    let _ = mat.at((7, 2));
  }

  #[test]
  fn mat_at_mut() {
    let mut mat = Mat::ones(4, 9);
    assert_eq!(mat.at((1, 2)), 1f32);
    assert_eq!(mat.at((2, 2)), 1f32);
    *mat.at_mut((1, 2)) = 2f32;
    *mat.at_mut((2, 2)) = 3f32;
    assert_eq!(mat.at((1, 2)), 2f32);
    assert_eq!(mat.at((2, 2)), 3f32);
  }

  #[test]
  fn mat_data_ptr_smoke() {
    let mut mat = Mat::ones(4, 9);
    unsafe {
      let data = mat.data_ptr();
      assert!(data != mem::transmute(0i));
    }
  }
}

#[cfg(test)]
mod test_mat_cmp {
  use super::{Mat};

  #[test]
  fn mat_eq_mat() {
    let mat = Mat::ones(4,4);
    let mat2 = Mat::ones(4,4);
    assert_eq!(mat, mat2);
  }

  #[test]
  fn mat_neq_mat() {
    let mat = Mat::ones(4,4);
    let mat2 = Mat::ones(4,4)+1f32;
    assert!(mat != mat2);
  }

  #[test]
  #[should_fail]
  fn mat_eq_wrong_size() {
    let mat = Mat::ones(4,4);
    let mat2 = Mat::ones(4,5);
    assert_eq!(mat, mat2);
  }
}
