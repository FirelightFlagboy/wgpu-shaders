// https://iquilezles.org/articles/palettes/
fn palette(time: f32) -> vec3<f32> {
    let a = vec3(.5, .5, .5);
    let b = vec3(.5, .5, .5);
    let c = vec3(1., 1., 1.);
    let d = vec3(0.263, 0.416, 0.557);
    return a + b * cos(6.28318 * (c * time + d));
}

fn main_image(frag_coord: vec4<f32>) -> vec4<f32> {
    let time: f32 = u.time;
    let uv0: vec2<f32> = uv_ratioed(frag_coord.xy);
    let d0 = length(uv0);
    var uv = uv0;

    var final_color = vec3(.0);

    for (var i = 0; i < 3; i++) {
        uv = fract(uv * 2.) - .5;
        let l = length(uv) * exp(-d0);
        let col = palette(d0 + f32(i) * .4 + time * .4);

        let d: f32 = smoothstep(.0, .1, abs(sin(l * 8. + time) / 8.));

        final_color += col * pow(0.01 / d, 1.2);
    }


    return vec4(final_color, 1.);
}
