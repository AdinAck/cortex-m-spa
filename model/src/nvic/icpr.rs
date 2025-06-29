pub mod clrpend;

use proto_hal_build::ir::structures::register::Register;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Instance {
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
}

impl Instance {
    fn ident(&self) -> String {
        match self {
            Self::I1 => "icpr1",
            Self::I2 => "icpr2",
            Self::I3 => "icpr3",
            Self::I4 => "icpr4",
            Self::I5 => "icpr5",
            Self::I6 => "icpr6",
            Self::I7 => "icpr7",
            Self::I8 => "icpr8",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        0x280 + 4 * *self as u32
    }
}

pub fn generate(instance: Instance) -> Register {
    Register::new(
        instance.ident(),
        instance.offset(),
        match instance {
            Instance::I8 => 0..16,
            _ => 0..32,
        }
        .map(|x| clrpend::generate((instance as u8) * 32 + x)),
    )
}
