pub mod setpend;

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
            Self::I1 => "ispr1",
            Self::I2 => "ispr2",
            Self::I3 => "ispr3",
            Self::I4 => "ispr4",
            Self::I5 => "ispr5",
            Self::I6 => "ispr6",
            Self::I7 => "ispr7",
            Self::I8 => "ispr8",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        0x200 + 4 * *self as u32
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
        .map(|x| setpend::generate((instance as u8) * 32 + x)),
    )
}
