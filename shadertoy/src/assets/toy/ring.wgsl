fn main_image(frag_coord: vec4<f32>) -> vec4<f32> {
    let uv: vec2<f32> = uv_ratioed(frag_coord.xy);

    let d = smoothstep(.0, .1, abs(length(uv) - .5));
    return vec4(d, d, d, 1.);
}
