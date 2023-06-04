// The mandelbrot set is contain between x: [-2, 0.47] and y: [-1.12, 1.12]
// Here we scale the pixel coord (where x: [0, screen size] and y: [0, screen size]) to the mandelbrot coord
fn scale_in_mandelbrot(xy: vec2<f32>) -> vec2<f32> {
    let max_xy: vec2<f32> = u.resolution.xy;
    return (xy / max_xy) * vec2(2.47, 2.24) + vec2(-2., -1.12);
}

fn above_threshold(uv_pow2: vec2<f32>) -> bool {
    return uv_pow2.x + uv_pow2.y <= 4.;
}

fn hsv2rgb(c: vec3<f32>) -> vec3<f32> {
    let K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, vec3(0.0), vec3(1.0)), c.y);
}

const MAX_ITER: i32 = 4000;

fn mandelbrot(c: vec2<f32>) -> f32 {

    var uv = vec2(.0);
    var uv_pow2 = vec2(.0);
    var i = 0;

    for (; above_threshold(uv_pow2) && i < MAX_ITER ; i++) {
        uv = vec2(uv_pow2.x - uv_pow2.y, 2. * uv.x * uv.y) + c;
        uv_pow2 = pow(uv, vec2(2., 2.));
    }

    if i == MAX_ITER {
        return f32(MAX_ITER);
    } else {
        return f32(i) + 1. - log(log2(uv_pow2.x + uv_pow2.y));
    }
}

fn main_image(frag_coord: vec4<f32>) -> vec4<f32> {

    let uv0 = scale_in_mandelbrot(frag_coord.xy);

    let iter = mandelbrot(uv0);

    if iter >= 1000. {
        return vec4(.0, .0, .0, 1.);
    } else {
        let hue = iter / f32(MAX_ITER);
        let staturation = 1.;
        let value = 1.;
        return vec4(hsv2rgb(vec3(hue, staturation, value)), 1.);
    }
}
