fn main_image(frag_coord: vec4<f32>) -> vec4<f32> {
    let uv: vec2<f32> = uv_centered(frag_coord.xy);
    return vec4(uv, .0, 1.);
}
