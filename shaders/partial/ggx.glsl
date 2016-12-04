#ifndef SHADER_PARTIAL_GGX_GLSL_INCLUDED
#define SHADER_PARTIAL_GGX_GLSL_INCLUDED

#include "../lib/constants.glsl"
#include "../lib/utils.glsl"

float ggx_distribution(float sigma2, float NdotM2) {
    float D = (NdotM2 * (sigma2 - 1.0)) + 1.0;

    return sigma2 / (PI * D * D);
}

float ggx_distribution_gamma(float sigma2, float NdotM, float gamma) {
    float N = chiX(NdotM) * (gamma - 1.0) * (sigma2 - 1.0);

    float D1 = PI * (1.0 - pow(sigma2, 1.0 - gamma));
    float D2 = (NdotM * NdotM) * (sigma2 - 1.0) + 1.0;

    return N / (D1 * pow(D2, gamma));
}

//http://learnopengl.com/#!PBR/Theory
//float ggx_schlick_distribution(float roughness, float NdotV)

float ggx_anisotropic_distribution(float sigma, float NdotM2, vec3 M, vec3 X, vec3 Y, float ax, float ay) {
    float sax = sigma * ax;
    float say = sigma * ay;

    float A = 1.0 / (PI * sax * say);

    float XdotM = dot(X, M);
    float YdotM = dot(Y, M);

    float B1 = (XdotM * XdotM) / (sax * sax);
    float B2 = (YdotM * YdotM) / (say * say);

    float B = B1 + B2 + NdotM2;

    return (A * (1.0 / (B * B)));
}

float ggx_geo_attenuation(float sigma2, float NdotM) {
    return (chiX(NdotM) * 2.0 * NdotM) /
           (NdotM + sqrt(sigma2 + ((1.0 - sigma2) * NdotM * NdotM)));
}

//https://hal.inria.fr/hal-00942452v1/document
float ggx_smith_lambda(float roughness, float cosAlpha) {
    float alpha = 1.0 / (roughness * tan(acos(cosAlpha)));

    return (sqrt(1.0 + 1.0 / (alpha * alpha)) - 1.0) * 0.5;
}

#endif //SHADER_PARTIAL_GGX_GLSL_INCLUDED