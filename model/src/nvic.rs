pub mod iabr;
pub mod icer;
pub mod icpr;
pub mod ipr;
pub mod iser;
pub mod ispr;

use proto_hal_build::ir::structures::peripheral::Peripheral;

/*
references:
- RM0440
- PM0214
- https://developer.arm.com/documentation/100166/0001/Nested-Vectored-Interrupt-Controller/NVIC-programmers-model/Table-of-NVIC-registers?lang=en
*/

pub fn generate() -> Peripheral {
    Peripheral::new("nvic", 0xe000_e000, {
        let mut v = vec![
            // enable
            iser::generate(iser::Instance::I1),
            iser::generate(iser::Instance::I2),
            iser::generate(iser::Instance::I3),
            iser::generate(iser::Instance::I4),
            iser::generate(iser::Instance::I5),
            iser::generate(iser::Instance::I6),
            iser::generate(iser::Instance::I7),
            iser::generate(iser::Instance::I8),
            // disable
            icer::generate(icer::Instance::I1),
            icer::generate(icer::Instance::I2),
            icer::generate(icer::Instance::I3),
            icer::generate(icer::Instance::I4),
            icer::generate(icer::Instance::I5),
            icer::generate(icer::Instance::I6),
            icer::generate(icer::Instance::I7),
            icer::generate(icer::Instance::I8),
            // pend
            ispr::generate(ispr::Instance::I1),
            ispr::generate(ispr::Instance::I2),
            ispr::generate(ispr::Instance::I3),
            ispr::generate(ispr::Instance::I4),
            ispr::generate(ispr::Instance::I5),
            ispr::generate(ispr::Instance::I6),
            ispr::generate(ispr::Instance::I7),
            ispr::generate(ispr::Instance::I8),
            // unpend
            icpr::generate(icpr::Instance::I1),
            icpr::generate(icpr::Instance::I2),
            icpr::generate(icpr::Instance::I3),
            icpr::generate(icpr::Instance::I4),
            icpr::generate(icpr::Instance::I5),
            icpr::generate(icpr::Instance::I6),
            icpr::generate(icpr::Instance::I7),
            icpr::generate(icpr::Instance::I8),
            // active
            iabr::generate(iabr::Instance::I1),
            iabr::generate(iabr::Instance::I2),
            iabr::generate(iabr::Instance::I3),
            iabr::generate(iabr::Instance::I4),
            iabr::generate(iabr::Instance::I5),
            iabr::generate(iabr::Instance::I6),
            iabr::generate(iabr::Instance::I7),
            iabr::generate(iabr::Instance::I8),
        ];

        v.extend(ipr::generate_group());

        v
    })
}
