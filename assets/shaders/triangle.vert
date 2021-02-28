#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in float pitch;
layout (location = 3) in float yaw;
layout (location = 4) in float roll;

out VS_OUTPUT {
    vec4 Color;
} OUT;

#define M_PI 3.1415926535897932384626433832795

void main()
{
    OUT.Color = Color;

    mat3 roll = mat3
    (
    cos(roll), -sin(roll), 0.0,
    sin(roll), cos(roll), 0.0,
    0.0, 0.0, 1.0
    );

    mat3 yaw = mat3
    (
    cos(yaw), 0, sin(yaw),
    0, 1, 0,
    -sin(yaw), 0, cos(yaw)
    );

    mat3 pitch = mat3
    (
    1,0,0,
    0, cos(pitch), -sin(pitch),
    0, sin(pitch), cos(pitch)
    );

    gl_Position = vec4(yaw * pitch * roll * Position, 1.0);
}