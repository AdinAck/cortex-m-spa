use proto_hal_model::{Register, model::PeripheralEntry};

use crate::nvic::iabr::active::active;

pub mod active;

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
            Self::I1 => "iabr1",
            Self::I2 => "iabr2",
            Self::I3 => "iabr3",
            Self::I4 => "iabr4",
            Self::I5 => "iabr5",
            Self::I6 => "iabr6",
            Self::I7 => "iabr7",
            Self::I8 => "iabr8",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        0x300 + 4 * *self as u32
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

pub fn iabr<'cx>(nvic: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut iabr = nvic.add_register(Register::new(instance.ident(), instance.offset()));

    for x in match instance {
        Instance::I8 => 0..16,
        _ => 0..32,
    } {
        active(&mut iabr, instance as u8 * 32 + x);
    }
}
