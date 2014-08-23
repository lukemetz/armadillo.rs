use ffi::*;
use mat::Mat;

/// Trait to enable operation overloading with Mat
pub trait OpsWithMatrix{
  fn do_add(&self, mat : &Mat) -> Mat;
  fn do_sub(&self, mat : &Mat) -> Mat;
  fn do_mul(&self, mat : &Mat) -> Mat;
  fn do_div(&self, mat : &Mat) -> Mat;
}

impl OpsWithMatrix for f32 {
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

impl OpsWithMatrix for Mat {
  fn do_add(&self, mat : &Mat) -> Mat{
    if self.shape != mat.shape {
      fail!(format!("Invalid matrix shapes: {}, {}", self.shape, mat.shape));
    }
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_add_Mat_f32(mat.raw, self.raw))
    }
  }

  fn do_sub(&self, mat : &Mat) -> Mat{
    if self.shape != mat.shape {
      fail!(format!("Invalid matrix shapes: {}, {}", self.shape, mat.shape));
    }
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_sub_Mat_f32(mat.raw, self.raw))
    }
  }

  fn do_mul(&self, mat : &Mat) -> Mat{
    if self.shape != mat.shape {
      fail!(format!("Invalid matrix shapes: {}, {}", self.shape, mat.shape));
    }
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_mul_Mat_f32(mat.raw, self.raw))
    }
  }

  fn do_div(&self, mat : &Mat) -> Mat{
    if self.shape != mat.shape {
      fail!(format!("Invalid matrix shapes: {}, {}", self.shape, mat.shape));
    }
    unsafe {
      Mat::new_from_raw(arma_Mat_f32_div_Mat_f32(mat.raw, self.raw))
    }
  }
}

impl<T : OpsWithMatrix> Add<T, Mat> for Mat {
  fn add(&self, rhs : &T) -> Mat {
    rhs.do_add(self)
  }
}

impl<T : OpsWithMatrix> Sub<T, Mat> for Mat {
  fn sub(&self, rhs : &T) -> Mat {
    rhs.do_sub(self)
  }
}

impl<T : OpsWithMatrix> Mul<T, Mat> for Mat {
  fn mul(&self, rhs : &T) -> Mat {
    rhs.do_mul(self)
  }
}

impl<T : OpsWithMatrix> Div<T, Mat> for Mat {
  fn div(&self, rhs : &T) -> Mat {
    rhs.do_div(self)
  }
}

#[cfg(test)]
mod test_ops_with_Mat {
  use mat::Mat;

  #[test]
  fn mat_add_f32() {
    let ones = Mat::ones(10, 20);
    let new = ones + 1f32;
    assert_eq!(2f32, new.at((9,15)));
    let second = new + 1f32;
    assert_eq!(3f32, second.at((9,15)));
    assert_eq!(2f32, new.at((9,15)));
  }

  #[test]
  fn mat_sub_f32() {
    let ones = Mat::ones(10, 20);
    let new = ones - 1f32;
    assert_eq!(0f32, new.at((9,9)));
  }

  #[test]
  fn mat_div_f32() {
    let ones = Mat::ones(10, 20);
    let new = ones / 4f32;
    assert_eq!(0.25f32, new.at((9,9)));
  }

  #[test]
  fn mat_mul_f32() {
    let ones = Mat::ones(10, 20);
    let new = ones * 4f32;
    assert_eq!(4f32, new.at((9,9)));
  }

  #[test]
  fn mat_add_mat() {
    let ones = Mat::ones(10, 20);
    let other = Mat::ones(10, 20);
    let new = ones + other;
    assert_eq!(2f32, new.at((9,9)));
  }

  #[test]
  #[should_fail]
  fn mat_add_mat_invalid_shapes() {
    let ones = Mat::ones(10, 20);
    let other = Mat::ones(11, 20);
    let new = ones + other;
    assert_eq!(2f32, new.at((9,9)));
  }

  #[test]
  fn mat_sub_mat() {
    let ones = Mat::ones(10, 10);
    let other = Mat::ones(10, 10);
    let new = ones - other;
    assert_eq!(0f32, new.at((9,9)));
  }

  #[test]
  fn mat_div_mat() {
    let ones = Mat::ones(10, 10)+1f32;
    let other = Mat::ones(10, 10)+3f32;
    let new = ones / other;
    assert_eq!(0.5f32, new.at((9,4)));
  }

  #[test]
  fn mat_mul_mat() {
    let ones = Mat::ones(10, 10) + 2f32;
    let other = Mat::ones(10, 10) + 1f32;
    let new = ones * other;
    assert_eq!(6f32, new.at((9,4)));
  }
}
