use enum_iterator::Sequence;
use phm::{Composition, diagnostic};
use proto_hal_build::model::{Diagnostic, Sources, elaborate_sources};

#[derive(Debug, Clone, Copy, Sequence)]
pub enum Device {
    M0,
    M4,
}

impl Device {
    /// The device's model description entry file: `(name, content)`.
    pub fn entry(&self) -> (&'static str, &'static str) {
        match self {
            Self::M0 => ("m0.phm", include_str!("m0.phm")),
            Self::M4 => ("m4.phm", include_str!("m4.phm")),
        }
    }

    /// The device's model description, assembled from the embedded sources.
    pub fn sources(&self) -> Sources {
        Sources::assemble(self.entry(), descriptions())
    }
}

/// The model component descriptions this crate ships, for provision to
/// importing evaluations: `import cortex_m.nvic`.
pub fn descriptions() -> &'static [(&'static str, &'static str)] {
    &[("cortex_m/nvic.phm", include_str!("cortex_m/nvic.phm"))]
}

/// The device model, evaluated from its description.
///
/// Fatal evaluation diagnostics carry over into the composition, so drivers
/// that only see the composition — the gate macros — still surface them;
/// `main` and `out/build.rs` report them fully rendered.
pub fn compose(device: Option<Device>) -> Composition {
    let Some(device) = device else {
        let mut composition = Composition::new();
        composition.add_diagnostic(diagnostic::Rank::Error, "A device must be specified.");
        return composition;
    };

    let sources = device.sources();
    let (mut composition, diagnostics) = elaborate_sources(&sources);

    for diagnostic in &diagnostics {
        if diagnostic.is_fatal() {
            composition.add_diagnostic(
                diagnostic::Rank::Error,
                match diagnostic {
                    Diagnostic::Semantic(semantic) => semantic.message.clone(),
                    Diagnostic::Syntax(error) => format!("{error:?}"),
                },
            );
        }
    }

    composition
}
