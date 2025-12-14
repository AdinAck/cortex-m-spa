use derive_more::Deref;
use proto_hal_model::{Register, model::PeripheralEntry};

use crate::nvic::{self, ipr::ip::ip};

pub mod ip;

#[derive(Clone, Copy, Deref)]
pub struct Instance(u8);

impl Instance {
    fn ident(&self) -> String {
        format!("ipr{}", self.0)
    }

    fn offset(&self) -> u32 {
        0x400 + 4 * self.0 as u32
    }

    pub fn iter(config: nvic::Configuration) -> impl Iterator<Item = Self> {
        match config {
            nvic::Configuration::M0 => 0..8,
            nvic::Configuration::M4 => 0..60,
        }
        .map(Self)
    }
}

pub fn ipr<'cx>(nvic: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut ipr = nvic.add_register(
        Register::new(instance.ident(), instance.offset())
            .reset(0)
            .partial(),
    );

    for x in 0..4 {
        ip(&mut ipr, *instance * 4 + x);
    }
}
