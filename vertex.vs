#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;

out vec2 TexCoord;
out vec3 WorldPos;
out vec3 Normal;

uniform mat4 camera;
uniform mat4 projection;
uniform mat4 model;

void main() {
	TexCoord = uv;
	WorldPos = vec3(model * vec4(position, 1.0));
	Normal = mat3(model) * normal;

	gl_Position = projection * camera * vec4(WorldPos, 1.0);
}