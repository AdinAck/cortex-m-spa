fn main() -> Result<(), String> {
    proto_hal_build::codegen::generate(model::generate);

    Ok(())
}
