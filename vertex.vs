#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;
out vec4 FragColor;
out vec2 TexCoord;
out vec3 FragPos;
out vec3 Normal;

uniform vec3 camera_pos;
uniform vec3 camera_tgt;
uniform vec3 camera_up;

uniform mat4 projection;
uniform mat4 model;

mat4 lookAt(vec3 eye, vec3 tgt, vec3 up)
{
	vec3 zaxis = normalize(eye - tgt);
	vec3 xaxis = normalize(cross(up, zaxis));
	vec3 yaxis = cross(zaxis, xaxis);

	return mat4(
		vec4(xaxis.x         , yaxis.x         , zaxis.x         , 0),
		vec4(xaxis.y         , yaxis.y         , zaxis.y         , 0),
		vec4(xaxis.z         , yaxis.z         , zaxis.z         , 0),
		vec4(-dot(xaxis, eye), -dot(yaxis, eye), -dot(zaxis, eye), 1)
	);
}

void main() {
	TexCoord = aTexCoord;
	Normal = aNormal;
	FragPos = vec3(aPos);

	mat4 camera = lookAt(camera_pos, camera_tgt, camera_up);

	gl_Position = projection * camera * model * vec4(aPos, 1.0);
}