#version 330 core
#extension GL_ARB_gpu_shader5 : enable

out vec4 color;

in vec2 UV;

#define FXAA_PC 1
#define FXAA_GLSL_130 1
#define FXAA_QUALITY__PRESET 39

#include "external/Fxaa3_11.h"

uniform sampler2D screen;
uniform vec2 resolution;

void main()
{
    vec2 rcp = 1.0 / resolution;

    //Filthy console dummies
    float dummy1 = 0.0;
    vec4 dummy4 = vec4(0.0);

    // Only used on FXAA Quality.
    // This used to be the FXAA_QUALITY__SUBPIX define.
    // It is here now to allow easier tuning.
    // Choose the amount of sub-pixel aliasing removal.
    // This can effect sharpness.
    //   1.00 - upper limit (softer)
    //   0.75 - default amount of filtering
    //   0.50 - lower limit (sharper, less sub-pixel aliasing removal)
    //   0.25 - almost off
    //   0.00 - completely off
    float fxaaQualitySubpix = 1.0;

    // Only used on FXAA Quality.
    // This used to be the FXAA_QUALITY__EDGE_THRESHOLD define.
    // It is here now to allow easier tuning.
    // The minimum amount of local contrast required to apply algorithm.
    //   0.333 - too little (faster)
    //   0.250 - low quality
    //   0.166 - default
    //   0.125 - high quality
    //   0.063 - overkill (slower)
    float fxaaQualityEdgeThreshold = 0.063;

    // Only used on FXAA Quality.
    // This used to be the FXAA_QUALITY__EDGE_THRESHOLD_MIN define.
    // It is here now to allow easier tuning.
    // Trims the algorithm from processing darks.
    //   0.0833 - upper limit (default, the start of visible unfiltered edges)
    //   0.0625 - high quality (faster)
    //   0.0312 - visible limit (slower)
    // Special notes when using FXAA_GREEN_AS_LUMA,
    //   Likely want to set this to zero.
    //   As colors that are mostly not-green
    //   will appear very dark in the green channel!
    //   Tune by looking at mostly non-green content,
    //   then start at zero and increase until aliasing is a problem.
    float fxaaQualityEdgeThresholdMin = 0.0312;

    color = FxaaPixelShader(UV, dummy4,
                            screen, screen, screen,
                            rcp, dummy4, dummy4, dummy4,
                            fxaaQualitySubpix, fxaaQualityEdgeThreshold,
                            fxaaQualityEdgeThresholdMin,
                            dummy1, dummy1, dummy1, dummy4);
}