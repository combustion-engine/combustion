#version 330
#pragma optionNV (unroll all)
//#define DEBUG

precision highp float;

uniform vec3 view_position;
uniform mat4 view;
uniform mat4 projection;
uniform vec2 resolution;

uniform float gamma = 2.2;
uniform float exposure = 1.0;
uniform float brightness = 1.0;
uniform float saturation = 1.0;
uniform float contrast = 1.0;

uniform float depth_edge_threshold = 0.65;

#include "lighting_phong.glsl"
#include "lighting_pbr.glsl"

#include "lib/convolution/sobel.glsl"

#include "lib/color.glsl"

layout (location = 0) out vec4 gColor;

in vec2 UV;

uniform sampler2D ColorSs;
uniform sampler2D NormalMs;
uniform sampler2D PositionDs;

void main() {
    vec2 MUV = UV;
    vec2 rcp = 1.0 / resolution;

#ifdef DEBUG
    MUV *= 2.0;

    if(UV.x > 0.5 && UV.y > 0.5) {
        gColor.rgb = texture(ColorSs, MUV - 1.0).rgb;
    } else if(UV.x > 0.5 && UV.y < 0.5) {
        gColor.rgb = texture(NormalMs, vec2(MUV.x - 1.0, MUV.y)).rgb;
    } else if(UV.x < 0.5 && UV.y > 0.5) {
        gColor.rgb = texture(PositionDs, vec2(MUV.x, MUV.y - 1.0)).rgb;
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

    //Get the edges
    float edge = sobel5w(rcp, PositionDs, MUV);

    //Uncomment this to show edge detection
    //if(edge >= depth_edge_threshold) { gColor.rgb = vec3(1.0); } else { gColor.rgb = vec3(0.0); }
    //gColor.a = -1.0;
    //return;

    if(length(Normal) > EPSILON) {
        test_lights();

        const float clearcoat = 0.9;
        const float retro_reflection = 1.0;
        const float metallic_absorption = 0.4;
        const float albedo = 0.8;
        const float ior = 1.0;
        const float anisotropy = 0.0;
        const float anisotropic_ratio = 2.0;

        //gColor.rgb = calc_lighting_phong(Color, Position, Normal, view_position, albedo, metallic).rgb;

        vec4 HDR_Color = calc_lighting_pbr(Color, Position, Normal, view_position,
            roughness, clearcoat, metallic, metallic_absorption, albedo, retro_reflection, ior, anisotropy, anisotropic_ratio);

        //Map HDR into linear space
        vec4 LDR_Color = ACESFilm_tonemap_exposure(HDR_Color, exposure);

        //Apply color transforms
        LDR_Color = ContrastSaturationBrightness(LDR_Color, brightness, saturation, contrast);

        //Convert to gamma space and clamp to 0-1
        gColor.rgb = clamp(gamma_encode(LDR_Color.rgb, gamma), 0.0, 1.0);

    } else {
        gColor.rgb = vec3(0.25);
    }

    //Encode color Luma for FXAA usage
    gColor.a = dot(gColor.rgb, vec3(0.299, 0.587, 0.114));

    if(edge < depth_edge_threshold) {
        //Invert Luma to tell the screen shader not to use FXAA on this texel
        gColor.a = -gColor.a;
    }
#ifdef DEBUG
    else {
        gColor.rgb = vec3(1.0);
    }
    }

    //Add lines between quadrants
    gColor.rgb += (rcp.x / abs(UV.x - 0.5));
    gColor.rgb += (rcp.y / abs(UV.y - 0.5));
#endif
}
