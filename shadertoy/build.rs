use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    let raw_root_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = raw_root_dir.to_string_lossy();
    let out_dir = env::var_os("OUT_DIR").expect("Missing out dir env variable");
    // let out_dir = "src";
    let dest_path = Path::new(&out_dir).join("toy.rs");

    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dest_path)
        .unwrap();
    let mut buf = BufWriter::new(file);

    buf.write_all(b"pub const SHADERS: &'static [(&'static str, &'static str)] = &[")
        .unwrap();
    for path in glob::glob("src/assets/toy/*.wgsl").expect("Cannot glob") {
        let path = path.unwrap();
        let filename = path.file_name().unwrap().to_string_lossy();
        write!(
            buf,
            "(\"{filename}\", std::include_str!(\"{root_dir}/src/assets/toy/{filename}\")),"
        )
        .unwrap();
    }
    buf.write_all(b"];").unwrap();
    buf.flush().unwrap();

    println!("cargo:rerun-if-changed=src/assets/toy/*.wgsl");
    println!("cargo:rerun-if-changed=build.rs");
}
