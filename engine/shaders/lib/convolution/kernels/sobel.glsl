#ifndef SHADER_LIB_CONVOLUTION_KERNELS_SOBEL_GLSL_INCLUDED
#define SHADER_LIB_CONVOLUTION_KERNELS_SOBEL_GLSL_INCLUDED

const mat3 sobel3_h_kernel = mat3(
    -1.0, 0.0, 1.0,
    -2.0, 0.0, 2.0,
    -1.0, 0.0, 1.0
);

const mat3 sobel3_v_kernel = mat3(
     1.0,  2.0,  1.0,
     0.0,  0.0,  0.0,
    -1.0, -2.0, -1.0
);

const float sobel5_h_kernel[25] = float[25](
    1.0,  2.0,  0.0,  -2.0,  -1.0,
    4.0,  8.0,  0.0,  -8.0,  -4.0,
    6.0, 12.0,  0.0, -12.0,  -6.0,
    4.0,  8.0,  0.0,  -8.0,  -4.0,
    1.0,  2.0,  0.0,  -2.0,  -1.0
);

const float sobel5_v_kernel[25] = float[25](
    -1.0, -4.0,  -6.0, -4.0, -1.0,
    -2.0, -8.0, -12.0, -8.0, -2.0,
     0.0,  0.0,   0.0,  0.0,  0.0,
     2.0,  8.0,  12.0,  8.0,  2.0,
     1.0,  4.0,   6.0,  4.0,  1.0
);

#endif //SHADER_LIB_CONVOLUTION_KERNELS_SOBEL_GLSL_INCLUDED