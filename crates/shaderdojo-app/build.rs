
fn main() {
    println!("!cargo:rerun-if-changed=src/shaders/hlsl/sample.hlsl");
    std::fs::copy(
        "src/shaders/hlsl/sample.hlsl",
        std::env::var("OUT_DIR").unwrap() + "/../../../sample.hlsl",
    )
    .expect("Copy");
}