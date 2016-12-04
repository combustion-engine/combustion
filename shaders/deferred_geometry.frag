#version 330 core
precision highp float;

#include "lib/utils.glsl"

layout (location = 0) out vec4 gColorR;
layout (location = 1) out vec4 gNormalM;
layout (location = 2) out vec3 gPosition;

in vec3 Position;
in vec3 Normal;
in vec3 UV;

uniform sampler2D color;

void main() {
    float roughness = 0.1;
    float metallic = 0.0;

    gColorR.rgb = texture(color, UV.xy).rgb;
    gColorR.w = roughness;

    gNormalM.xyz = Normal;
    gNormalM.w = metallic;

    gPosition.xyz = Position;
}