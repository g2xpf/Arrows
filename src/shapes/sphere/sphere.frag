#version 330

in vec4 vColor;

out vec4 color;

void main(){
    color = (length(vColor.xyz) < 0.001) ? vec4(1., 1., 0., 1.) : vec4(0., 0., 1., 1.);
}
