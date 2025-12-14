use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn setpend<'cx>(ispr: &mut RegisterEntry<'cx>, x: u8) {
    let mut setpend = ispr.add_read_write_field(Field::new(format!("setpend{x}"), x % 32, 1));

    setpend.add_read_variant(Variant::new("Idle", 0));
    setpend.add_read_variant(Variant::new("Pending", 1));

    setpend.add_write_variant(Variant::new("Noop", 0).inert());
    setpend.add_write_variant(Variant::new("Pend", 1));
}
