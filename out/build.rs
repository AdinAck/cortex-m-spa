#[allow(unused)]
use model::{compose, Device};

fn main() {
    let device = cfg_select! {
        feature = "m0" => Some(Device::M0),
        feature = "m4" => Some(Device::M4),
        _ => None,
    };

    phb::render(&compose(device));

    println!("cargo::rerun-if-changed=../model");
}
