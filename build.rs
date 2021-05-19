fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().out_dir("src/proto").compile(
        &["proto/render/v0/renderer_service.proto"],
        &["proto/render/v0"],
    )?;

    Ok(())
}
