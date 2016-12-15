#include <iostream>
#include <igl/cotmatrix.h>
#include <Eigen/Dense>
#include <Eigen/Sparse>

using namespace std;

extern "C" {
int test() {
    cout << "Running test function from C/C++" << endl;

    Eigen::MatrixXd V( 4, 2 );
    V << 0, 0,
        1, 0,
        1, 1,
        0, 1;
    Eigen::MatrixXi F( 2, 3 );
    F << 0, 1, 2,
        0, 2, 3;
    Eigen::SparseMatrix<double> L;
    igl::cotmatrix( V, F, L );
    std::cout << "Hello, mesh: " << std::endl << L * V << std::endl;
    return 10;
}
}
