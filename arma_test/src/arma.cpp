#include <armadillo>

extern "C" struct ARM_Wrapper {
    double *data;
    unsigned long row;
    unsigned long col;

    ARM_Wrapper(arma::mat &m) {
        data = m.memptr();
        row = m.n_rows;
        col = m.n_cols;
    };

    arma::mat to_mat() {
        return arma::mat(data, row, col);
    }
};

extern "C" {
    double det(ARM_Wrapper* m) {
        return arma::det(m->to_mat());
    }
}
