use proto_hal_model::Model;

pub mod nvic;

use nvic::nvic;

#[derive(Debug, Default)]
pub struct Configuration {
    nvic: Option<nvic::Configuration>,
}

impl Configuration {
    pub fn m0() -> Self {
        Self {
            nvic: Some(nvic::Configuration::M0),
        }
    }

    pub fn m4() -> Self {
        Self {
            nvic: Some(nvic::Configuration::M4),
        }
    }
}

pub fn model(config: Configuration) -> Model {
    let mut model = Model::new();

    if let Some(nvic_config) = config.nvic {
        nvic(&mut model, nvic_config);
    }

    model
}
