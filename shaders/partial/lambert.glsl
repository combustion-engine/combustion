#ifndef SHADER_PARTIAL_LAMBERT_GLSL_INCLUDED
#define SHADER_PARTIAL_LAMBERT_GLSL_INCLUDED

#include "../lib/constants.glsl"

#include "fresnel.glsl"

float lambert(float cosTheta, float albedo, float metallic) {
    return albedo * cosTheta * (1.0 - metallic) * FRAC_1_PI;
}

#endif //SHADER_PARTIAL_LAMBERT_GLSL_INCLUDED
