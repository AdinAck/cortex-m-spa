use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn ip<'cx>(ipr: &mut RegisterEntry<'cx>, x: u8) {
    let mut ip = ipr.add_store_field(Field::new(format!("ip{x}"), (x % 4) * 8 + 4, 4));

    for i in 0..16 {
        ip.add_variant(Variant::new(format!("P{i}"), i));
    }
}
