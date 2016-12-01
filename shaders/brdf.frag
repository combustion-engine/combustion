#version 330 core
#pragma optionNV (unroll all)

precision highp float;

/* Generic reference material used with this shader:
 * PLEASE NOTE that not all of them are correct, much to my annoyance,
 * and the correct equations and algorithms were taken from many sources.
 *
 *  https://en.wikipedia.org/wiki/Specular_highlight
 *  http://graphicrants.blogspot.com/2013/08/specular-brdf-reference.html
 *  http://simonstechblog.blogspot.com/2011/12/microfacet-brdf.html
 *  http://www.codinglabs.net/article_physically_based_rendering_cook_torrance.aspx
 *  https://hal.inria.fr/hal-00942452v1/document
 *  https://disney-animation.s3.amazonaws.com/library/s2012_pbs_disney_brdf_notes_v2.pdf
 *  http://www.gamedev.net/topic/638197-cook-torrance-brdf-general/
 *  http://blog.selfshadow.com/publications/s2013-shading-course/andersson/s2013_pbs_mia_notes.pdf
 *  https://en.wikipedia.org/wiki/Lambertian_reflectance
 *  https://en.wikipedia.org/wiki/Oren%E2%80%93Nayar_reflectance_model
 *  https://imdoingitwrong.wordpress.com/2011/01/31/light-attenuation/
 *  http://graphicrants.blogspot.jp/2013/12/tone-mapping.html
 *  http://filmicgames.com/archives/75
 *  https://github.com/cbaggers/filmic-tone-mapping-operators
 *  https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
 *  http://blog.selfshadow.com/publications/s2013-shading-course/
 *  http://blog.selfshadow.com/publications/s2014-shading-course/
 *  http://blog.selfshadow.com/2014/08/14/siggraph-2014-links/
 *  http://blog.selfshadow.com/publications/s2015-shading-course/
 *  http://blog.selfshadow.com/publications/s2016-shading-course/
 *  http://blog.selfshadow.com/publications/s2016-shading-course/hoffman/s2016_pbs_recent_advances_v2.pdf
 *  http://jcgt.org/published/0003/02/03/
 *  https://mouaif.wordpress.com/2009/01/05/photoshop-math-with-glsl-shaders/
 *  http://developer.amd.com/wordpress/media/2012/10/ShaderX_PerPixelAniso.pdf
 *
 * Videos/Presentations:
 *  https://www.youtube.com/watch?v=j-A0mwsJRmk
 *  https://youtu.be/zs0oYjwjNEo?t=14m45s
 *  http://www.gdcvault.com/play/1023519/Fast-Flexible-Physically-Based-Volumetric
 *  http://www.gdcvault.com/play/1012351/Uncharted-2-HDR
 */


//Tell the Oren-Nayar algorithm to smooth out the lower 25% of the cosine
//This is based on a similar technique used in the "mia-material" engine
#define SMOOTH_OREN_NAYAR_EDGE 0.25

#define UNCHARTED_2_TONEMAP_REFERENCE_ARCHIVE

#include "lib/constants.glsl"       //Things like PI, EPSILON, etc
#include "lib/utils.glsl"           //Various misc functions
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

#define DIRECTIONAL_LIGHT   1   //Like the Sun. All rays are parallel.
#define POINT_LIGHT         2   //Like a normal lightbulb
#define SPOT_LIGHT          3   //Like a flashlight

struct Light {
    vec2 zdistance;             //Minimum and maximum distances the light can touch. Hard limit. Ignored for Directional Lights
    vec3 position;              //Position of light in space. Ignored for Directional lights
    vec3 direction;             //Direction the light is pointing at. Used in Directional and Spot lights
    vec4 color;                 //Light color
    vec4 ambient;               //Ambient color
    int kind;                   //Directional, Point, Spot, etc
    float radius;               //Spherical radius of entire light. Soft limit. Ignored for Directional lights
    float inner_cone;           //Angle (in radians) of the inner spotlight cone
    float outer_cone;           //Angle (in radians) of the outer spotlight cone
    float reflector_efficiency; //Efficiency of the spotlight cone reflector (like the inside of a flashlight)
    float intensity;            //Light intensity
    bool disabled;              //Pretty obvious
};

struct MaterialModifiers {
    float ambient;      //ambient light intensity. Zero or near-zero is recommended
    float smoothness;   //material smoothness from 0 to 1, 1 being perfectly smooth.
    float albedo;       //diffuse absorbtion amount
    float ior;          //index of refraction
    float metallic;     //material conductivity
    vec4 tint;          //color to tint the object. Totally optional.
};

in vec3 Position;
in vec3 Normal;
in vec3 Tangent;
in vec3 BiTangent;
in vec3 UV;

out vec4 color;

//Texture toggles
uniform bool has_normal_map = false;
uniform bool has_texture    = true;
uniform bool srgb_texture   = true;

//Textures
uniform sampler2D texture_map;
uniform sampler2D normal_map;

//Material toggles
uniform bool has_material_a = false;
uniform bool has_material_b = false;

//Materials
uniform sampler2D material_a;
uniform sampler2D material_b;

struct MaterialA {
    float smoothness;
    float albedo;
    float metallic;
    float ior;
};

struct MaterialB {
    float anisotropy;
    float anisotropy_ratio;
    vec2  anisotropy_direction;
};

MaterialA parse_material_a(vec2 uv) {
    vec4 data = unpack_channels(texture(material_a, uv));

    return MaterialA(data.r, data.g, data.b, data.a);
}

MaterialB parse_material_b(vec2 uv) {
    vec4 data = unpack_channels(texture(material_b, uv));

    return MaterialB(data.r, data.g, data.ba);
}

uniform vec3 camera_position;

#define MAX_LIGHTS 16

Light lights[MAX_LIGHTS];

uniform float alpha_min = 0.01;

uniform float monitor_gamma = 2.2;

#define GOLD_IOR 0.27049

//TODO: Move all these into the structure
uniform float ambient               = 0;
uniform float smoothness            = 1;
uniform float albedo                = 1.0;
uniform float ior                   = 2.4906;
uniform float metallic              = 0;
uniform float metallic_absorbtion   = 1.8;
uniform float anisotropy            = 0;
uniform float anisotropic_ratio     = 10.0;
uniform float clearcoat             = 0;
uniform float attenuation_threshold = 0.005;

void main() {
    vec4 object_tint = vec4(0.0, 0.0, 0.0, 1.0);

    //TODO: Move lights into uniform buffer
    for(int i = 0; i < MAX_LIGHTS; i++) {
        lights[0].disabled = true;
    }

    vec3 light_pos = vec3(10, 10, 10);
    vec3 light_target = vec3(0, 0, 0);

    vec3 light_dir = normalize(light_target - light_pos);

    lights[0] = Light(
        vec2(0, 100000),    //zdistance
        vec3(10, 10, 10),      //position
        light_dir,      //direction
        vec4(1, 1, 1, 1),   //color
        vec4(1, 1, 1, 1),   //ambient color
        DIRECTIONAL_LIGHT,        //kind
        50,                 //radius
        radians(1.0),
        radians(4.0),
        1,
        2,                   //intensity
        false
    );

    lights[1] = Light(
        vec2(0, 100000),    //zdistance
        vec3(-10, -10, -10),      //position
        vec3(-1, -1, -1),      //direction
        vec4(1, 1, 1, 1),   //color
        vec4(1, 1, 1, 1),   //ambient color
        POINT_LIGHT,        //kind
        50,                 //radius
        radians(1.0),
        radians(2.0),
        1,
        100,                   //intensity
        false
    );

    lights[2] = Light(
        vec2(0, 100000),    //zdistance
        vec3(0, 0, 2),      //position
        normalize(vec3(0, 0, -1)),     //direction
        vec4(1, 1, 1, 1),   //color
        vec4(1, 1, 1, 1),   //ambient color
        SPOT_LIGHT,         //kind
        50,                 //radius
        radians(0.0),    //inner cone angle
        radians(10.0),    //outer cone angle
        0.95,               //reflector efficiency
        0.4,                  //intensity
        false               //disabled
    );

    if(has_texture) {
        if(srgb_texture) {
            object_tint = gamma_decode(texture(texture_map, UV.xy), monitor_gamma);

        } else {
            object_tint = texture(texture_map, UV.xy);
        }
    }

    //Don't bother rendering this fragment at all if it's transparent.
    if(object_tint.a < alpha_min) {
        discard;
    }

    vec4 linear_color = vec4(0.0, 0.0, 0.0, 1.0);
    vec4 diffuse_color = vec4(0.0, 0.0, 0.0, 1.0);
    vec4 specular_color = vec4(0.0, 0.0, 0.0, 1.0);

    vec3 normal;

    if(has_normal_map) {
        mat3 TBN    = mat3(normalize(Tangent), normalize(BiTangent), normalize(Normal));
        normal      = normalize(TBN * normalize(unpack_channels(texture(normal_map, UV.xy)).rgb)); //Ignore alpha channel
        //normal      = normalize(normalize(Normal) + normalize(unpack_channels(texture(normal_map, UV.xy)).rgb));
    } else {
        normal      = normalize(Normal);
    }

    float roughness = pow(1.0 - smoothness, 2.0);

    for(int i = 0; i < MAX_LIGHTS; i++) {
        if(lights[i].disabled) {
            continue;
        }

        #define light lights[i]

        //Always add ambient color to everything
        linear_color += light.ambient * ambient;

        bool should_render = true;

        float light_distance;
        vec3 light_direction;

        if(light.kind == DIRECTIONAL_LIGHT) {
            //The negation is intentional, because the algorithms expect the opposite vector for the light direction.
            light_direction = -light.direction;

            /*Ignore light distance for directional lights. They are infinitely far away.*/

        } else {
            vec3 light_position = light.position;

            light_distance  = distance(light_position, Position);
            light_direction = normalize(light_position - Position);

            //Since point and spot lights are NOT infinitely far away, check if we should render them at all
            should_render   = light_distance > light.zdistance.x &&
                              light_distance < light.zdistance.y;
        }

        if(should_render) {
            float NdotL         = dot(normal, light_direction);
            float NdotLSign     = sign(NdotL);

            NdotL = clamp(abs(NdotL), 0.0, 1.0);

            //The attenuation is basically the resulting brightness of the light
            //depending on distance from the light and the light intensity
            float attenuation;

            //Vectors
            vec3 view_direction = normalize(camera_position - Position);
            vec3 half_direction = normalize(light_direction + view_direction);

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
            if(attenuation >= attenuation_threshold) {
                //Results
                vec3  specular = vec3(0.0);
                float diffuse  = 0.0;

                //Dot products, saturated
                float NdotH = clamp(dot(normal, half_direction), 0.0, 1.0);
                float NdotV = clamp(dot(normal, view_direction), 0.0, 1.0);
                float VdotH = clamp(dot(view_direction, half_direction), 0.0, 1.0);
                float VdotL = clamp(dot(view_direction, light_direction), 0.0, 1.0);

                //roughness squared
                float sigma2 = roughness * roughness;

                //Compute fresnel ratio term
                float F0 = fresnel0(ior);

                /// Oren-Nayar diffuse part plus retro-reflection

                diffuse = oren_nayar(sigma2, albedo, metallic, NdotL, NdotV, F0, view_direction, light_direction, normal);

                diffuse = apply_retro_reflection(diffuse, roughness, albedo, metallic, NdotV, NdotL, VdotH);

                /// Modified Cook-Torrance specular part

                //Terms
                float D, G;
                vec3  F;

                //Geometric attenuation part using weighted anisotropic/isotropic attenuations
                float NdotH2 = NdotH * NdotH;

                if(anisotropy > EPSILON) {
                    vec3 X = normalize(Tangent - dot(normal, Tangent) * normal);
                    vec3 Y = cross(X, normal);

                    //float D_isotropic   = gtr_distribution(sigma2 + EPSILON, NdotH, 2.0);
                    float D_isotropic   = ggx_distribution(sigma2, NdotH2);
                    float D_anisotropic = ggx_anisotropic_distribution(roughness, NdotH2, half_direction, X, Y, 1.0, anisotropic_ratio);

                    D = mix(D_isotropic, D_anisotropic, anisotropy);

                } else {
                    //Just use the purelly isotropic version if no anisotropy is specified.
                    //D = gtr_distribution(sigma2 + EPSILON, NdotH, 2.0);
                    D = ggx_distribution(sigma2, NdotH2);
                }

                if(metallic > EPSILON) {
                    float F_Dialectric = fresnel_schlick(F0, VdotH);
                    float F_Conductive = fresnel_metallic(ior, VdotH, metallic_absorbtion);

                    F = mix(vec3(F_Dialectric), mix(F_Conductive, object_tint.rgb, metallic), metallic);

                } else {
                    //Just use the purely dialectric fresnel if no conductivity is specified
                    F = vec3(fresnel_schlick(F0, VdotH));
                }

                //Smith form of the goemetric attenuations
                G = ggx_geo_attenuation(sigma2, NdotL) *
                    ggx_geo_attenuation(sigma2, NdotV);

                //Combine all the terms using the Cook-Torrance form
                specular =     (D * F * G) /
                          (NdotV * NdotL * 4.0);

                diffuse_color.rgb += (diffuse * (1.0 - specular)) * light.color.rgb * attenuation;
                specular_color.rgb += specular * light.color.rgb * attenuation;
            }
        }
    }

    vec3 blended_specular_color = mix(BlendMultiply(Desaturate(object_tint.rgb, 1.0 - clearcoat), specular_color.rgb),
                                  mix(specular_color.rgb, BlendHardLight(object_tint.rgb, specular_color.rgb), roughness),
                                  clearcoat);

    //Merge diffuse and specular components with object tint
    linear_color.rgb += BlendMultiply(object_tint.rgb, diffuse_color.rgb) + blended_specular_color;

    //Map HDR into linear space
    linear_color.rgb = ACESFilm_tonemap_exposure(linear_color.rgb, 2.0);

    //Convert linear color into monitor gamma space
    color = gamma_encode(linear_color, monitor_gamma);

    //Premultiply color by alpha before blending
    color.rgb *= color.a;
}
