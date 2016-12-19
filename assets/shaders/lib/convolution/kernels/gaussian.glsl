#ifndef SHADER_LIB_CONVOLUTION_KERNELS_GAUSSIAN_GLSL_INCLUDED
#define SHADER_LIB_CONVOLUTION_KERNELS_GAUSSIAN_GLSL_INCLUDED

#include "../../constants.glsl"
#include "../convolution.glsl"
#include "../../math.glsl"

/*
 * The default kernels all have a sigma of 1.0
 */

const mat3 gaussian3_kernel = mat3(
    1.0 / 16.0,  2.0 / 16.0,  1.0 / 16.0,
    2.0 / 16.0,  4.0 / 16.0,  2.0 / 16.0,
    1.0 / 16.0,  2.0 / 16.0,  1.0 / 16.0
);

const float[25] gaussian5_kernel = float[25](
   1.0 / 273.0,  4.0 / 273.0,  7.0 / 273.0,  4.0 / 273.0, 1.0 / 273.0,
   4.0 / 273.0, 16.0 / 273.0, 26.0 / 273.0, 16.0 / 273.0, 4.0 / 273.0,
   7.0 / 273.0, 26.0 / 273.0, 41.0 / 273.0, 26.0 / 273.0, 7.0 / 273.0,
   4.0 / 273.0, 16.0 / 273.0, 26.0 / 273.0, 16.0 / 273.0, 4.0 / 273.0,
   1.0 / 273.0,  4.0 / 273.0,  7.0 / 273.0,  4.0 / 273.0, 1.0 / 273.0
);

//http://dev.theomader.com/gaussian-kernel-calculator/
const float[49] gaussian7_kernel = float[49](
    0.000036, 0.000363, 0.001446, 0.002291, 0.001446, 0.000363, 0.000036,
    0.000363, 0.003676, 0.014662, 0.023226, 0.014662, 0.003676, 0.000363,
    0.001446, 0.014662, 0.058488, 0.092651, 0.058488, 0.014662, 0.001446,
    0.002291, 0.023226, 0.092651, 0.146768, 0.092651, 0.023226, 0.002291,
    0.001446, 0.014662, 0.058488, 0.092651, 0.058488, 0.014662, 0.001446,
    0.000363, 0.003676, 0.014662, 0.023226, 0.014662, 0.003676, 0.000363,
    0.000036, 0.000363, 0.001446, 0.002291, 0.001446, 0.000363, 0.000036
);

mat3 generate_gaussian3_kernel(float sigma) {
    mat3 result;

    for(int row = 0; row < 3; row++) {
        for(int col = 0; col < 3; col++) {
            result[col][row] = isotropic_gaussian(row - 1, col - 1, sigma);
        }
    }
#ifdef NORMALIZE_KERNELS
    return normalize_kernel3(result);
#else
    return result;
#endif
}

float[25] generate_gaussian5_kernel(float sigma) {
    float[25] result;

    for(int row = 0; row < 5; row++) {
        for(int col = 0; col < 5; col++) {
            result[row + col * 5] = isotropic_gaussian(row - 2, col - 2, sigma);
        }
    }
#ifdef NORMALIZE_KERNELS
    return normalize_kernel5(result);
#else
    return result;
#endif
}

float[49] generate_gaussian7_kernel(float sigma) {
    float[49] result;

    for(int row = 0; row < 7; row++) {
        for(int col = 0; col < 7; col++) {
            result[row + col * 7] = isotropic_gaussian(7 - row, 7 - col, sigma);
        }
    }
#ifdef NORMALIZE_KERNELS
    return normalize_kernel7(result);
#else
    return result;
#endif
}

#endif //SHADER_LIB_CONVOLUTION_KERNELS_GAUSSIAN_GLSL_INCLUDED