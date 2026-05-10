#[allow(unused)]
use model::Device;
use proto_hal_macros::generate_macros;

generate_macros!({
    cfg_select! {
        feature = "m0" => Some(Device::M0),
        feature = "m4" => Some(Device::M4),
        _ => None,
    }
});
