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
    float u_TexDimensionsX;
    float u_TexDimensionsY;
    float u_Zoom;
    vec2 u_CamPos;
    vec2 u_ScreenSize;
};


float HexDist(vec2 p) {
	p = abs(p);
    
    float c = dot(p, normalize(vec2(1,1.73)));
    c = max(c, p.x);
    
    return c;
}

vec4 HexCoords(vec2 uv, out vec2 otherCoord) {
	vec2 r = vec2(1, 1.73);
    vec2 h = r*.5;
    
    vec2 a = mod(uv, r)-h;
    vec2 b = mod(uv-h, r)-h;
    
    vec2 gv = dot(a, a) < dot(b,b) ? a : b;
    
    float x = atan(gv.x, gv.y);
    float y = .5-HexDist(gv);

    otherCoord.x = x;
    otherCoord.y = y;

    vec2 id = uv-gv;
    return vec4(gv.x, gv.y, id.x,id.y);
}

vec2 cube_to_axial(vec3 cube) {
    float q = cube.x;
    float r = cube.z;
    return vec2(q, r);
}

vec3 axial_to_cube(vec2 hex) {
    float x = hex.x;
    float z = hex.y;
    float y = -x-z;
    return vec3(x, y, z);
}

vec3 cube_round(vec3 cube) {
    float rx = round(cube.x);
    float ry = round(cube.y);
    float rz = round(cube.z);

    float x_diff = abs(rx - cube.x);
    float y_diff = abs(ry - cube.y);
    float z_diff = abs(rz - cube.z);

    if (x_diff > y_diff && x_diff > z_diff)
        rx = -ry-rz;
    else if (y_diff > z_diff)
        ry = -rx-rz;
    else
        rz = -rx-ry;

    return vec3(rx, ry, rz);
}

vec2 hex_round(vec2 hex) {
    return cube_to_axial(cube_round(axial_to_cube(hex)));
}

const float size = 4.0;

vec2 pixel_to_pointy_hex(vec2 point) {
    float q = (sqrt(3)/3 * point.x  -  1./3 * point.y) / size;
    float r = (                        2./3 * point.y) / size;
    return hex_round(vec2(q, r));
}


void main() {
    //Target0 = texture(t_Texture, v_Uv) * v_Color * u_Rate;

    vec4 col = vec4(0, 0, 0, 1);
	vec2 uv = v_Uv;
    uv.x = min(1.0, uv.x);
    vec2 scr_pos = uv * u_ScreenSize;
    scr_pos += u_CamPos;
    vec2 hex_coord = pixel_to_pointy_hex(scr_pos);
    vec4 terrain = texture(t_Texture, hex_coord / vec2(u_TexDimensionsX, u_TexDimensionsY));
    col.xyz = terrain.xyz;


    // v_Uv *= 20.0;
    // vec2 uv = v_Uv;
    // uv.y = 1.0 - uv.y;
    // uv *= u_Zoom;
    // uv += u_CamPos;

    // vec2 otherCoord;
    // vec4 hexCoord = HexCoords(uv, otherCoord);
    // vec4 terrain = texture(t_Texture, vec2(hexCoord.z, hexCoord.w) / vec2(u_TexDimensionsX, u_TexDimensionsY));

    // // vec2 r = vec2(1, 1.73);
    // // vec2 h = r * 0.5;

    // // vec2 a = mod(uv, r) - h;
    // // vec2 b = mod(uv - h, r) - h;

    // // vec2 gv = dot(a, a) < dot(b, b) ? a : b;

    // float c = smoothstep(0.05, 0.1, otherCoord.y);
    // if(otherCoord.y < 0.05){
    //     col.rgb = vec3(1.0, 1.0, 1.0);
    // } else {
    //     col.xyz = terrain.xyz;
    //     // col.rg = hexCoord.zw / 15.0;
    // }
    //col.rg = otherCoord;

    Target0 = col;
    // Target0 = vec4(v_Uv, 0.0, 1.0);
    // Target0 = vec4(0.0, 1.0, 0.0, 1.0);
}