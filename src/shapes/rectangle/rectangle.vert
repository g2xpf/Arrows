#version 330

in vec3 position;
in vec3 color;

uniform vec3 coord;
uniform float width;
uniform float height;
uniform float angle;

out vec4 vColor;

mat2 rotMat(float angle){
    return mat2(cos(angle), sin(angle), -sin(angle), cos(angle));
}


void main(){
    gl_Position = vec4(coord.xy + rotMat(angle) * (vec2(width, height) * position.xy), 0.0, 1.0);
    vColor = vec4(color, 1.0);
}
