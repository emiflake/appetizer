#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;
out vec4 FragColor;
out vec2 TexCoord;
out vec3 FragPos;
out vec3 Normal;

uniform mat4 camera;

uniform mat4 projection;
uniform mat4 model;

void main() {
	TexCoord = uv;
	Normal = normal;
	FragPos = vec3(model * vec4(position, 1.0));

	gl_Position = projection * camera * model * vec4(position, 1.0);
}