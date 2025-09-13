#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying vec2 v_uv;

void main() {
    gl_Position = vec4(position, 1.0); // no camera transform!
    v_uv = texcoord;
}