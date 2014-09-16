#include <armadillo>
#include <stdlib.h>

extern "C" struct Mat_f32 {
  float *data;
  unsigned int n_cols;
  unsigned int n_rows;

  Mat_f32(arma::Mat<float> &m) {
    data = m.memptr();
    n_cols = m.n_cols;
    n_rows = m.n_rows;
  };
};

extern "C" {

  Mat_f32 * arma_Mat_f32_zeros(unsigned int r, unsigned int c) {
    float * data = static_cast<float*>(malloc(sizeof(float) * r * c));
    arma::Mat<float> m(data, r, c, false);
    m.zeros();
    return new Mat_f32(m);
  }

  Mat_f32 * arma_Mat_f32_ones(unsigned int r, unsigned int c) {
    float * data = static_cast<float*>(malloc(sizeof(float) * r * c));
    arma::Mat<float> m(data, r, c, false);
    m.ones();
    return new Mat_f32(m);
  }

  Mat_f32 * arma_Mat_f32_randu(unsigned int r, unsigned int c) {
    float * data = static_cast<float*>(malloc(sizeof(float) * r * c));
    arma::Mat<float> m(data, r, c, false);
    m.randu();
    return new Mat_f32(m);
  }

  Mat_f32 * arma_Mat_f32_eye(unsigned int r, unsigned int c) {
    float * data = static_cast<float*>(malloc(sizeof(float) * r * c));
    arma::Mat<float> m(data, r, c, false);
    m.eye();
    return new Mat_f32(m);
  }

  Mat_f32 * arma_Mat_f32_randn(unsigned int r, unsigned int c) {
    float * data = static_cast<float*>(malloc(sizeof(float) * r * c));
    arma::Mat<float> m(data, r, c, false);
    m.randn();
    return new Mat_f32(m);
  }

  Mat_f32 * arma_Mat_f32_make_raw(unsigned int r, unsigned int c, float* data) {
    arma::Mat<float> m(data, r, c, false);
    return new Mat_f32(m);
  }

  Mat_f32 * arma_Mat_f32_add_Mat_f32(Mat_f32 * m, Mat_f32 * n) {
    arma::Mat<float> m_mat(m->data, m->n_rows, m->n_cols, false);
    arma::Mat<float> n_mat(n->data, n->n_rows, n->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * n->n_cols));
    arma::Mat<float> res(data, m->n_rows, n->n_cols, false);
    res = m_mat + n_mat;
    return new Mat_f32(res);
  }

  Mat_f32 * arma_Mat_f32_sub_Mat_f32(Mat_f32 * m, Mat_f32 * n) {
    arma::Mat<float> m_mat(m->data, m->n_rows, m->n_cols, false);
    arma::Mat<float> n_mat(n->data, n->n_rows, n->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * n->n_cols));
    arma::Mat<float> res(data, m->n_rows, n->n_cols, false);
    res = m_mat - n_mat;
    return new Mat_f32(res);
  }

  Mat_f32 * arma_Mat_f32_mul_Mat_f32(Mat_f32 * m, Mat_f32 * n) {
    arma::Mat<float> m_mat(m->data, m->n_rows, m->n_cols, false);
    arma::Mat<float> n_mat(n->data, n->n_rows, n->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * n->n_cols));
    arma::Mat<float> res(data, m->n_rows, n->n_cols, false);
    res = m_mat % n_mat;
    return new Mat_f32(res);
  }

  Mat_f32 * arma_Mat_f32_div_Mat_f32(Mat_f32 * m, Mat_f32 * n) {
    arma::Mat<float> m_mat(m->data, m->n_rows, m->n_cols, false);
    arma::Mat<float> n_mat(n->data, n->n_rows, n->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * n->n_cols));
    arma::Mat<float> res(data, m->n_rows, n->n_cols, false);
    res = m_mat / n_mat;
    return new Mat_f32(res);
  }

  Mat_f32 * arma_Mat_f32_dot_Mat_f32(Mat_f32 * m, Mat_f32 * n) {
    arma::Mat<float> m_mat(m->data, m->n_rows, m->n_cols, false);
    arma::Mat<float> n_mat(n->data, n->n_rows, n->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * n->n_cols));
    arma::Mat<float> res(data, m->n_rows, n->n_cols, false);
    res = m_mat * n_mat;
    return new Mat_f32(res);
  }

  Mat_f32 * arma_Mat_f32_add_f32(Mat_f32 * m, float n) {
    arma::Mat<float> mat(m->data, m->n_rows, m->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * m->n_cols));
    arma::Mat<float> res(data, m->n_rows, m->n_cols, false);
    res = mat + n;
    return new Mat_f32(res);
  }

  Mat_f32 * arma_Mat_f32_sub_f32(Mat_f32 * m, float n) {
    arma::Mat<float> mat(m->data, m->n_rows, m->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * m->n_cols));
    arma::Mat<float> res(data, m->n_rows, m->n_cols, false);
    res = mat - n;
    return new Mat_f32(res);
  }

  Mat_f32 * arma_Mat_f32_mul_f32(Mat_f32 * m, float n) {
    arma::Mat<float> mat(m->data, m->n_rows, m->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * m->n_cols));
    arma::Mat<float> res(data, m->n_rows, m->n_cols, false);
    res = mat * n;
    return new Mat_f32(res);
  }

  Mat_f32 * arma_Mat_f32_div_f32(Mat_f32 * m, float n) {
    arma::Mat<float> mat(m->data, m->n_rows, m->n_cols, false);
    float * data = static_cast<float*>(malloc(sizeof(float) * m->n_rows * m->n_cols));
    arma::Mat<float> res(data, m->n_rows, m->n_cols, false);
    res = mat / n;
    return new Mat_f32(res);
  }

  const char * arma_Mat_f32_print(Mat_f32 * m) {
    arma::Mat<float> mat(m->data, m->n_rows, m->n_cols, false);
    std::stringstream ss;
    ss << mat << '\0';
    std::string s = ss.str();
    char * out = new char[s.size() + 1];
    s.copy(out, s.size());
    s[s.size()] = '\0';
    return out;
  }

  void arma_Mat_f32_free(Mat_f32 * m) {
    delete m->data;
    delete m;
  }

  float arma_Mat_f32_at(Mat_f32 * m, unsigned int r, unsigned int c) {
    arma::Mat<float> mat(m->data, m->n_rows, m->n_cols, false);
    return mat.at(r, c);
  }

  float* arma_Mat_f32_at_ptr(Mat_f32 * m, unsigned int r, unsigned int c) {
    arma::Mat<float> mat(m->data, m->n_rows, m->n_cols, false);
    return &mat.at(r, c);
  }

  unsigned int arma_Mat_f32_n_rows(Mat_f32 * m) {
    return m->n_rows;
  }

  unsigned int arma_Mat_f32_n_cols(Mat_f32 * m) {
    return m->n_cols;
  }

  float * arma_Mat_f32_data(Mat_f32 * m) {
    return m->data;
  }

  unsigned int arma_Mat_f32_eq(Mat_f32 * n, Mat_f32 * m) {
    arma::Mat<float> matN(n->data, n->n_rows, n->n_cols, false);
    arma::Mat<float> matM(m->data, m->n_rows, m->n_cols, false);
    return arma::accu(arma::abs(matN - matM)) == 0;
  }
}
