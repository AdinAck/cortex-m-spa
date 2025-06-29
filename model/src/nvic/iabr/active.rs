use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(x: u8) -> Field {
    Field::new(
        format!("active{x}"),
        x % 32,
        1,
        Access::read(Numericity::enumerated([
            Variant::new("Inactive", 0),
            Variant::new("Active", 1),
        ])),
    )
}
