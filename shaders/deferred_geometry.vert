#version 330 core
precision highp float;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 uvw;
layout(location = 3) in vec3 tangent;
layout(location = 4) in vec3 bitangent;

uniform mat4 model;
uniform mat4 mvp;
uniform mat4 mit;

out vec3 Normal;
out vec3 Position;
out vec3 Tangent;
out vec3 BiTangent;
out vec3 UV;

void main() {
    vec4 ModelPosition = vec4(position, 1.0);
    vec4 ModelNormal = vec4(normal, 0.0);
    vec4 ModelTangent = vec4(tangent, 0.0);
    vec4 ModelBiTangent = vec4(bitangent, 0.0);

    Position = (model * ModelPosition).xyz;

    Normal = normalize(mit * ModelNormal).xyz;
    Tangent = normalize(mit * ModelTangent).xyz;
    BiTangent = normalize(mit * ModelBiTangent).xyz;

    UV = uvw;

    gl_Position = mvp * ModelPosition;
}