#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec3 Normal;

out VS_OUTPUT {
    vec4 Color;
} OUT;

#define M_PI 3.1415926535897932384626433832795

uniform float model_scale;

// Model matrics
uniform mat4 model_rotation;
uniform mat4 model_translation;

// View matrices
uniform mat4 view_rotation;
uniform mat4 view_translation;

// Projection matrix
uniform mat4 projection;

// Spotlight color
uniform vec4 solid_color;

void main()
{
    gl_Position = vec4(
        projection *
        view_rotation *
        view_translation *
        model_translation *
        model_rotation *
        vec4(model_scale * Position.xyz, 1.0));

    // Intentionally ignore vertex color and use the solid color uniform instead
    OUT.Color = solid_color;
}