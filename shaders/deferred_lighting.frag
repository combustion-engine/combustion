#version 330
#pragma optionNV (unroll all)

precision highp float;

layout (location = 0) out vec3 gColor;

in vec2 UV;

uniform sampler2D colors;
uniform sampler2D normals;
uniform sampler2D RMDs;

void main() {
    vec4 color = texture(colors, UV);
    vec4 normal = texture(normals, UV);
    vec4 rmd = texture(RMDs, UV);

    gColor = color.xyz;
}
