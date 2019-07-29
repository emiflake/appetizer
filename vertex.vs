#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;
out vec4 FragColor;
out vec2 TexCoord;
out vec3 FragPos;
out vec3 Normal;

uniform mat4 camera;
uniform mat4 projection;


void main() {
	TexCoord = aTexCoord;
	Normal = aNormal;
	FragPos = vec3(aPos);
	gl_Position = projection * camera * vec4(aPos, 1.0);
}