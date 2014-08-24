extern crate libc;

use libc::{c_uint, c_char, c_float};

#[repr(C)]
pub struct Matf32Raw;

#[link(name = "carmadillo", kind="static")]
#[link(name = "armadillo")]
#[link(name = "stdc++")]
extern {
    pub fn arma_Mat_f32_zeros(r : c_uint, c : c_uint) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_ones(r : c_uint, c : c_uint) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_randu(r : c_uint, c : c_uint) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_eye(r : c_uint, c : c_uint) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_randn(r : c_uint, c : c_uint) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_add_Mat_f32(m : *mut Matf32Raw, n : *mut Matf32Raw) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_sub_Mat_f32(m : *mut Matf32Raw, n : *mut Matf32Raw) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_mul_Mat_f32(m : *mut Matf32Raw, n : *mut Matf32Raw) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_div_Mat_f32(m : *mut Matf32Raw, n : *mut Matf32Raw) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_dot_Mat_f32(m : *mut Matf32Raw, n : *mut Matf32Raw) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_add_f32(m : *mut Matf32Raw, n : c_float) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_sub_f32(m : *mut Matf32Raw, n : c_float) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_mul_f32(m : *mut Matf32Raw, n : c_float) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_div_f32(m : *mut Matf32Raw, n : c_float) -> *mut Matf32Raw;
    pub fn arma_Mat_f32_print(m : *mut Matf32Raw) -> *const c_char;
    pub fn arma_Mat_f32_free(m : *mut Matf32Raw);
    pub fn arma_Mat_f32_at(m : *mut Matf32Raw, r : c_uint, c : c_uint) -> c_float;
    pub fn arma_Mat_f32_at_ptr(m : *mut Matf32Raw, r : c_uint, c : c_uint) -> *mut c_float;
    pub fn arma_Mat_f32_n_rows(m : *mut Matf32Raw) -> c_uint;
    pub fn arma_Mat_f32_n_cols(m : *mut Matf32Raw) -> c_uint;
    pub fn arma_Mat_f32_data(m : *mut Matf32Raw) -> *mut c_float;
    pub fn arma_Mat_f32_eq(n : *mut Matf32Raw, m : *mut Matf32Raw) -> c_uint;
}
