#ifndef SHADER_LIB_SMOOTHING_GLSL_INCLUDED
#define SHADER_LIB_SMOOTHING_GLSL_INCLUDED

#include "constants.glsl"

float scale(float a, float b, float x) {
    return (x - a) / (b - a);
}

float smooth_cos_clamped2(float edge0, float edge1, float x) {
    return (1.0 - cos((1.0 - clamp(scale(edge0, edge1, x), 0.0, 1.0)) * PI)) * 0.5;
}

float smooth_sin(float edge0, float edge1, float x) {
    return sin(scale(edge0, edge1, x) * FRAC_PI_2);
}

float smooth_sin_clamped(float edge0, float edge1, float x) {
    return sin(clamp(scale(edge0, edge1, x), 0.0, 1.0) * FRAC_PI_2);
}

float smooth_cos(float edge0, float edge1, float x) {
    return 1.0 - cos(scale(edge0, edge1, x) * FRAC_PI_2);
}

float smooth_cos_clamped(float edge0, float edge1, float x) {
    return 1.0 - cos(clamp(scale(edge0, edge1, x), 0.0, 1.0) * FRAC_PI_2);
}

float smootherstep(float edge0, float edge1, float x) {
    x = clamp(scale(edge0, edge1, x), 0.0, 1.0);

    return x * x * x * (x * (x * 6 - 15) + 10);
}

#endif //SHADER_LIB_SMOOTHING_GLSL_INCLUDED