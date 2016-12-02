pub static SCREEN_VERTEX_SHADER_SRC: &'static str = r"
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 uvs;

out vec2 UV;

void main() {
    gl_Position = vec4(position.x, position.y, 0.0f, 1.0f);
    UV = uvs;
}
";

pub static SCREEN_FRAGMENT_SHADER_SRC: &'static str = r"
#version 330 core
in vec2 UV;
out vec4 color;

uniform sampler2D screen;

void main() {
    color.rgb = texture(screen, UV).rgb;
    color.a = 1.0;
}
";

pub static SCREEN_SHADER_NAMES: [&'static str; 1] = ["screen"];