#ifndef SHADER_LIB_GAMMA_GLSL_INCLUDED
#define SHADER_LIB_GAMMA_GLSL_INCLUDED

vec4 gamma_encode(vec4 color, float gamma) {
    return pow(color, vec4(1.0 / gamma));
}

vec4 gamma_decode(vec4 color, float gamma) {
    return pow(color, vec4(gamma));
}

vec3 gamma_encode(vec3 color, float gamma) {
    return pow(color, vec3(1.0 / gamma));
}

vec3 gamma_decode(vec3 color, float gamma) {
    return pow(color, vec3(gamma));
}


#endif //SHADER_LIB_GAMMA_GLSL_INCLUDED