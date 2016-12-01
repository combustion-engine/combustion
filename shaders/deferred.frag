#version 330 core
precision highp float;

#include "lib/utils.glsl"

layout (location = 0) out vec3 gColor;
layout (location = 1) out vec3 gNormal;
layout (location = 2) out vec3 gRMD;

in vec3 Normal;
in vec3 Position;
in vec3 Tangent;
in vec3 BiTangent;
in vec3 UV;

//uniform sampler2D texture_diffuse1;
//uniform sampler2D texture_specular1;

void main() {
    gColor = vec3(1.0, 1.0, 1.0);
    gNormal = pack_channels(normalize(Normal));
    gRMD = vec3(0.1, 0.0, gl_FragCoord.z / gl_FragCoord.w);
}