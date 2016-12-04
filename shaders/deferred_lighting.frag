#version 330
#pragma optionNV (unroll all)
//#define DEBUG

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

uniform vec3 view_position;
uniform mat4 view;
uniform mat4 projection;

#include "lighting_phong.glsl"
#include "lighting_pbr.glsl"

layout (location = 0) out vec3 gColor;

in vec2 UV;

uniform sampler2D ColorRs;
uniform sampler2D NormalMs;
uniform sampler2D Positions;

void main() {
    vec2 MUV = UV;

#ifdef DEBUG
    MUV *= 2.0;

    if(UV.x > 0.5 && UV.y > 0.5) {
        gColor = texture(ColorRs, MUV).rgb;
    } else if(UV.x > 0.5 && UV.y < 0.5) {
        gColor = texture(NormalMs, MUV).rgb;
    } else if(UV.x < 0.5 && UV.y > 0.5) {
        gColor = texture(Positions, MUV).rgb;
    } else {
#endif

    vec4 ColorR     = texture(ColorRs, MUV);
    vec4 NormalM    = texture(NormalMs, MUV);

    vec3 Color      = gamma_decode(ColorR.rgb, 2.2);
    vec3 Normal     = normalize(NormalM.xyz);
    vec3 Position   = texture(Positions, MUV).xyz;

    float roughness = 0.1; //ColorR.w;
    float metallic  = NormalM.w;

    if(length(Normal) > EPSILON) {
        test_lights();

        const float clearcoat = 1.0;
        const float metallic_absorption = 0.0;
        const float albedo = 1.0;
        const float ior = 1.0;
        const float anisotropy = 0.0;
        const float anisotropic_ratio = 2.0;

        gColor = calc_lighting_phong(Color, Position, Normal, view_position, albedo, metallic).rgb;

        //gColor = calc_lighting_pbr(Color, Position, Normal, view_position,
        //    roughness, clearcoat, metallic, metallic_absorption, albedo, ior, anisotropy, anisotropic_ratio).rgb;
    }

#ifdef DEBUG
    }

    //Add lines between quadrants
    gColor += vec3(1.0) * (0.0005 / abs(UV.x - 0.5));
    gColor += vec3(1.0) * (0.0005 / abs(UV.y - 0.5));
#endif
}
