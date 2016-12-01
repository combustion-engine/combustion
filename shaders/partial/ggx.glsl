#ifndef SHADER_PARTIAL_GGX_GLSL_INCLUDED
#define SHADER_PARTIAL_GGX_GLSL_INCLUDED

#include "../lib/constants.glsl"
#include "../lib/utils.glsl"

float ggx_distribution(float sigma2, float NdotH2) {
    float denom = NdotH2 * (sigma2 - 1.0) + 1.0;

    return sigma2 / (PI * denom * denom);
}

float gtr_distribution(float sigma2, float NdotH, float gamma) {
    float N = chiX(NdotH) * (gamma - 1.0) * (sigma2 - 1.0);

    float D1 = PI * (1.0 - pow(sigma2, 1.0 - gamma));
    float D2 = (NdotH * NdotH) * (sigma2 - 1.0) + 1.0;

    return N / (D1 * pow(D2, gamma));
}

//http://learnopengl.com/#!PBR/Theory
//float ggx_schlick_distribution(float roughness, float NdotV)

float ggx_anisotropic_distribution(float sigma, float NdotH2, vec3 H, vec3 X, vec3 Y, float ax, float ay) {
    float sax = sigma * ax;
    float say = sigma * ay;

    float A = 1.0 / (PI * sax * say);

    float XdotH = dot(X, H);
    float YdotH = dot(Y, H);

    float B1 = (XdotH * XdotH) / (sax * sax);
    float B2 = (YdotH * YdotH) / (say * say);

    float B = B1 + B2 + NdotH2;

    return (A * (1.0 / (B * B)));
}

float ggx_geo_attenuation(float sigma2, float cosAlpha) {
    return (chiX(cosAlpha) * 2.0 * cosAlpha) /
           (cosAlpha + sqrt(sigma2 + ((1.0 - sigma2) * cosAlpha * cosAlpha)));
}

//https://hal.inria.fr/hal-00942452v1/document
float ggx_smith_lambda(float roughness, float cosAlpha) {
    float alpha = 1.0 / (roughness * tan(acos(cosAlpha)));

    return (sqrt(1.0 + 1.0 / (alpha * alpha)) - 1.0) * 0.5;
}

#endif //SHADER_PARTIAL_GGX_GLSL_INCLUDED