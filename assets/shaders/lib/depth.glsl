#ifndef SHADER_LIB_DEPTH_GLSL_INCLUDED
#define SHADER_LIB_DEPTH_GLSL_INCLUDED

vec3 depth_to_pos(float depth, vec2 uv, mat4 projection, mat4 view) {
    float z = depth;

    vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, z, 1.0);

    vec4 viewSpacePosition = inverse(projection) * clipSpacePosition;

    viewSpacePosition.xyz /= viewSpacePosition.w;

    vec4 worldSpacePosition = inverse(view) * viewSpacePosition;

    return worldSpacePosition.xyz;
}

vec3 depth_to_pos_other(float depth, vec2 uv, mat4 projection, mat4 view) {
    vec4 projected_pos = vec4(uv * 2.0 - 1.0, depth, 1.0);

    vec4 position = inverse(projection) * projected_pos;

    return position.xyz / position.w;
}

#endif //SHADER_LIB_DEPTH_GLSL_INCLUDED