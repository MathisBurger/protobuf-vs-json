extern crate prost_build;
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/spec.proto"], &["src/"])?;
    Ok(())
}
