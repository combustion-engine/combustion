#ifndef SHADER_LIB_UTILS_GLSL_INCLUDED
#define SHADER_LIB_UTILS_GLSL_INCLUDED

vec4 unpack_channels(vec4 values) {
    return (2.0 * values) - 1.0;
}

vec4 pack_channels(vec4 values) {
    return (values + 1.0) * 0.5;
}

vec3 unpack_channels(vec3 values) {
    return (2.0 * values) - 1.0;
}

vec3 pack_channels(vec3 values) {
    return (values + 1.0) * 0.5;
}

float sin_integral(float a, float b) {
    return -cos(b) + cos(a);
}

float cos_integral(float a, float b) {
    return sin(b) - sin(a);
}

float chiX(float value) {
    return value > 0.0 ? 1.0 : 0.0;
}

#endif //SHADER_LIB_UTILS_GLSL_INCLUDED