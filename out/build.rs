use model::{compose, Configuration};

fn main() {
    let variant = if cfg!(feature = "m0") {
        Configuration::m0()
    } else if cfg!(feature = "m4") {
        Configuration::m4()
    } else {
        Configuration::default()
    };

    phb::render(&compose(variant));

    println!("cargo::rerun-if-changed=../model");
}
