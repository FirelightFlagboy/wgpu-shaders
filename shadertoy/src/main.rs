fn main() {
    tokio::runtime::Runtime::new()
        .expect("Cannot create tokio runtime")
        .block_on(shadertoy::run());
}
