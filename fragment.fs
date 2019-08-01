#version 330 core
out vec4 FragColor;
uniform float our_color;
in vec2 TexCoord;
in vec3 Normal;

struct Material {
	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
	float shininess;
};

struct PointLight {
	vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
	
    float constant;
    float linear;
    float quadratic;
};

uniform Material material;
uniform PointLight point_light;

uniform sampler2D our_texture;
uniform vec3 camera_pos;

in vec3 FragPos;


void main() {
	// float dist = length(point_light.position - FragPos);
	// float attenuation = 
	// 	1.0 / (point_light.constant + point_light.linear * dist + 
	// 		point_light.quadratic * (dist * dist));

	// // Ensure normal is actually a normal, lol
	// vec3 norm = normalize(Normal);

	// // FragPos (position of hit) to light
	// vec3 light_dir = normalize(point_light.position - FragPos);

	// // Check how 'close' the normal and the light is
	// // the closer, the more light it has
	// vec3 ambient = point_light.ambient * material.ambient;

	// float diff = max(dot(norm, light_dir), 0.0);
	// vec3 diffuse = diff * point_light.diffuse * material.diffuse;

	// vec3 view_dir = normalize(camera_pos - FragPos);
	// vec3 reflect_dir = reflect(-light_dir, norm);

	// float spec = pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);

	// vec3 specular = (spec * material.specular) * point_light.specular;

	// // ambient  *= attenuation;
	// // diffuse  *= attenuation;
	// // specular *= attenuation;

	// vec3 eqn = ambient + diffuse + specular;
	vec3 eqn = vec3(1.0);

	vec3 result = (eqn) * vec3(texture(our_texture, TexCoord));
	FragColor = vec4(result, 1.0);
	// FragColor = vec4(1.0);
}
