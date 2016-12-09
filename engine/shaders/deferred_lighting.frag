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
 *  https://gist.github.com/bkaradzic/6011431
 *  http://computergraphics.stackexchange.com/questions/3646/opengl-glsl-sobel-edge-detection-filter
 *  http://www.cim.mcgill.ca/~image529/TA529/Image529_99/assignments/edge_detection/references/sobel.htm
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
uniform vec2 resolution;

uniform float gamma = 2.2;
uniform float exposure = 1.0;
uniform float depth_edge_threshold = 0.25;

#include "lighting_phong.glsl"
#include "lighting_pbr.glsl"

#include "lib/convolution/sobel5.glsl"
#include "lib/color.glsl"

layout (location = 0) out vec4 gColor;

in vec2 UV;

uniform sampler2D ColorSs;
uniform sampler2D NormalMs;
uniform sampler2D PositionDs;

void main() {
    vec2 MUV = UV;

#ifdef DEBUG
    MUV *= 2.0;

    if(UV.x > 0.5 && UV.y > 0.5) {
        gColor.rgb = texture(ColorSs, MUV).rgb;
    } else if(UV.x > 0.5 && UV.y < 0.5) {
        gColor.rgb = texture(NormalMs, MUV).rgb;
    } else if(UV.x < 0.5 && UV.y > 0.5) {
        gColor.rgb = texture(PositionDs, MUV).rgb;
    } else {
#endif

    vec4 ColorS     = texture(ColorSs, MUV);
    vec4 NormalM    = texture(NormalMs, MUV);

    vec3 Color      = gamma_decode(ColorS.rgb, 2.2);
    vec3 Normal     = normalize(NormalM.xyz);
    vec4 PositionD  = texture(PositionDs, MUV);
    vec3 Position = PositionD.xyz;
    float Depth = PositionD.w;

    float smoothness = ColorS.w;
    float roughness = pow(1.0 - smoothness, 2.0);
    float metallic  = NormalM.w;

    if(length(Normal) > EPSILON) {
        test_lights();

        const float clearcoat = 0.0;
        const float retro_reflection = 1.0;
        const float metallic_absorption = 0.4;
        const float albedo = 1.0;
        const float ior = 1.0;
        const float anisotropy = 0.0;
        const float anisotropic_ratio = 2.0;

        //gColor.rgb = calc_lighting_phong(Color, Position, Normal, view_position, albedo, metallic).rgb;

        vec4 HDR_Color = calc_lighting_pbr(Color, Position, Normal, view_position,
            roughness, clearcoat, metallic, metallic_absorption, albedo, retro_reflection, ior, anisotropy, anisotropic_ratio);

        //Map HDR into linear space
        vec4 LDR_Color = ACESFilm_tonemap_exposure(HDR_Color, exposure);

        //Convert to gamma space
        gColor.rgb = gamma_encode(LDR_Color.rgb, gamma);
    }

#ifdef DEBUG
    }

    //Add lines between quadrants
    gColor.rgb += (0.0001 / abs(UV.x - 0.5));
    gColor.rgb += (0.0001 / abs(UV.y - 0.5));
#endif

    //Get the edges
    float edge = sobel5(1.0 / resolution, PositionDs, MUV);

    //Encode color Luma for FXAA usage
    gColor.a = dot(gColor.rgb, vec3(0.299, 0.587, 0.114));

    if(edge < depth_edge_threshold) {
        //Invert Luma to tell the screen shader not to use FXAA on this texel
        gColor.a = -gColor.a;
    }
}
