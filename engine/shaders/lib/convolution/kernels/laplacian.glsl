#ifndef SHADER_LIB_CONVOLUTION_KERNELS_LAPLACIAN_GLSL_INCLUDED
#define SHADER_LIB_CONVOLUTION_KERNELS_LAPLACIAN_GLSL_INCLUDED

//http://fmwconcepts.com/imagemagick/laplacian/index.php

const mat3 laplacian3_kernel = mat3(
    0.0,  1.0, 0.0,
    1.0, -4.0, 1.0,
    0.0,  1.0, 0.0
);

const mat3 laplacian3_kernel_strong = mat3(
    -2.0, 1.0, -2.0,
     1.0, 4.0,  1.0,
    -2.0, 1.0, -2.0
);

const mat3 laplacian3_kernel_stronger = mat3(
    -2.0, 1.0, -2.0,
     1.0, 4.0,  1.0,
    -2.0, 1.0, -2.0
);

const float[25] laplacian5_kernel = float[25](
    -4.0, -1.0, 0.0, -1.0, -4.0,
    -1.0,  2.0, 3.0,  2.0, -1.0,
     0.0,  3.0, 4.0,  3.0,  0.0,
    -1.0,  2.0, 3.0,  2.0, -1.0,
    -4.0, -1.0, 0.0, -1.0, -4.0
);

const float[49] laplacian7_kernel = float[49](
    -10.0, -5.0, -2.0, -1.0, -2.0, -5.0, -10.0,
     -5.0,  0.0,  3.0,  4.0,  3.0,  0.0,  -5.0,
     -2.0,  3.0,  6.0,  7.0,  6.0,  3.0,  -2.0,
     -1.0,  4.0,  7.0,  8.0,  7.0,  4.0,  -1.0,
     -2.0,  3.0,  6.0,  7.0,  6.0,  3.0,  -2.0,
     -5.0,  0.0,  3.0,  4.0,  3.0,  0.0,  -5.0,
    -10.0, -5.0, -2.0, -1.0, -2.0, -5.0, -10.0
);

#endif //SHADER_LIB_CONVOLUTION_KERNELS_LAPLACIAN_GLSL_INCLUDED