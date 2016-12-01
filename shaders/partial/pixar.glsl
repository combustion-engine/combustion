#ifndef SHADER_PARTIAL_PIXAR_GLSL_INCLUDED
#define SHADER_PARTIAL_PIXAR_GLSL_INCLUDED

//See page 10: http://blog.selfshadow.com/publications/s2013-shading-course/pixar/s2013_pbs_pixar_notes.pdf
float pixar_brdf(float sigma2, float NdotH, float VdotH) {
    float NdotH2 = NdotH * NdotH;
    float A = (NdotH2 - 1.0) / (sigma2 * NdotH2);
    float B = 4.0 * PI * sigma2 * pow(NdotH, 3.0) * VdotH;

    return exp(A) / B;
}

#endif //SHADER_PARTIAL_PIXAR_GLSL_INCLUDED