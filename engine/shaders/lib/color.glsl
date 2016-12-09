#ifndef SHADER_LIB_COLOR_GLSL_INCLUDED
#define SHADER_LIB_COLOR_GLSL_INCLUDED

float contrast(float value, float c) {
    return ((value - 0.5) * max(c, 0.0)) + 0.5;
}

vec2 contrast(vec2 value, float c) {
    return ((value - 0.5) * max(c, 0.0)) + 0.5;
}

vec3 contrast(vec3 value, float c) {
    return ((value - 0.5) * max(c, 0.0)) + 0.5;
}

vec4 contrast(vec4 value, float c) {
    return ((value - 0.5) * max(c, 0.0)) + 0.5;
}

#endif //SHADER_LIB_COLOR_GLSL_INCLUDED