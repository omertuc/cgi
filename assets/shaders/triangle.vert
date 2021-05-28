#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec3 Normal;

out VS_OUTPUT {
    vec4 Color;
    vec3 Normal;
    vec3 WorldCoords;
} OUT;

#define M_PI 3.1415926535897932384626433832795

uniform float model_scale;

// Model matrics
uniform mat4 model_rotation;
uniform mat4 model_translation;

// View matrices
uniform mat4 view_rotation;
uniform mat4 view_translation;
uniform vec3 view_location;

// Projection matrix
uniform mat4 projection;

void main()
{
    vec3 vertex_world_location = (model_translation * model_rotation * vec4(model_scale * Position.xyz, 1.0)).xyz;
    gl_Position = projection * view_rotation * view_translation * vec4(vertex_world_location, 1.0);

    OUT.Color = Color;
    OUT.Normal = (model_rotation * vec4(Normal, 1.0)).xyz;
    OUT.WorldCoords = vertex_world_location;
}