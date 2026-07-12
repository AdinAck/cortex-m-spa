fn main() {
    let path = cfg_select! {
        feature = "m0" => "../model/src/m0.phm",
        feature = "m4" => "../model/src/m4.phm",
        _ => panic!("a device must be specified: enable feature `m0` or `m4`"),
    };

    phb::model::render(path);
}
