// TODO Add in a high level heres how things work docs
// Split implementation up into a few traits
// MathWithMatrix / MatrixOps / OpsWithMatrix <-- really just ops
// Constructors (possibly odd construction syntax) let matrix : Matf32 = MatrixConstruction::zeros();
//      Maybe leave these on the impl
// "Decomposition"
// Slicing

#![feature(unsafe_destructor)]
#![feature(globs)]
#![feature(macro_rules)]
extern crate debug;
extern crate libc;

pub use mat::{Mat, MatrixFuncs};
pub use matrixMath::OpsWithMatrix;
pub use gen_ffi::Matf32Raw;

mod gen_ffi;
mod matrixMath;
mod mat;
