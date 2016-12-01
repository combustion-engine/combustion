#ifndef SHADER_LIB_GAMMA_GLSL_INCLUDED
#define SHADER_LIB_GAMMA_GLSL_INCLUDED

vec4 gamma_encode(vec4 color, float gamma) {
    return pow(color, vec4(1.0 / gamma));
}

vec4 gamma_decode(vec4 color, float gamma) {
    return pow(color, vec4(gamma));
}

#endif //SHADER_LIB_GAMMA_GLSL_INCLUDED