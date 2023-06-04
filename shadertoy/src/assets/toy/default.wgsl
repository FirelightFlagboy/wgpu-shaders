fn main_image(frag_coord: vec4<f32>) -> vec4<f32> {
    let uv: vec2<f32> = frag_coord.xy / u.resolution;
    let color: vec3<f32> = 0.5 + 0.5 * cos(u.time + uv.xyx + vec3(0.0, 2.0, 4.0));
    return vec4(color, 1.0);
}
