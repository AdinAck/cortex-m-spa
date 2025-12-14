use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn active<'cx>(iabr: &mut RegisterEntry<'cx>, x: u8) {
    let mut active = iabr.add_read_field(Field::new(format!("active{x}"), x % 32, 1));

    active.add_variant(Variant::new("Inactive", 0));
    active.add_variant(Variant::new("Active", 1));
}
