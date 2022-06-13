struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct MyMat {
    brightness: f32;
    progression: f32;
};

[[group(1), binding(0)]]
var<uniform> uniform_data: MyMat;

[[group(1), binding(1)]]
var texture: texture_2d<f32>;

[[group(1), binding(2)]]
var texture_sampler: sampler;

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var output_color = vec4<f32>(input.uv, 0.0, uniform_data.brightness);
    output_color = output_color * textureSample(texture, texture_sampler, input.uv);
    output_color = output_color * uniform_data.brightness;

    return output_color;
}
