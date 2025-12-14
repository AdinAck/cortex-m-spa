use cortex_m_spa_model::{Configuration, model};

fn main() {
    for variant in [Configuration::m0(), Configuration::m4()] {
        println!("=== Variant: {variant:?} ===");
        proto_hal_model::validate(&model(variant));
    }
}
