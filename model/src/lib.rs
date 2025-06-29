use proto_hal_build::ir::{structures::hal::Hal, utils::diagnostic::Diagnostics};

#[cfg(feature = "m4")] // temporary // why is this temporary?
pub mod nvic;

pub fn generate() -> Result<Hal, Diagnostics> {
    let hal = Hal::new([
        #[cfg(feature = "m4")]
        nvic::generate(),
    ]);

    let diagnostics = hal.validate();

    if !diagnostics.is_empty() {
        Err(diagnostics)?
    }

    Ok(hal)
}
