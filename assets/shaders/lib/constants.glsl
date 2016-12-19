#ifndef SHADER_LIB_CONSTANTS_GLSL_INCLUDED
#define SHADER_LIB_CONSTANTS_GLSL_INCLUDED

#define EPSILON          0.0009765625    //2^-10
#define EPSILON_1        (1.0 - EPSILON)
#define ONE_PLUS_EPSILON (1.0 + EPSILON)
#define PI               3.14159265358979323846264338327950288
#define PI_SQRT          1.77245385090551588191942755656782538
#define FRAC_1_PI        0.318309886183790671537767526745028724
#define FRAC_2_PI        0.636619772367581343075535053490057448
#define FRAC_2_PI_SQRT   0.797884560802865405726436165423365309
#define FRAC_PI_2        1.570796326794896557998981734272092581
#define TAU              6.283185307179586231995926937088370323
#define FRAC_1_TAU       (1.0 / TAU)
#define COS_25_PERCENT   0.382683432365089837290383911749813706
#define PI_SUB_2_FRAC_PI 0.363380227632418617567111596144968644 //(PI - 2.0) / PI
#define SQRT_2           1.41421356237309504880168872420969808

#endif //SHADER_LIB_CONSTANTS_GLSL_INCLUDED