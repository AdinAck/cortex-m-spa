use proto_hal_macros::generate_macros;

generate_macros!(::proto_hal_build::model::compose(cfg_select! {
    feature = "m0" => concat!(env!("CARGO_MANIFEST_DIR"), "/../model/devices/m0.phm"),
    feature = "m4" => concat!(env!("CARGO_MANIFEST_DIR"), "/../model/devices/m4.phm"),
    _ => panic!("a device must be specified: enable feature `m0` or `m4`"),
}));
