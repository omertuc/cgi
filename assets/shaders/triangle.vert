#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in float rot;

out VS_OUTPUT {
    vec4 Color;
} OUT;

#define M_PI 3.1415926535897932384626433832795

void main()
{
    OUT.Color = Color;

    mat3 roll = mat3
    (
    cos(rot), -sin(rot), 0.0,
    sin(rot), cos(rot), 0.0,
    0.0, 0.0, 1.0
    );

    mat3 yaw = mat3
    (
    cos(rot), 0, sin(rot),
    0, 1, 0,
    -sin(rot), 0, cos(rot)
    );

    mat3 pitch = mat3
    (
    1,0,0,
    0, cos(rot), -sin(rot),
    0, sin(rot), cos(rot)
    );

    gl_Position = vec4(roll * Position, 1.0);
}