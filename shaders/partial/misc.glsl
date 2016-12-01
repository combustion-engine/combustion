#ifndef SHADER_PARTIAL_MISC_GLSL_INCLUDED
#define SHADER_PARTIAL_MISC_GLSL_INCLUDED

float schlick_beckmann_geo_attenuation(float sigma, float NdotX) {
    float k = sigma * FRAC_2_PI_SQRT;

    return NdotX / (NdotX * (1.0 - k) + k);
}

#endif //SHADER_PARTIAL_MISC_GLSL_INCLUDED