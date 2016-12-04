#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 uvs;

out vec2 UV;

out vec2 rgbNW;
out vec2 rgbNE;
out vec2 rgbSW;
out vec2 rgbSE;
out vec2 rgbM;
out vec2 UVResolution;

uniform vec2 resolution;

#include "lib/fxaa/texcoords.glsl"

void main() {
    gl_Position = vec4(position.x, position.y, 0.0f, 1.0f);

    UV = uvs;
    UVResolution = UV * resolution;

    fxaa_texcoords(UVResolution, resolution, rgbNW, rgbNE, rgbSW, rgbSE, rgbM);
}