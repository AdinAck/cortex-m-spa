pub mod ip;

use proto_hal_build::ir::structures::register::Register;

#[derive(Clone, Copy)]
pub struct Instance(u8);

impl Instance {
    fn new(i: u8) -> Option<Self> {
        if i < 60 { Some(Self(i)) } else { None }
    }

    fn ident(&self) -> String {
        format!("ipr{}", self.0)
    }

    fn offset(&self) -> u32 {
        0x400 + 4 * self.0 as u32
    }
}

pub fn generate(instance: Instance) -> Register {
    Register::new(
        instance.ident(),
        instance.offset(),
        (0..4).map(|x| ip::generate((instance.0 as u8) * 4 + x)),
    )
    .reset(0)
}

pub fn generate_group() -> impl Iterator<Item = Register> {
    (0..60).map(|i| generate(Instance::new(i).unwrap()))
}
