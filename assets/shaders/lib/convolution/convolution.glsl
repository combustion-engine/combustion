#ifndef SHADER_LIB_CONVOLUTION_GLSL_INCLUDED
#define SHADER_LIB_CONVOLUTION_GLSL_INCLUDED

float convolve3w(mat3 kernel, vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[3] = float[3](-1.0, 0.0, 1.0);

    float sum = 0.0;

    for(int row = 0; row < 3; row++) {
        for(int col = 0; col < 3; col++) {
            float C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                        uv.y + (rcp.y * M[row]))).w;

            sum += C * kernel[col][row];
        }
    }

    return sum;
}

vec4 convolve3(mat3 kernel, vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[3] = float[3](-1.0, 0.0, 1.0);

    vec4 sum = vec4(0.0);

    for(int row = 0; row < 3; row++) {
        for(int col = 0; col < 3; col++) {
            vec4 C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                       uv.y + (rcp.y * M[row])));

            sum += C * kernel[col][row];
        }
    }

    return sum;
}

float convolve5w(float[25] kernel, vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[5] = float[5](-2.0, -1.0, 0.0, 1.0, 2.0);

    float sum = 0.0;

    for(int row = 0; row < 5; row++) {
        for(int col = 0; col < 5; col++) {
            float C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                        uv.y + (rcp.y * M[row]))).w;

            sum += C * kernel[row + col * 5];
        }
    }

    return sum;
}

vec4 convolve5(float[25] kernel, vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[5] = float[5](-2.0, -1.0, 0.0, 1.0, 2.0);

    vec4 sum = vec4(0.0);

    for(int row = 0; row < 5; row++) {
        for(int col = 0; col < 5; col++) {
            vec4 C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                       uv.y + (rcp.y * M[row])));

            sum += C * kernel[row + col * 5];
        }
    }

    return sum;
}

vec4 convolve7(float[49] kernel, vec2 rcp, sampler2D tex, vec2 uv) {
    const float M[7] = float[7](-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0);

    vec4 sum = vec4(0.0);

    for(int row = 0; row < 7; row++) {
        for(int col = 0; col < 7; col++) {
            vec4 C = texture(tex, vec2(uv.x + (rcp.x * M[col]),
                                       uv.y + (rcp.y * M[row])));

            sum += C * kernel[row + col * 7];
        }
    }

    return sum;
}

mat3 normalize_kernel3(mat3 kernel) {
    float sum = 0.0;

    for(int row = 0; row < 3; row++) {
        for(int col = 0; col < 3; col++) {
            sum += kernel[col][row];
        }
    }

    return kernel / sum;
}

float[25] normalize_kernel5(float[25] kernel) {
    float sum = 0.0;

    for(int row = 0; row < 5; row++) {
        for(int col = 0; col < 5; col++) {
            sum += kernel[row + col * 5];
        }
    }

    for(int row = 0; row < 5; row++) {
        for(int col = 0; col < 5; col++) {
            kernel[row + col * 5] /= sum;
        }
    }

    return kernel;
}

float[49] normalize_kernel7(float[49] kernel) {
    float sum = 0.0;

    for(int row = 0; row < 7; row++) {
        for(int col = 0; col < 7; col++) {
            sum += kernel[row + col * 7];
        }
    }

    for(int row = 0; row < 7; row++) {
        for(int col = 0; col < 7; col++) {
            kernel[row + col * 7] /= sum;
        }
    }

    return kernel;
}


#endif //SHADER_LIB_CONVOLUTION_GLSL_INCLUDED