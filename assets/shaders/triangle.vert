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
uniform vec3 view_location;

// Projection matrix
uniform mat4 projection;

// Spot lights
#define MAX_LIGHTS 330u
uniform uint lights_count;
uniform vec3[MAX_LIGHTS] light_locations;
uniform vec3[MAX_LIGHTS] light_colors;
uniform float[MAX_LIGHTS] light_radiuses;

// Ambient lighting
float ambient_strength = 0.35;
vec3 ambient_color = ambient_strength * vec3(1.0, 1.0, 1.0);

// Specular lighting
float specual_strength = 10;

float normal_dot_sat(vec3 v1, vec3 v2) {
    return max(dot(normalize(v1), normalize(v2)), 0.0);
}

void main()
{
    vec3 vertex_world_location = (model_translation * model_rotation * vec4(model_scale * Position.xyz, 1.0)).xyz;
    vec3 vertex_normal = (model_rotation * vec4(Normal, 1.0)).xyz;

    vec3 final_color = ambient_color;
    for (uint i = 0u; i < lights_count; i++) {
        // Get current light
        vec3 light_location = light_locations[i];
        vec3 light_color = light_colors[i];
        float light_radius = light_radiuses[i];

        // Light distance / attenuation
        float light_distance = distance(light_location, vertex_world_location);
        float light_attenuation = 1.0 - (min(light_distance, light_radius) / light_radius);

        // Light direction
        vec3 light_direction = light_location - vertex_world_location;
        float diffuse = normal_dot_sat(vertex_normal, light_direction);
        vec3 diffuse_color = light_attenuation * diffuse * light_color;
        final_color += diffuse_color;

        // Specular
//        vec3 view_direction = view_location - vertex_world_location;
//        vec3 reflect_direction = reflect(-normalize(light_direction), normalize(vertex_normal));
//        float spec_dot = normal_dot_sat(view_direction, reflect_direction);
//        vec3 specular = specual_strength * pow(spec_dot, 32) * light_color * light_attenuation;

        vec3 view_direction = view_location - vertex_world_location;
        vec3 halfway = light_direction + view_direction;
        float blinn = normal_dot_sat(vertex_normal, halfway);
        vec3 specular = specual_strength * pow(blinn, 32) * light_color * light_attenuation;

        final_color += specular;
    }

    OUT.Color = Color * vec4(final_color, 1.0);

    gl_Position = projection * view_rotation * view_translation * vec4(vertex_world_location, 1.0);

}