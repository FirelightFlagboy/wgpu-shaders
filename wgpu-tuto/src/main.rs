use wgpu_shaders;

fn main() {
    tokio::runtime::Runtime::new()
        .expect("Can't create tokio async runtime")
        .block_on(wgpu_shaders::run());
}
