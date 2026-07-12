use std::process::ExitCode;

use cortex_m_spa_model::Device;
use enum_iterator::all;
use proto_hal_build::model::{evaluate_sources, report};

fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;

    for device in all::<Device>() {
        println!("=== Variant: {device:?} ===");

        let sources = device.sources();
        let evaluation = evaluate_sources(&sources);

        report(&sources, &evaluation.diagnostics);

        if evaluation.failed() {
            exit_code = ExitCode::FAILURE;
        }
    }

    exit_code
}
