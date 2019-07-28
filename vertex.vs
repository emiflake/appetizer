#version 330 core
layout (location = 0) in vec3 aPos;
out vec4 vertex_color; // specify a color output to the fragment shader
out vec4 opos_o;

void main() {
	vec4 opos = vec4(aPos.x, aPos.y, aPos.z, 1.0);
	gl_Position = opos;
	opos_o = opos;
	vertex_color = vec4(1.0, 0.5, 0.3, 1.0);
}