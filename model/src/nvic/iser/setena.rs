use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(x: u8) -> Field {
    Field::new(
        format!("setena{x}"),
        x % 32,
        1,
        Access::read_write_asymmetrical(
            Numericity::enumerated([Variant::new("Disabled", 0), Variant::new("Enabled", 1)]),
            Numericity::enumerated([Variant::new("Noop", 0).inert(), Variant::new("Enable", 1)]),
        ),
    )
}
