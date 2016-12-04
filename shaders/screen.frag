#version 330 core

out vec4 color;

in vec2 UV;
in vec2 rgbNW;
in vec2 rgbNE;
in vec2 rgbSW;
in vec2 rgbSE;
in vec2 rgbM;
in vec2 UVResolution;

#include "lib/fxaa/fxaa.glsl"

uniform sampler2D screen;
uniform vec2 resolution;

void main() {
    color = fxaa(screen, UVResolution, resolution, rgbNW, rgbNE, rgbSW, rgbSE, rgbM);
}