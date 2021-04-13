#version 330 core

layout (location = 0) in vec4 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec3 Normal;

out VS_OUTPUT {
    vec4 Color;
} OUT;

#define M_PI 3.1415926535897932384626433832795

uniform mat4 model_rotation;
uniform mat4 model_translation;
uniform mat4 view_translation;
uniform mat4 view_rotation;
uniform mat4 projection;

vec4 lightPosition = vec4(20.0, 20.0, 2.0, 1.0);
vec4 lightColor = vec4(1.0, 1.0, 1.0, 1.0);
float lightMaxDistance = 15.0;

// Ambient lightning
float ambientStrength = 0.1;
vec4 ambientColor = vec4((ambientStrength * vec3(1.0, 1.0, 1.0)).xyz, 1.0);

void main()
{
    vec4 worldCoordinates = model_translation * model_rotation * Position;

    vec4 objectRotatedNormal = model_rotation * vec4(Normal.xyz, 1.0);
    float lightDistance = distance(lightPosition, worldCoordinates);
    float lightPower = 1.0 - (min(lightDistance, lightMaxDistance) / lightMaxDistance);
    vec4 lightDirection = vec4(normalize(lightPosition.xyz - worldCoordinates.xyz).xyz, 1.0);
    float diffuse = max(dot(objectRotatedNormal, lightDirection), 0.0);
    vec4 finalSpotColor = lightPower * diffuse * lightColor;

    vec4 lightDistanceEffect = finalSpotColor * lightPower;


    OUT.Color = (lightDistanceEffect + ambientColor) * Color;

    gl_Position = vec4(projection * view_translation * view_rotation * worldCoordinates);
}