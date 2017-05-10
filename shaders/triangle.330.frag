#version 330 core

in vec4 v_color;

uniform Transform {
    mat4 u_view;
    mat4 u_projection;
};

out vec4 f_color;

void main() {
    f_color = v_color;
}
