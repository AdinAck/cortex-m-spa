use cortex_m_spa_model::{Configuration, compose};

fn main() {
    for variant in [Configuration::m0(), Configuration::m4()] {
        println!("=== Variant: {variant:?} ===");
        phm::validate(compose(variant));
    }
}
