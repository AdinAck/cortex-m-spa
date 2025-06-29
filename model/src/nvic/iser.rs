pub mod setena;

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
            Self::I1 => "iser1",
            Self::I2 => "iser2",
            Self::I3 => "iser3",
            Self::I4 => "iser4",
            Self::I5 => "iser5",
            Self::I6 => "iser6",
            Self::I7 => "iser7",
            Self::I8 => "iser8",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        0x100 + 4 * *self as u32
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
        .map(|x| setena::generate((instance as u8) * 32 + x)),
    )
}
