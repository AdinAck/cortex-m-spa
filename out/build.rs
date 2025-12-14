use model::{model, Configuration};

fn main() -> Result<(), String> {
    let variant = if cfg!(feature = "m0") {
        Configuration::m0()
    } else if cfg!(feature = "m4") {
        Configuration::m4()
    } else {
        Configuration::default()
    };

    proto_hal_build::render(&model(variant));

    println!("cargo::rerun-if-changed=../model");

    Ok(())
}
