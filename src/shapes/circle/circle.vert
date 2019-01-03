#version 330

in vec3 position;
in vec3 color;

uniform vec3 coord;

out vec4 vColor;

void main(){
    gl_Position = vec4(position + coord, 1.0);
    vColor = vec4(color, 1.0);
}

