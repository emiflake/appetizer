#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;
layout (location = 3) in vec3 tangent;
layout (location = 4) in vec3 bitangent;

out vec2 TexCoord;
out vec3 WorldPos;
out vec3 Normal;
out mat3 TBN;
out vec3 TangentLightPos;
out vec3 TangentViewPos;
out vec3 TangentFragPos;

uniform mat4 camera;
uniform mat4 projection;
uniform mat4 model;

void main() {
	TexCoord = uv;
	WorldPos = vec3(model * vec4(position, 1.0));
	Normal = mat3(model) * normal;

	vec3 T = normalize(mat3(model) * tangent);
    vec3 B = normalize(mat3(model) * bitangent);
    vec3 N = normalize(mat3(model) * normal);
    TBN = transpose(mat3(T, B, N));
	
	gl_Position = projection * camera * vec4(WorldPos, 1.0);
}