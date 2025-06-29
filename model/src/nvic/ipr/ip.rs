use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(x: u8) -> Field {
    Field::new(
        format!("ip{x}"),
        (x % 4) * 8 + 4,
        4,
        Access::read_write(Numericity::enumerated(
            (0..16).map(|i| Variant::new(format!("P{i}"), i)),
        )),
    )
    .reset("P0")
}
