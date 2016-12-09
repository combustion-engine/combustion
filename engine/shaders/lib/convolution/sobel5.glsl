#ifndef SHADER_LIB_SOBEL_GLSL_INCLUDED
#define SHADER_LIB_SOBEL_GLSL_INCLUDED

#include "../utils.glsl"

float sobel5_T(sampler2D tex, float x, float y) {
    return texture(tex, vec2(x, y), 0.0).w;
}

float sobel5(vec2 rcp, sampler2D tex, vec2 uv) {
    const float sobel_h1[5] = float[5](1, 2,  0, -2,  -1);
    const float sobel_h2[5] = float[5](4, 8,  0, -8,  -4);
    const float sobel_h3[5] = float[5](6, 12, 0, -12, -6);
    const float sobel_h4[5] = float[5](4, 8,  0, -8,  -4);
    const float sobel_h5[5] = float[5](1, 2,  0, -2,  -1);

    const float sobel_v1[5] = float[5](-1, -4,  -6, -4, -1);
    const float sobel_v2[5] = float[5](-2, -8, -12, -8, -2);
    const float sobel_v3[5] = float[5](0,   0,   0,  0,  0);
    const float sobel_v4[5] = float[5](2,   8,  12,  8,  2);
    const float sobel_v5[5] = float[5](1,   4,   6,  4,  1);

    const float M[5] = float[5](-2, -1, 0, 1, 2);

    float C[5 * 5];

    for(int row = 0; row < 5; row++) {
        for(int col = 0; col < 5; col++) {
            C[row + col * 5] = sobel5_T(tex, uv.x + (rcp.x * M[col]),
                                             uv.y + (rcp.y * M[row]));
        }
    }

    float Gz = dot5(sobel_h1, C, 0) +
               dot5(sobel_h2, C, 1) +
               dot5(sobel_h3, C, 2) +
               dot5(sobel_h4, C, 3) +
               dot5(sobel_h5, C, 4);

    float Gy = dot5(sobel_v1, C, 0) +
               dot5(sobel_v2, C, 1) +
               dot5(sobel_v3, C, 2) +
               dot5(sobel_v4, C, 3) +
               dot5(sobel_v5, C, 4);

    return clamp(sqrt(Gz * Gz + Gy * Gy), 0.0, 1.0);
}


#endif //SHADER_LIB_SOBEL_GLSL_INCLUDED