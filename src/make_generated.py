
header = """
#include <armadillo>
#include <stdlib.h>
"""
rust_header = """
extern crate libc;

use std::c_str::CString;
use libc::{c_uint, c_char, c_float, c_double};

pub struct Matf32Raw;
pub struct Mat64Raw;

#[link(name = "carmadillo")]//, kind = "static")]
#[link(name = "armadillo")]
"""

types = [("f32", "float", "c_float", "*mut Matf32Raw"), ("f64", "double", "c_double", "*mut Matf64Raw")]
types = [types[0]]
type_template = """
extern "C" struct Mat_{t[0]} {{
  {t[1]} *data;
  unsigned int n_cols;
  unsigned int n_rows;

  Mat_{t[0]}(arma::Mat<{t[1]}> &m) {{
    data = m.memptr();
    n_cols = m.n_cols;
    n_rows = m.n_rows;
  }};
}};
"""

fill_kinds = ["zeros", "ones", "randu", "eye", "randn"]
fill_template = """
  Mat_{t[0]} * arma_Mat_{t[0]}_{fill}(unsigned int r, unsigned int c) {{
    {t[1]} * data = static_cast<{t[1]}*>(malloc(sizeof({t[1]}) * r * c));
    arma::Mat<{t[1]}> m(data, r, c, false);
    m.{fill}();
    return new Mat_{t[0]}(m);
  }}
"""
rust_fill_template = """
    pub fn arma_Mat_{t[0]}_{fill}(r : c_uint, c : c_uint) -> {struct};"""

op_self_kinds = [("add", "+"), ("sub", "-"), ("mul", "%"), ("div", "/")]
op_self_template = """
  Mat_{t[0]} * arma_Mat_{t[0]}_{op[0]}_Mat_{t[0]}(Mat_{t[0]} * m, Mat_{t[0]} * n) {{
    arma::Mat<{t[1]}> m_mat(m->data, m->n_rows, m->n_cols, false);
    arma::Mat<{t[1]}> n_mat(n->data, n->n_rows, n->n_cols, false);
    {t[1]} * data = static_cast<{t[1]}*>(malloc(sizeof({t[1]}) * m->n_rows * m->n_cols));
    arma::Mat<{t[1]}> res(data, m->n_rows, m->n_cols, false);
    res = m_mat {op[1]} n_mat;
    return new Mat_{t[0]}(res);
  }}
"""
rust_op_self_template = """
    pub fn arma_Mat_{t[0]}_{op[0]}_Mat_{t[0]}(m : {struct}, n : {struct}) -> {struct};"""

op_scalar_kinds = [("add", "+"), ("sub", "-"), ("mul", "*"), ("div", "/")]
op_scalar_template = """
  Mat_{t[0]} * arma_Mat_{t[0]}_{op[0]}_{t[0]}(Mat_{t[0]} * m, {t[1]} n) {{
    arma::Mat<{t[1]}> mat(m->data, m->n_rows, m->n_cols, false);
    {t[1]} * data = static_cast<{t[1]}*>(malloc(sizeof({t[1]}) * m->n_rows * m->n_cols));
    arma::Mat<{t[1]}> res(data, m->n_rows, m->n_cols, false);
    res = mat {op[1]} n;
    return new Mat_{t[0]}(res);
  }}
"""
rust_op_scalar_template = """
    pub fn arma_Mat_{t[0]}_{op[0]}_{t[0]}(m : {struct}, n : {ct}) -> {struct};"""

print_template = """
  const char * arma_Mat_{t[0]}_print(Mat_{t[0]} * m) {{
    arma::Mat<{t[1]}> mat(m->data, m->n_rows, m->n_cols, false);
    std::stringstream ss;
    ss << mat;
    std::string s = ss.str();
    char * out = new char[s.size() + 1];
    s.copy(out, s.size());
    return out;
  }}
"""
rust_print_template = """
    pub fn arma_Mat_{t[0]}_print(m : {struct}) -> *const c_char;"""

free_template = """
  void arma_Mat_{t[0]}_free(Mat_{t[0]} * m) {{
    delete m->data;
    delete m;
  }}
"""
rust_free_template = """
    pub fn arma_Mat_{t[0]}_free(m : {struct});"""

at_template = """
  {t[1]} arma_Mat_{t[0]}_at(Mat_{t[0]} * m, unsigned int r, unsigned int c) {{
    arma::Mat<{t[1]}> mat(m->data, m->n_rows, m->n_cols, false);
    return mat.at(r, c);
  }}
"""
rust_at_template = """
    pub fn arma_Mat_{t[0]}_at(m : {struct}, r : c_uint, c : c_uint) -> {ct};"""

members = ["n_rows", "n_cols"]
members_template = """
  unsigned int arma_Mat_{t[0]}_{member}(Mat_{t[0]} * m) {{
    return m->{member};
  }}
"""

rust_members_template = """
    pub fn arma_Mat_{t[0]}_{member}(m : {struct}) -> c_uint;"""


output = "// ==== Auto generated! Do not edit! Modify gen_armadillo.cpp to remake====="
rust_out = output;
output += header
rust_out += rust_header;

for t in types:
  output += type_template.format(t = t)

rust_out += """
extern {"""

output += """
extern "C" {
"""

for t in types:
  for fill in fill_kinds:
    output += fill_template.format(t = t, fill = fill)
    rust_out += rust_fill_template.format(t=t, fill=fill, ct=t[2], struct=t[3])

  for op in op_self_kinds:
    output += op_self_template.format(t = t, op = op)
    rust_out += rust_op_self_template.format(t=t, op = op, ct=t[2], struct=t[3])
  for op in op_scalar_kinds:
    output += op_scalar_template.format(t = t, op = op)
    rust_out += rust_op_scalar_template.format(t=t, op=op, ct=t[2], struct=t[3])

  output += print_template.format(t=t)
  rust_out += rust_print_template.format(t=t, ct=t[2], struct=t[3])
  output += free_template.format(t=t)
  rust_out += rust_free_template.format(t=t, ct=t[2], struct=t[3])
  output += at_template.format(t=t)
  rust_out += rust_at_template.format(t=t, ct=t[2], struct=t[3])
  for member in members:
    output += members_template.format(t=t, member=member)
    rust_out += rust_members_template.format(t=t, member=member, ct=t[2], struct=t[3])

output += "}"
rust_out += "\n}"
open("gen_armadillo.cpp", "wr+").write(output)
open("gen_ffi.rs", "wr+").write(rust_out)


