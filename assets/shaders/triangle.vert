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
uniform mat4 view_translation;
uniform mat4 view_rotation;

// Projection matrix
uniform mat4 projection;

// Spot lights
#define MAX_LIGHTS 330u
uniform uint lights_count;
uniform vec3[MAX_LIGHTS] light_positions;
uniform vec4[MAX_LIGHTS] light_colors;
uniform float[MAX_LIGHTS] light_radiuses;

// Ambient lightning
float ambient_strength = 0.2;
vec4 ambient_color = vec4((ambient_strength * vec3(1.0, 1.0, 1.0)), 1.0);

void main()
{
    vec3 vertex_world_position = (model_translation * model_rotation * vec4(model_scale * Position.xyz, 1.0)).xyz;
    vec3 vertex_normal = (model_rotation * vec4(Normal, 1.0)).xyz;

    vec4 final_color = ambient_color;
    for (uint i = 0u; i < lights_count; i++) {
        // Get current light
        vec3 light_position = light_positions[i];
        vec4 light_color = light_colors[i];
        float light_radius = light_radiuses[i];

        // Light direction
        vec3 light_direction = light_position - vertex_world_position;
        float diffuse = max(0.0, dot(normalize(vertex_normal), normalize(light_direction)));

        // Light distance / attenuation
        float light_distance = distance(light_position, vertex_world_position);
        float light_attenuation = 1.0 - (min(light_distance, light_radius) / light_radius);

        // Output final color
        vec4 final_spot_color = light_attenuation * diffuse * light_color;
        final_color += final_spot_color;
    }

    OUT.Color = Color * final_color;

    gl_Position = projection * view_translation * view_rotation * vec4(vertex_world_position, 1.0);

}