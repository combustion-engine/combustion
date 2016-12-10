#version 330 core
#pragma optionNV (unroll all)

out vec4 color;

in vec2 UV;

uniform sampler2D screen;

uniform vec2 resolution;
uniform vec2 texture_resolution;

void main() {
    //vec2 rcp = 1.0 / resolution;
    //vec2 trcp = 1.0 / texture_resolution;

    //float aspect = resolution.y / resolution.x;
    //float taspect = texture_resolution.y / texture_resolution.x;

    //if(aspect > taspect) {
    //    float h_edge = resolution.y - texture_resolution.y;
    //}

    color.rgb = texture(screen, UV).rgb;
    color.a = 1.0;
}