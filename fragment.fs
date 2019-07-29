#version 330 core
out vec4 FragColor;
uniform float our_color;
in vec2 TexCoord;
in vec3 Normal;

uniform sampler2D ourTexture;
uniform vec3 light_pos;
uniform vec3 light_color;
uniform vec3 camera_pos;

in vec3 FragPos;

void main() {
	// Ensure normal is actually a normal, lol
	vec3 norm = normalize(Normal);

	// FragPos (position of hit) to light
	vec3 light_dir = normalize(light_pos - FragPos);

	// Check how 'close' the normal and the light is
	// the closer, the more light it has
	vec3 ambient = vec3(1.0) * 0.3;

	float diff = max(dot(norm, light_dir), 0.0);
	vec3 diffuse = diff * light_color * 0.7;

	vec3 view_dir = normalize(camera_pos - FragPos);
	vec3 reflect_dir = reflect(-light_dir, norm);

	float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
	float specular_strength = 0.5;

	vec3 specular = specular_strength * spec * light_color;

	vec3 eqn = ambient + diffuse + specular;

	vec3 result = (eqn) * vec3(texture(ourTexture, TexCoord));
	FragColor = vec4(result, 1.0);
}