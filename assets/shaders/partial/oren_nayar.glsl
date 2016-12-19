#ifndef SHADER_PARTIAL_OREN_NAYAR_GLSL_INCLUDED
#define SHADER_PARTIAL_OREN_NAYAR_GLSL_INCLUDED

#include "../lib/constants.glsl"
#include "../lib/smoothing.glsl"

#include "fresnel.glsl"

float oren_nayar(float sigma2, float albedo, float metallic,
                float cosTheta, float cosPhi,
                float F0, vec3 v, vec3 l, vec3 n) {

    float Fdiff = 1.0 - fresnel_schlick(F0, cosTheta);

//http://blog.selfshadow.com/publications/s2013-shading-course/andersson/s2013_pbs_mia_notes.pdf
#ifdef SMOOTH_OREN_NAYAR_EDGE
    float edge = SMOOTH_OREN_NAYAR_EDGE;

    //cosTheta = mix(edge * smooth_cos(0.0, edge, cosTheta), cosTheta, step(edge, cosTheta));

    if(cosTheta <= edge) {
        //The original mia_material notes used smoothstep, but I like sinusoidal smoothing
        cosTheta = edge * smooth_cos(0.0, edge, cosTheta);
    }
#endif

    float diffuse_amount = 1.0;

    //Optimize for smooth objects or objects with no diffuse
    if(sigma2 > 0.0 && albedo > 0.0) {
        float theta, phi, gamma, A, B, C;

        theta = acos(cosTheta);
        phi   = acos(cosPhi);

        gamma = dot(v - n * cosPhi,
                    l - n * cosTheta);

        //The constants are part of the original equation
        A     = 1.0 - (0.50 * (sigma2 / (sigma2 + 0.57)));
        B     =        0.45 * (sigma2 / (sigma2 + 0.09));

        C     = sin(max(phi, theta)) * tan(min(phi, theta));

        diffuse_amount = (A + B * max(0.0, gamma) * C);
    }

    return Fdiff * (diffuse_amount * albedo * cosTheta * (1.0 - metallic) * FRAC_1_PI);
}

#endif //SHADER_PARTIAL_OREN_NAYAR_GLSL_INCLUDED
