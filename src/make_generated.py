
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
  Mat_{t[0]}(arma::Mat<{t[1]}> m) : m(m) {{}};
  arma::Mat<{t[1]}> m;
}};
"""

fill_kinds = ["zeros", "ones", "randu", "eye", "randn"]
fill_template = """
  Mat_{t[0]} * arma_Mat_{t[0]}_{fill}(unsigned int r, unsigned int c) {{
    return new Mat_{t[0]}(arma::Mat<{t[1]}>(r, c, arma::fill::{fill}));
  }}
"""
rust_fill_template = """
    pub fn arma_Mat_{t[0]}_{fill}(r : c_uint, c : c_uint) -> {struct};"""

op_kinds = [("add", "+"), ("sub", "-"), ("mul", "*"), ("div", "/")]
op_self_template = """
  Mat_{t[0]} * arma_Mat_{t[0]}_{op[0]}_Mat_{t[0]}(Mat_{t[0]} * m, Mat_{t[0]} * n) {{
    return new Mat_{t[0]}(m->m {op[1]} n->m);
  }}
"""
rust_op_self_template = """
    pub fn arma_Mat_{t[0]}_{op[0]}_Mat_{t[0]}(m : {struct}, n : {struct}) -> {struct};"""

op_scalar_template = """
  Mat_{t[0]} * arma_Mat_{t[0]}_{op[0]}_{t[0]}(Mat_{t[0]} * m, {t[1]} n) {{
    return new Mat_{t[0]}(m->m {op[1]} n);
  }}
"""
rust_op_scalar_template = """
    pub fn arma_Mat_{t[0]}_{op[0]}_{t[0]}(m : {struct}, n : {ct}) -> {struct};"""

print_template = """
  const char * arma_Mat_{t[0]}_print(Mat_{t[0]} * m) {{
    std::stringstream ss;
    ss << m->m;
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
    delete m;
  }}
"""
rust_free_template = """
    pub fn arma_Mat_{t[0]}_free(m : {struct});"""

at_template = """
  {t[1]} arma_Mat_{t[0]}_at(Mat_{t[0]} * m, unsigned int r, unsigned int c) {{
    return m->m.at(r, c);
  }}
"""
rust_at_template = """
    pub fn arma_Mat_{t[0]}_at(m : {struct}, r : c_uint, c : c_uint) -> {ct};"""

members = ["n_rows", "n_cols"]
members_template = """
  unsigned int arma_Mat_{t[0]}_{member}(Mat_{t[0]} * m) {{
    return m->m.{member};
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

  for op in op_kinds:
    output += op_self_template.format(t = t, op = op)
    rust_out += rust_op_self_template.format(t=t, op = op, ct=t[2], struct=t[3])
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


