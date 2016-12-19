#ifndef SHADER_LIB_SOBEL_GLSL_INCLUDED
#define SHADER_LIB_SOBEL_GLSL_INCLUDED

#include "kernels/sobel.glsl"

/*
 * Optimized convolution filters for Sobel edge detection.
 * Only accesses textures once for both kernels.
 */

float sobel3w(vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[3] = float[3](-1.0, 0.0, 1.0);

    float Gx = 0.0;
    float Gy = 0.0;

    for(int row = 0; row < 3; row++) {
        for(int col = 0; col < 3; col++) {
            float C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                        uv.y + (rcp.y * M[row]))).w;

            Gx += C * sobel3_h_kernel[col][row];
            Gy += C * sobel3_v_kernel[col][row];
        }
    }

    return clamp(sqrt(Gx * Gx + Gy * Gy), 0.0, 1.0);
}

vec4 sobel3(vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[3] = float[3](-1.0, 0.0, 1.0);

    vec4 Gx = vec4(0.0);
    vec4 Gy = vec4(0.0);

    for(int row = 0; row < 3; row++) {
        for(int col = 0; col < 3; col++) {
            vec4 C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                       uv.y + (rcp.y * M[row])));

            Gx += C * sobel3_h_kernel[col][row];
            Gy += C * sobel3_v_kernel[col][row];
        }
    }

    return clamp(sqrt(Gx * Gx + Gy * Gy), 0.0, 1.0);
}

float sobel5w(vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[5] = float[5](-2.0, -1.0, 0.0, 1.0, 2.0);

    float Gx = 0.0;
    float Gy = 0.0;

    for(int row = 0; row < 5; row++) {
        for(int col = 0; col < 5; col++) {
            float C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                        uv.y + (rcp.y * M[row]))).w;

            int index = row + col * 5;

            Gx += C * sobel5_h_kernel[index];
            Gy += C * sobel5_v_kernel[index];
        }
    }

    return clamp(sqrt(Gx * Gx + Gy * Gy), 0.0, 1.0);
}

vec4 sobel5(vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[5] = float[5](-2.0, -1.0, 0.0, 1.0, 2.0);

    vec4 Gx = vec4(0.0);
    vec4 Gy = vec4(0.0);

    for(int row = 0; row < 5; row++) {
        for(int col = 0; col < 5; col++) {
            vec4 C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                       uv.y + (rcp.y * M[row])));

            int index = row + col * 5;

            Gx += C * sobel5_h_kernel[index];
            Gy += C * sobel5_v_kernel[index];
        }
    }

    return clamp(sqrt(Gx * Gx + Gy * Gy), 0.0, 1.0);
}

#endif //SHADER_LIB_SOBEL_GLSL_INCLUDED