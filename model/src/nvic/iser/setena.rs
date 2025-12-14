use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn setena<'cx>(iser: &mut RegisterEntry<'cx>, x: u8) {
    let mut setena = iser.add_read_write_field(Field::new(format!("setena{x}"), x % 32, 1));

    setena.add_read_variant(Variant::new("Disabled", 0));
    setena.add_read_variant(Variant::new("Enabled", 1));

    setena.add_write_variant(Variant::new("Noop", 0).inert());
    setena.add_write_variant(Variant::new("Enable", 1));
}
