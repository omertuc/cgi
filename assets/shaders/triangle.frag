#version 330 core

out vec4 Color;

in VS_OUTPUT {
    vec4 Color;
    vec3 Normal;
    vec3 WorldCoords;
} IN;

// Camera
uniform vec3 view_location;

// Spot lights
#define MAX_LIGHTS 330u
uniform uint lights_count;
uniform vec3[MAX_LIGHTS] light_locations;
uniform vec3[MAX_LIGHTS] light_colors;
uniform float[MAX_LIGHTS] light_radiuses;

// Ambient lighting
float ambient_strength = 0.1;
vec3 ambient_color = ambient_strength * vec3(1.0, 1.0, 1.0);

// Specular lighting
float specular_strength = 100;
float specular_roughness = 0.9;

float normal_dot_sat(vec3 v1, vec3 v2) {
    return max(dot(normalize(v1), normalize(v2)), 0.0);
}

void main()
{
    vec3 vertex_world_location = IN.WorldCoords;
    vec3 vertex_normal = IN.Normal;

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
        vec3 view_direction = view_location - vertex_world_location;
        vec3 halfway = light_direction + view_direction;
        float angle = acos(dot(normalize(vertex_normal), normalize(halfway)));
        float exponent = (angle / specular_roughness);
        float term = exp(-(exponent * exponent));
        vec3 specular = specular_strength * pow(term, 64) * light_color * light_attenuation;

        final_color += specular;
    }

    Color = IN.Color * vec4(final_color, 1.0);
}
