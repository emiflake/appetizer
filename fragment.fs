#version 330 core
out vec4 FragColor;
uniform float our_color;
in vec2 TexCoord;
in vec3 Normal;  

uniform sampler2D ourTexture;
uniform vec3 light_pos;  
uniform vec3 light_color;  

in vec3 FragPos;

void main() {
	// Ensure normal is actually a normal, lol
	vec3 norm = normalize(Normal);

	// FragPos (position of hit) to light
	vec3 light_dir = normalize(light_pos - FragPos);  
	// Check how 'close' the normal and the light is
	// the closer, the more light it has
	float diff = max(dot(norm, light_dir), 0.0);
	vec3 diffuse = diff * light_color;
	vec3 result = diffuse * vec3(texture(ourTexture, TexCoord));
	FragColor = vec4(result, 1.0);
}