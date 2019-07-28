#version 330 core
out vec4 FragColor;
uniform float our_color;
in vec2 TexCoord;

uniform sampler2D ourTexture;


void main() {
	FragColor = texture(ourTexture, TexCoord) + our_color;
	// FragColor = vec4(1.0, our_color, 1.0, 1.0);
}