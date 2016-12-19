#ifndef SHADER_LIB_HDR_GLSL_INCLUDED
#define SHADER_LIB_HDR_GLSL_INCLUDED

vec3 reinhard_tonemap(vec3 color) {
    return color / (color + 1.0);
}

vec3 exposure_tonemap(vec3 color, float exposure) {
    return 1.0 - exp(-color * exposure);
}

//http://filmicgames.com/archives/75
vec3 uncharted_2_tonemap(vec3 x) {
#ifdef UNCHARTED_2_TONEMAP_REFERENCE_ARCHIVE
   const float A = 0.15; //Shoulder Strength
   const float B = 0.50; //Linear Strength
   const float C = 0.10; //Linear Angle
   const float D = 0.20; //Toe Strength
   const float E = 0.02; //Toe Numerator
   const float F = 0.30; //Toe Denominator
#else
   const float A = 0.22; //Shoulder Strength
   const float B = 0.30; //Linear Strength
   const float C = 0.10; //Linear Angle
   const float D = 0.20; //Toe Strength
   const float E = 0.01; //Toe Numerator
   const float F = 0.30; //Toe Denominator
#endif

   return ((x * (A * x + C * B) + D * E) / (x * (A * x + B) + D * F)) - (E / F);
}

vec3 uncharted_2_white_point = uncharted_2_tonemap(vec3(11.2));

float uncharted_2_hard_bias = 16.0;

vec3 uncharted_2_exposure_tonemap(vec3 color, float exposure_bias) {
    return uncharted_2_tonemap(color * uncharted_2_hard_bias * exposure_bias) / uncharted_2_white_point;
}

vec3 ACESFilm_tonemap(vec3 x) {
    float A = 2.51f;
    float B = 0.03f;
    float C = 2.43f;
    float D = 0.59f;
    float E = 0.14f;

    return (x * (A * x + B)) / (x * (C * x + D) + E);
}

vec4 ACESFilm_tonemap(vec4 x) {
    return vec4(ACESFilm_tonemap(x.rgb), x.a);
}

vec3 ACESFilm_tonemap_exposure(vec3 color, float exposure_bias) {
    return ACESFilm_tonemap(color * exposure_bias);
}

vec4 ACESFilm_tonemap_exposure(vec4 color, float exposure_bias) {
    return ACESFilm_tonemap(color * exposure_bias);
}

#endif //SHADER_LIB_HDR_GLSL_INCLUDED