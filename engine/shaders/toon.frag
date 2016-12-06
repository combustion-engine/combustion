#version 330 core

precision highp float;

in vec3 WorldPos;
in vec3 Normal;

out vec4 color;

uniform vec3 view_pos;

const vec3 object_color = vec3(0.75, 0.75, 0.75);

const int LEVELS = 4;

vec4 levels[LEVELS];

void main() {
    levels[0] = vec4(vec3(1), 0.95);
    levels[1] = vec4(vec3(0.6), 0.5);
    levels[2] = vec4(vec3(0.4), 0.25);
    levels[3] = vec4(vec3(0.2), 0);

    vec3 normal             = normalize(Normal);
    vec3 view_direction     = normalize(view_pos - WorldPos);

    float intensity = clamp(dot(view_direction, normal), 0, 1);

    vec3 multiple = vec3(0);

    for(int i = 0; i < LEVELS; i++) {
        if(intensity >= levels[i].w) {
            multiple = levels[i].xyz;
            break;
        }
    }

    color.rgb   = multiple * object_color;
    color.a     = 1.0;
}