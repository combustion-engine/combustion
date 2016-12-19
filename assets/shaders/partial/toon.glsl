#ifndef SHADER_PARTIAL_TOON_GLSL_INCLUDED
#define SHADER_PARTIAL_TOON_GLSL_INCLUDED

const int LEVELS = 4;

vec2 levels[LEVELS] = vec2[LEVELS](
    vec2(0.95, 1.0),
    vec2(0.50, 0.6),
    vec2(0.25, 0.4),
    vec2(0.00, 0.2)
);

vec3 toonify(vec3 color) {
    float intensity = length(color);

    for(int i = 0; i < LEVELS; i++) {
        if(intensity >= levels[i].x) {
            vec3 c = color * levels[i].y;

            return c;
        }
    }

    return vec3(0.0);
}

#endif //SHADER_PARTIAL_TOON_GLSL_INCLUDED