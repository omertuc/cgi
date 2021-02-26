#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    OUT.Color = Color.xyz;
    gl_Position = vec4(Position, 1.0);
}