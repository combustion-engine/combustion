#version 330 core
precision highp float;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 uvw;

uniform mat4 model;
uniform mat4 mvp;
uniform mat4 mit;

out vec3 Position;
out vec3 Normal;
out vec3 UV;

void main() {
    vec4 ModelPosition = vec4(position, 1.0);
    vec4 ModelNormal = vec4(normal, 0.0);

    Position = (model * ModelPosition).xyz;

    Normal = normalize(mit * ModelNormal).xyz;

    UV = uvw;

    gl_Position = mvp * ModelPosition;
}