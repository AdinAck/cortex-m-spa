pub mod setena;

use proto_hal_model::{Register, model::PeripheralEntry};
use setena::setena;

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
    fn ident(&self) -> &str {
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
    }

    fn offset(&self) -> u32 {
        0x100 + 4 * *self as u32
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::I1,
            Self::I2,
            Self::I3,
            Self::I3,
            Self::I4,
            Self::I5,
            Self::I6,
            Self::I7,
            Self::I8,
        ]
        .into_iter()
    }
}

pub fn iser<'cx>(nvic: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut iser = nvic.add_register(Register::new(instance.ident(), instance.offset()).partial());

    for x in match instance {
        Instance::I8 => 0..16,
        _ => 0..32,
    } {
        setena(&mut iser, instance as u8 * 32 + x);
    }
}
