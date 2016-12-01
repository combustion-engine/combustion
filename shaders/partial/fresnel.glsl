#ifndef SHADER_PARTIAL_FRESNEL_GLSL_INCLUDED
#define SHADER_PARTIAL_FRESNEL_GLSL_INCLUDED

float fresnel0(float ior) {
    float F0 = (1.0 - ior) / (1.0 + ior);

    return F0 * F0;
}

float fresnel_schlick(float F0, float cosTheta) {
    return pow(1.0 - cosTheta, 5.0) * (1.0 - F0) + F0;
}

float fresnel_metallic(float eta, float cosTheta, float k) {
    float t1 = eta - 1.0;
    float t2 = eta + 1.0;

    float k2 = k * k;

    float A = (t1 * t1) + (4.0 * eta * pow(1.0 - cosTheta, 5.0)) + k2;
    float B = (t2 * t2) + k2;

    return A / B;
}

#endif //SHADER_PARTIAL_FRESNEL_GLSL_INCLUDED