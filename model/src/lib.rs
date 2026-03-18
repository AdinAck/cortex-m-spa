use phm::Model;

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

pub fn model(config: Configuration) -> phm::Result<Model> {
    let mut model = Model::new();

    if let Some(nvic_config) = config.nvic {
        nvic(&mut model, nvic_config);
    }

    Ok(model)
}
