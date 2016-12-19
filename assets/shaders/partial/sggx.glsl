#ifndef SHADER_PARTIAL_SGGX_GLSL_INCLUDED
#define SHADER_PARTIAL_SGGX_GLSL_INCLUDED

#include "../lib/constants.glsl"

mat3 SGGX_S(float sigma2, float x, float y, float z) {
    mat3 A = mat3(
        x * x, x * y, x * z,
        x * y, y * y, y * z,
        x * z, y * z, z * z
    );

    mat3 B = mat3(
        (y * y) + (z * z), -x * y,            -x * z,
        -x * y,            (x * x) + (z * z), -y * z,
        -x * z,            -y * z,            (x * x) + (y * y)
    );

    return A + (sigma2 * B);
}

vec3 SGGX_Distribution(float sigma2, vec3 N, vec3 I) {
    mat3 S = SGGX_S(sigma2, N.x, N.y, N.z);

    float D = determinant(S);
    mat3 SI = inverse(S);

    vec3 C = SI * N;

    float NdotI = clamp(dot(N, I), 0.0, 1.0);

    vec3 denom = PI * sqrt(D) * C * C;

    return (1.0 / denom) * NdotI;
}

#endif //SHADER_PARTIAL_SGGX_GLSL_INCLUDED