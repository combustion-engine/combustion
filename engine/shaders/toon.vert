#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform mat4 model_inverse_transposed;

out vec3 Normal;
out vec3 WorldPos;

void main() {
    vec4 vModelPosition = view * model * vec4(position, 1.0);

    gl_Position = projection * vModelPosition;

    WorldPos = vModelPosition.xyz;

    Normal = normalize(mat3(model_inverse_transposed) * normal);
}
