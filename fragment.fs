#version 330 core
out vec4 FragColor;
in vec4 vertex_color;
in vec4 opos_o;

void main() {
	FragColor = vertex_color + opos_o;
}