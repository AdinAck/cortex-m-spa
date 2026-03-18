use cortex_m_spa_model::{Configuration, model};

fn main() -> phm::Result<()> {
    for variant in [Configuration::m0(), Configuration::m4()] {
        println!("=== Variant: {variant:?} ===");
        phm::validate(&model(variant)?);
    }

    Ok(())
}
