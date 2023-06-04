fn main_image(frag_coord: vec4<f32>) -> vec4<f32> {
    let uv: vec2<f32> = uv_ratioed(frag_coord.xy);

    let d: f32 = smoothstep(.0, .1, abs(sin(length(uv) * 8. + u.time) / 8.));

    let col = vec3(1., 2., 3.) * 0.02 / d;

    return vec4(col, 1.);
}
