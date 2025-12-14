use model::Configuration;
use proto_hal_macros::generate_macros;

generate_macros!({
    if cfg!(feature = "m0") {
        Configuration::m0()
    } else if cfg!(feature = "m4") {
        Configuration::m4()
    } else {
        Configuration::default()
    }
});
