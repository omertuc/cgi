#version 330 core

out vec4 Color;

in VS_OUTPUT {
    vec4 Color;
} IN;

void main()
{
    Color = IN.Color;
}