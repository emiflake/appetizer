#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in vec3 WorldPos;
in vec3 Normal;
in mat3 TBN;
in vec3 TangentLightPos;
in vec3 TangentViewPos;
in vec3 TangentFragPos;

uniform sampler2D tex_base;
uniform sampler2D tex_ao;
uniform sampler2D tex_rough;
uniform sampler2D tex_metal;
uniform sampler2D tex_normal;
uniform sampler2D tex_bump;

uniform vec3 light_pos;
uniform vec3 light_color;
uniform vec3 camera_pos;

const float PI = 3.14159265359;

float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a = roughness*roughness;
    float a2 = a*a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return nom / max(denom, 0.001); // prevent divide by zero for roughness=0.0 and NdotH=1.0
}

float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float nom   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}

vec3 fresnelSchlick(float cosTheta, vec3 F0)
{
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}

void main() {
	vec3 albedo = pow(texture(tex_base, TexCoord).rgb, vec3(2.2));
	float roughness = texture(tex_rough, TexCoord).r;
	float metalness = texture(tex_metal, TexCoord).r;
	float ao = texture(tex_ao, TexCoord).r;
	vec3 normal = texture(tex_normal, TexCoord).rgb;
	normal = normalize(normal * 2.0 - 1.0);
	normal = normalize(TBN * normal);

	vec3 N = normalize(Normal * (normal * 2.0 - 1.0));
	vec3 V = normalize(camera_pos - WorldPos);

	vec3 F0 = vec3(0.04);
	F0 = mix(F0, albedo, metalness);

	vec3 Lo = vec3(0.0);
	// For each light: 
	vec3 L = normalize(light_pos - WorldPos);
	vec3 H = normalize(V + L);
	float distance = length(light_pos - WorldPos);
	float attenuation = 1.0 / (distance * distance);
	vec3 radiance = light_color * attenuation;

	float NDF = DistributionGGX(N, H, roughness);
	float G = GeometrySmith(N, V, L, roughness);
	vec3 F = fresnelSchlick(max(dot(H, V), 0.0), F0);
	
	vec3 kS = F;
	vec3 kD = vec3(1.0) - kS;
	kD *= 1.0 - metalness;

	vec3 numerator = NDF * G * F;
	float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
	vec3 specular = numerator / max(denominator, 0.001);
	
	float NdotL = max(dot(N, L), 0.0);
	Lo += (kD * albedo / PI + specular) * radiance * NdotL;
	// End for each light

	vec3 ambient = vec3(0.03) * albedo * 1.0;
	vec3 color = ambient + Lo;
	
	// HDR tonemapping
	color = color / (color + vec3(1.0));
	// gamma correct
	color = pow(color, vec3(1.0 / 2.2)); 

	FragColor = vec4(color, 1.0);
}
