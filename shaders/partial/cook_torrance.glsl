#ifndef SHADER_PARTIAL_COOK_TORRANCE_GLSL_INCLUDED
#define SHADER_PARTIAL_COOK_TORRANCE_GLSL_INCLUDED

#include "../lib/utils.glsl"

float beckmann_distribution(float sigma2, float NdotH2) {
    float tan2Alpha = (NdotH2 - 1.0) / (NdotH2 * sigma2);

    return exp(tan2Alpha) / (PI * sigma2 * NdotH2 * NdotH2);
}

float geo_attenuation_cook_torrance(float NdotH, float VdotH, float NdotV, float NdotL) {
    float x = 2.0 * NdotH / VdotH;
    return max(0.0, min(1.0, min(x * NdotV, x * NdotL)));
}

//https://hal.inria.fr/hal-00942452v1/document
float smith_geo_attenuation(float roughness, float NdotX, float smith_lambda) {
    return chiX(NdotX) / (1.0 + smith_lambda);
}

#endif //SHADER_PARTIAL_COOK_TORRANCE_GLSL_INCLUDED