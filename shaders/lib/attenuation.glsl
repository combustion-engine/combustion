#ifndef SHADER_LIB_ATTENUATION_GLSL_INCLUDED
#define SHADER_LIB_ATTENUATION_GLSL_INCLUDED

float attenuation_radius(float d, float r, float i) {
    //https://imdoingitwrong.wordpress.com/2011/01/31/light-attenuation/
    float k = 1.0 + (d / r);
    return i / (k * k);
}

float inverse_square_attenuation(float d, float r, float i) {
    float l = d / r;

    float k = clamp(1.0 - (l * l * l * l), 0.0, 1.0);

    return i * ((k * k) / (d * d + 1.0));
}

#endif //SHADER_LIB_ATTENUATION_GLSL_INCLUDED