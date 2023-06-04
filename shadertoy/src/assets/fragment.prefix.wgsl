struct Uniforms {
    // cursor: vec2<f32>,
    // mouse_down: u32,
    // mouse_press: vec2<f32>,
    // mouse_release: vec2<f32>,
    resolution: vec2<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> u: Uniforms;

fn texture(texture: texture_2d<f32>, spl: sampler, uv: vec2<f32>) -> vec4<f32> {
    return textureSample(texture, spl, vec2(uv.x, 1.0 - uv.y));
}

fn uv_centered(coord: vec2<f32>) -> vec2<f32> {
    return ((coord / u.resolution) - .5) * 2.;
}

fn uv_ratioed(coord: vec2<f32>) -> vec2<f32> {
    return (coord * 2. - u.resolution.xy) / u.resolution.y;
}
