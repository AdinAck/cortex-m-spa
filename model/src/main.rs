use std::process::ExitCode;

use cortex_m_spa_model::compose;
use enum_iterator::all;

fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;

    for device in all() {
        println!("=== Variant: {device:?} ===");
        if ExitCode::FAILURE == phm::validate(compose(Some(device))) {
            exit_code = ExitCode::FAILURE;
        }
    }

    exit_code
}
