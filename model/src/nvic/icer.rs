pub mod clrena;

use clrena::clrena;
use proto_hal_model::{Register, model::PeripheralEntry};

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
            Self::I1 => "icer1",
            Self::I2 => "icer2",
            Self::I3 => "icer3",
            Self::I4 => "icer4",
            Self::I5 => "icer5",
            Self::I6 => "icer6",
            Self::I7 => "icer7",
            Self::I8 => "icer8",
        }
    }

    fn offset(&self) -> u32 {
        0x180 + 4 * *self as u32
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

pub fn icer<'cx>(nvic: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut icer = nvic.add_register(Register::new(instance.ident(), instance.offset()).leaky());

    for x in match instance {
        Instance::I8 => 0..16,
        _ => 0..32,
    } {
        clrena(&mut icer, instance as u8 * 32 + x);
    }
}
