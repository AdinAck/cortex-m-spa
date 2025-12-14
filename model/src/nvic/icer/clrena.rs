use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn clrena<'cx>(icer: &mut RegisterEntry<'cx>, x: u8) {
    let mut clrena = icer.add_read_write_field(Field::new(format!("clrena{x}"), x % 32, 1));

    clrena.add_read_variant(Variant::new("Disabled", 0));
    clrena.add_read_variant(Variant::new("Enabled", 1));

    clrena.add_write_variant(Variant::new("Noop", 0).inert());
    clrena.add_write_variant(Variant::new("Disable", 1));
}
