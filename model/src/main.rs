use cortex_m_spa_model::compose;
use enum_iterator::all;

fn main() {
    for device in all() {
        println!("=== Variant: {device:?} ===");
        phm::validate(compose(Some(device)));
    }
}
