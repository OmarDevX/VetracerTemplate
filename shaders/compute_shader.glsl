#version 460 core

layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;
layout(rgba32f, binding = 0) uniform image2D screen;

uniform float currentTime;

void main()
{
    ivec2 texel_coords = ivec2(gl_GlobalInvocationID.xy);
    vec2 screen_resolution = vec2(imageSize(screen));
    vec2 normalized_coords = (vec2(texel_coords) + vec2(0.5)) / screen_resolution * 2.0 - 1.0;
    float aspect_ratio = screen_resolution.x / screen_resolution.y;

    vec3 initial_rayDir;
    vec3 initial_rayOrigin;
    imageStore(screen, texel_coords, vec4(1.0,0.5,0.5, 1.0));

}
