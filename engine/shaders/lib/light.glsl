#ifndef SHADER_LIB_LIGHT_GLSL_INCLUDED
#define SHADER_LIB_LIGHT_GLSL_INCLUDED

#define DIRECTIONAL_LIGHT   1   //Like the Sun. All rays are parallel.
#define POINT_LIGHT         2   //Like a normal lightbulb
#define SPOT_LIGHT          3   //Like a flashlight

#define ATTENUATION_THRESHOLD 0.005

struct Light {
    vec2 zdistance;             //Minimum and maximum distances the light can touch. Hard limit. Ignored for Directional Lights
    vec3 position;              //Position of light in space. Ignored for Directional lights
    vec3 direction;             //Direction the light is pointing at. Used in Directional and Spot lights
    vec4 color;                 //Light color
    vec4 ambient;               //Ambient color
    int kind;                   //Directional, Point, Spot, etc
    float radius;               //Spherical radius of entire light. Soft limit. Ignored for Directional lights
    float inner_cone;           //Angle (in radians) of the inner spotlight cone
    float outer_cone;           //Angle (in radians) of the outer spotlight cone
    float reflector_efficiency; //Efficiency of the spotlight cone reflector (like the inside of a flashlight)
    float intensity;            //Light intensity
    bool disabled;              //Pretty obvious
};

#define DISABLED_LIGHT Light(vec2(0), vec3(0), vec3(0), vec4(0), vec4(0), DIRECTIONAL_LIGHT, 0, 0, 0, 0, 0, true)

#define MAX_LIGHTS 16

Light lights[MAX_LIGHTS] = Light[MAX_LIGHTS](
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    DISABLED_LIGHT,
    Light(
        vec2(0, 100000),    //zdistance
        vec3(0, 0, 10),      //position
        vec3(-1, -1, -1),      //direction
        vec4(1, 1, 1, 1),   //color
        vec4(1, 1, 1, 1),   //ambient color
        POINT_LIGHT,        //kind
        50,                 //radius
        radians(1.0),
        radians(2.0),
        1,
        100,                   //intensity
        false
    ),
    Light(
        vec2(0, 100000),    //zdistance
        vec3(5, 0, -5),      //position
        vec3(-1, -1, -1),      //direction
        vec4(1, 1, 1, 1),   //color
        vec4(1, 1, 1, 1),   //ambient color
        POINT_LIGHT,        //kind
        50,                 //radius
        radians(1.0),
        radians(2.0),
        1,
        100,                   //intensity
        false
    ),
    Light(
        vec2(0, 100000),    //zdistance
        vec3(0, 0, 10),      //position
        vec3(0, 0, -1),      //direction
        vec4(1, 1, 1, 1),   //color
        vec4(1, 1, 1, 1),   //ambient color
        SPOT_LIGHT,        //kind
        50,                 //radius
        radians(7.0),
        radians(15.0),
        1,
        10,                   //intensity
        false
    ),
    Light(
        vec2(0, 100000),    //zdistance
        vec3(0, 2, 0),      //position
        vec3(0, -1, 0),      //direction
        vec4(1, 1, 1, 1),   //color
        vec4(1, 1, 1, 1),   //ambient color
        SPOT_LIGHT,        //kind
        50,                 //radius
        radians(7.0),
        radians(15.0),
        1,
        10,                   //intensity
        false
    ),
    Light(
        vec2(0, 100000),    //zdistance
        vec3(0, 6, 0),      //position
        vec3(0, -1, 0),      //direction
        vec4(1, 1, 1, 1),   //color
        vec4(1, 1, 1, 1),   //ambient color
        SPOT_LIGHT,        //kind
        50,                 //radius
        radians(7.0),
        radians(15.0),
        1,
        10,                   //intensity
        false
    )
);

void test_lights() {

}
#endif //SHADER_LIB_LIGHT_GLSL_INCLUDED