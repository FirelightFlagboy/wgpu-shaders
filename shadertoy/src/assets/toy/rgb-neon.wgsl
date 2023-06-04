// https://iquilezles.org/articles/palettes/
fn palette(time: f32) -> vec3<f32> {
    let a = vec3(.5, .5, .5);
    let b = vec3(.5, .5, .5);
    let c = vec3(1., 1., 1.);
    let d = vec3(0.263, 0.416, 0.557);
    return a + b * cos(6.28318 * (c * time + d));
}

fn main_image(frag_coord: vec4<f32>) -> vec4<f32> {
    let uv: vec2<f32> = uv_ratioed(frag_coord.xy);

    let l = length(uv);
    let d: f32 = smoothstep(.0, .1, abs(sin(l * 8. + u.time) / 8.));

    let col = palette(l + u.time) * 0.02 / d;
    return vec4(col, 1.);
}
