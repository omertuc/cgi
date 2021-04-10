#version 330 core

layout (location = 0) in vec4 Position;
layout (location = 1) in vec4 Color;

out VS_OUTPUT {
    vec4 Color;
} OUT;

#define M_PI 3.1415926535897932384626433832795

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    OUT.Color = Color;

    gl_Position = vec4(projection * view * model * Position);
}