#ifndef SHADER_LIB_MATH_GLSL_INCLUDED
#define SHADER_LIB_MATH_GLSL_INCLUDED

float isotropic_gaussian(float x, float y, float sigma) {
    float sigma22 = 2.0 * sigma * sigma;

    float e = ((x * x) + (y * y)) / sigma22;

    return exp(-e) / (PI * sigma22);
}

#endif //SHADER_LIB_MATH_GLSL_INCLUDED