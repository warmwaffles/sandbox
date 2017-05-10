#version 330 core

in vec3 a_position;
in vec4 a_color;

uniform Transform {
    mat4 u_combined;
};

out vec4 v_color;

void main() {
    v_color = a_color;
    gl_Position = u_combined * vec4(a_position, 1.0);
}
