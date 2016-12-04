#ifndef SHADER_PARTIAL_PHONG_GLSL_INCLUDED
#define SHADER_PARTIAL_PHONG_GLSL_INCLUDED

float phong(vec3 lightDirection, vec3 viewDirection, vec3 surfaceNormal, float shininess) {
  //Calculate Phong power
  vec3 R = -reflect(lightDirection, surfaceNormal);

  return pow(max(0.0, dot(viewDirection, R)), shininess);
}

float blinn_phong(vec3 lightDirection, vec3 viewDirection, vec3 surfaceNormal, float shininess) {
  //Calculate Blinn-Phong power
  vec3 H = normalize(viewDirection + lightDirection);

  return pow(max(0.0, dot(surfaceNormal, H)), shininess);
}

#endif //SHADER_PARTIAL_PHONG_GLSL_INCLUDED