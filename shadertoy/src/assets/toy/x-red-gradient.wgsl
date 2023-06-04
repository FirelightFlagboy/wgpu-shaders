fn main_image(frag_coord: vec4<f32>) -> vec4<f32> {
    let uv: vec2<f32> = frag_coord.xy / u.resolution;
    return vec4(uv.x, .0, .0, 1.);
}
