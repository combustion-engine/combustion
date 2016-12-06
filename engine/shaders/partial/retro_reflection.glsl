#ifndef SHADER_PARTIAL_RETRO_REFLECTION_GLSL_INCLUDED
#define SHADER_PARTIAL_RETRO_REFLECTION_GLSL_INCLUDED

#include "../lib/constants.glsl"

//http://blog.selfshadow.com/publications/s2015-shading-course/burley/s2015_pbs_disney_bsdf_notes.pdf
//https://disney-animation.s3.amazonaws.com/library/s2012_pbs_disney_brdf_notes_v2.pdf
//https://youtu.be/zs0oYjwjNEo?t=19m51s
float apply_retro_reflection(float diffuse, float roughness, float albedo, float metallic,
                             float NdotV, float NdotL, float VdotH) {
    float fl = (1.0 - pow(NdotL, 5.0));
    float fv = (1.0 - pow(NdotV, 5.0));

    float rr = 2.0 * roughness * VdotH * VdotH;

    float retro_reflection = albedo * (1.0 - metallic) * FRAC_1_PI * rr * (fl + fv + (fl * fv * (rr - 1)));

    return (diffuse * (1.0 - (fl * 0.5)) * (1.0 - (fv * 0.5))) + retro_reflection;
}

#endif //SHADER_PARTIAL_RETRO_REFLECTION_GLSL_INCLUDED