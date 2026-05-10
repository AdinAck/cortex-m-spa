use enum_iterator::Sequence;
use phm::{Composition, diagnostic};

pub mod nvic;

use nvic::nvic;

#[derive(Debug, Clone, Copy, Sequence)]
pub enum Device {
    M0,
    M4,
}

impl Device {
    pub fn apply_to(&self, composition: &mut Composition) {
        nvic(composition, *self);
    }
}

pub fn compose(device: Option<Device>) -> Composition {
    let mut composition = Composition::new();

    if let Some(device) = device {
        device.apply_to(&mut composition);
    } else {
        composition.add_diagnostic(diagnostic::Rank::Error, "A device must be specified.");
    }

    composition
}
