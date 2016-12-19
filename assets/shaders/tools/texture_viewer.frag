#version 330 core
#pragma optionNV (unroll all)

out vec4 color;

in vec2 UV;

uniform sampler2D screen;

uniform vec2 resolution;
uniform vec2 texture_resolution;

uniform float zoom;
uniform vec2 pos;

void main() {
    vec2 MUV = UV;

    MUV -= 0.5;

    if(texture_resolution.x < resolution.x) {
        MUV.x /= texture_resolution.x / resolution.x;
    } else {
        MUV.x *= resolution.x / texture_resolution.x;
    }

    if(texture_resolution.y < resolution.y) {
        MUV.y /= texture_resolution.y / resolution.y;
    } else {
        MUV.y *= resolution.y / texture_resolution.y;
    }

    MUV.x += (pos.x / texture_resolution.x);
    MUV.y -= (pos.y / texture_resolution.y);

    MUV *= zoom;
    MUV += 0.5;

    MUV.y = 1.0 - MUV.y;

    color.rgb = texture(screen, MUV, step(5.0, zoom) * (1.0 / zoom)).rgb;

    color.a = 1.0;
}