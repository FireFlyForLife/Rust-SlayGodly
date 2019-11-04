#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

layout (std140) uniform Dim {
    float u_Rate;
};

void main() {
    //Target0 = texture(t_Texture, v_Uv) * v_Color * u_Rate;

    vec4 col = vec4(0, 0, 0, 1);
				
    // v_Uv *= 20.0;
    vec2 uv = (v_Uv) * 5.0;

    vec2 r = vec2(1, 1.73);
    vec2 h = r * 0.5;

    vec2 a = mod(uv, r) - h;
    vec2 b = mod(uv - h, r) - h;

    vec2 gv = dot(a, a) < dot(b, b) ? a : b;

    col.rg = gv;

    Target0 = col;
    // Target0 = vec4(v_Uv, 0.0, 1.0);
    // Target0 = vec4(0.0, 1.0, 0.0, 1.0);
}