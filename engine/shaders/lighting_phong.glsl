#ifndef SHADER_LIGHTING_PHONG_GLSL_INCLUDED
#define SHADER_LIGHTING_PHONG_GLSL_INCLUDED

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

#include "partial/phong.glsl"
#include "partial/lambert.glsl"

vec4 calc_lighting_phong(vec3 color,
                         vec3 position,
                         vec3 normal,
                         vec3 view_position,
                         float albedo,
                         float metallic) {

    vec4 linear_color = vec4(0.0, 0.0, 0.0, 1.0);
    vec4 diffuse_color = vec4(0.0, 0.0, 0.0, 1.0);
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
                vec3 view_direction = normalize(view_position - position);

                float diffuse = lambert(NdotL, albedo, metallic);
                float specular = 10.0 * phong(light_direction, view_direction, normal, 48);

                diffuse_color.rgb += (diffuse * (1.0 - specular)) * light.color.rgb * attenuation;
                specular_color.rgb += specular * light.color.rgb * attenuation;
            }
        }
    }

    //Merge diffuse and specular components with object tint
    linear_color.rgb += (diffuse_color + specular_color).rgb * color.rgb;
    //linear_color.rgb += blended_specular_color;

    //Map HDR into linear space
    linear_color.rgb = ACESFilm_tonemap_exposure(linear_color.rgb, 2.0);

    return linear_color;
}

#endif //SHADER_LIGHTING_PHONG_GLSL_INCLUDED