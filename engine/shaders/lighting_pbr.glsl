#ifndef SHADER_LIGHTING_GLSL_INCLUDED
#define SHADER_LIGHTING_GLSL_INCLUDED

//Tell the Oren-Nayar algorithm to smooth out the lower 25% of the cosine
//This is based on a similar technique used in the "mia-material" engine
#ifndef SMOOTH_OREN_NAYAR_EDGE
    #define SMOOTH_OREN_NAYAR_EDGE 0.25
#endif

#ifndef UNCHARTED_2_TONEMAP_REFERENCE_ARCHIVE
    #define UNCHARTED_2_TONEMAP_REFERENCE_ARCHIVE
#endif

#include "lib/light.glsl"

#include "lib/constants.glsl"       //Things like PI, EPSILON, etc
#include "lib/utils.glsl"           //Various misc functions
#include "lib/depth.glsl"           //Calculate position from depth
#include "lib/smoothing.glsl"       //Smoothing/interpolation algorithms
#include "lib/attenuation.glsl"     //Lighting attenuation equations
#include "lib/gamma.glsl"           //Routines for gamma correction
#include "lib/hdr.glsl"             //Tone mapping equations

#include "external/PhotoshopMathFP.glsl"

#include "partial/fresnel.glsl"             //Fresnel equations
#include "partial/cook_torrance.glsl"       //Cook-Torrance functions
#include "partial/oren_nayar.glsl"          //Oren-Nayar diffuse lighting algorithm
#include "partial/retro_reflection.glsl"    //Diffuse retro-reflection
#include "partial/ggx.glsl"                 //GGX (Trowbridge-Reitz) BRDF equations

#include "partial/pixar.glsl"

vec4 calc_lighting_pbr(vec3 color,
                       vec3 position,
                       vec3 normal,
                       vec3 view_position,
                       float roughness,
                       float clearcoat,
                       float metallic,
                       float metallic_absorption,
                       float albedo,
                       float retro_reflection,
                       float ior,
                       float anisotropy,
                       float anisotropic_ratio) {

    vec4 linear_color = vec4(0.0, 0.0, 0.0, 1.0);
    vec4 diffuse_color = vec4(0.0, 0.0, 0.0, 1.0) + 0.01;
    vec4 specular_color = vec4(0.0, 0.0, 0.0, 1.0);

    for(int i = 0; i < MAX_LIGHTS; i++) {
        if(lights[i].disabled) {
            continue;
        }

        #define light lights[i]

        bool should_render = true;

        float light_distance;
        vec3  light_direction;

        if(light.kind == DIRECTIONAL_LIGHT) {
            //The negation is intentional, because the algorithms expect the opposite vector for the light direction.
            light_direction = -light.direction;

            /*Ignore light distance for directional lights. They are infinitely far away.*/

        } else {
            vec3 light_position = light.position;

            light_distance  = distance(light_position, position);
            light_direction = normalize(light_position - position);

            //Since point and spot lights are NOT infinitely far away, check if we should render them at all
            should_render   = light_distance > light.zdistance.x &&
                              light_distance < light.zdistance.y;
        }

        float NdotL = dot(normal, light_direction);

        if(should_render && NdotL > 0.0) {
            NdotL = max(NdotL, 0.0);

            //The attenuation is basically the resulting brightness of the light
            //depending on distance from the light and the light intensity
            float attenuation;

            //Ignore attenuation for infinite lights
            if(light.kind == DIRECTIONAL_LIGHT) {
                attenuation = light.intensity;

            } else {
                //Get the distance attenuation which applies to all sized lights
                attenuation = inverse_square_attenuation(light_distance, light.radius, light.intensity);

                //If the light is a spotlight, apply the attenuation from the cone angles
                if(light.kind == SPOT_LIGHT) {
                    float theta = acos(dot(normalize(-light.direction), light_direction));

                    if(theta > light.inner_cone) {
                        if(theta > light.outer_cone) {
                            attenuation = 0.0;

                        } else {
                            attenuation *= smooth_cos_clamped2(light.inner_cone, light.outer_cone, theta);
                        }
                    }

                    float multiplier = (TAU / (light.inner_cone + light.outer_cone)) - 1.0;

                    attenuation += attenuation * light.reflector_efficiency * multiplier;
                }
            }

            //Don't bother continuing if it won't be illuminated anyway
            if(attenuation >= ATTENUATION_THRESHOLD) {
                //Results
                vec3  specular = vec3(0.0);
                float diffuse = 0.0;

                //Vectors
                vec3 view_direction = normalize(view_position - position);
                vec3 half_direction = normalize(light_direction + view_direction);

                //Dot products, saturated
                float NdotH = dot(normal, half_direction);
                float NdotV = dot(normal, view_direction);
                float VdotH = dot(view_direction, half_direction);
                float VdotL = dot(view_direction, light_direction);

                //roughness squared
                float sigma2 = roughness * roughness;

                //Compute fresnel ratio term
                float F0 = fresnel0(ior);

                /// Oren-Nayar diffuse part plus retro-reflection

                diffuse = oren_nayar(sigma2, albedo, metallic, NdotL, NdotV, F0, view_direction, light_direction, normal);

                diffuse = apply_retro_reflection(diffuse, roughness, albedo * retro_reflection, metallic, NdotV, NdotL, VdotH);

                /// Modified Cook-Torrance specular part

                //Terms
                float D, G;
                vec3  F;

                //Geometric attenuation part using weighted anisotropic/isotropic attenuations
                float NdotH2 = NdotH * NdotH;

                D = ggx_distribution(sigma2, NdotH2);

                //Just use the purely dialectric fresnel if no conductivity is specified
                F = vec3(fresnel_schlick(F0, VdotH));

                //Smith form of the goemetric attenuations
                G = ggx_geo_attenuation(sigma2, NdotL) *
                    ggx_geo_attenuation(sigma2, NdotV);

                //Combine all the terms using the Cook-Torrance form
                specular =     (D * F * G) /
                          (VdotH * NdotL * 4.0);

                specular = vec3(1.0) * pixar_brdf(sigma2, NdotH, VdotH);

                diffuse_color.rgb += (diffuse * (1.0 - specular)) * light.color.rgb * attenuation;
                specular_color.rgb += specular * light.color.rgb * attenuation;
            }
        }
    }

    vec3 blended_specular_color = mix(BlendMultiply(Desaturate(color.rgb, 1.0 - clearcoat), specular_color.rgb),
                                      mix(specular_color.rgb, BlendHardLight(color.rgb, specular_color.rgb), roughness),
                                      clearcoat);

    //Merge diffuse and specular components with object tint
    linear_color.rgb += BlendMultiply(color.rgb, diffuse_color.rgb) + blended_specular_color;
    //linear_color.rgb += specular_color.rgb;

    return linear_color;
}


#endif //SHADER_LIGHTING_GLSL_INCLUDED