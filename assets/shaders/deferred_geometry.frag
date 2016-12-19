#version 330 core
precision highp float;

#include "lib/utils.glsl"

layout (location = 0) out vec4 gColorS;
layout (location = 1) out vec4 gNormalM;
layout (location = 2) out vec4 gPositionD;

in vec3 Position;
in vec3 Normal;
in vec3 UV;

uniform sampler2D color;

void main() {
    float smoothness = 0.8;
    float metallic = 0.0;

    gColorS.rgb = texture(color, UV.xy).rgb;
    gColorS.w = smoothness;

    gNormalM.xyz = Normal;
    gNormalM.w = metallic;

    gPositionD.xyz = Position;
    gPositionD.w = gl_FragCoord.z / gl_FragCoord.w;
}