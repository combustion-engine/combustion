#ifndef SHADER_PARTIAL_HYPER_CAUCHY_GLSL_INCLUDED
#define SHADER_PARTIAL_HYPER_CAUCHY_GLSL_INCLUDED

#include "../lib/constants.glsl"

float hyper_cauchy_distribution(float sigma2, float gamma, float cosTheta) {
    float theta = acos(cosTheta);
    float tanTheta = tan(theta);

    float N = (gamma - 1.0) * pow(SQRT_2, (2.0 * gamma) - 2.0);
    float D = PI * sigma2 * pow(cosTheta, 4.0) * pow(2.0 + ((tanTheta * tanTheta) / sigma2), gamma);

    return (N / D);
}

#endif //SHADER_PARTIAL_HYPER_CAUCHY_GLSL_INCLUDED