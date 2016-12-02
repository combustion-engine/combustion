#version 330
#pragma optionNV (unroll all)

precision highp float;

layout (location = 0) out vec3 gColor;

in vec2 UV;

uniform sampler2D colors;
uniform sampler2D normals;
uniform sampler2D RMDs;

#define DEBUG

void main() {
    vec2 MUV = UV;

#ifdef DEBUG
    MUV *= 2.0;

    if(UV.x > 0.5 && UV.y > 0.5) {
        gColor = texture(colors, MUV).rgb;
    } else if(UV.x > 0.5 && UV.y < 0.5) {
        gColor = texture(normals, MUV).rgb;
    } else if(UV.x < 0.5 && UV.y > 0.5) {
        gColor = texture(RMDs, MUV).rgb;
    } else {
#endif

    vec4 color = texture(colors, MUV);
    vec4 normal = texture(normals, MUV);
    vec4 rmd = texture(RMDs, MUV);

    gColor = color.xyz / 10.0;

#ifdef DEBUG
    }

    float dx = abs(UV.x - 0.5);
    float dy = abs(UV.y - 0.5);

    //if(dx < 0.01) {
        gColor += vec3(1.0) * (0.0005 / dx);
    //}

    //if(dy < 0.01) {
        gColor += vec3(1.0) * (0.0005 / dy);
    //}
#endif
}
