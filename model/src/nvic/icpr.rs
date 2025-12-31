use proto_hal_model::{Register, model::PeripheralEntry};

use crate::nvic::icpr::clrpend::clrpend;

pub mod clrpend;

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

pub fn icpr<'cx>(nvic: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut icpr = nvic.add_register(Register::new(instance.ident(), instance.offset()).leaky());

    for x in match instance {
        Instance::I8 => 0..16,
        _ => 0..32,
    } {
        clrpend(&mut icpr, instance as u8 * 32 + x);
    }
}
