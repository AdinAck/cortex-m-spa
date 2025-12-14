use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn clrpend<'cx>(icpr: &mut RegisterEntry<'cx>, x: u8) {
    let mut clrpend = icpr.add_read_write_field(Field::new(format!("clrpend{x}"), x % 32, 1));

    clrpend.add_read_variant(Variant::new("Idle", 0));
    clrpend.add_read_variant(Variant::new("Pending", 1));

    clrpend.add_write_variant(Variant::new("Noop", 0).inert());
    clrpend.add_write_variant(Variant::new("Unpend", 1));
}
