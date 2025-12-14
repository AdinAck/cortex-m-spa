use proto_hal_model::{Model, Peripheral};

use crate::nvic::{iabr::iabr, icer::icer, icpr::icpr, ipr::ipr, iser::iser, ispr::ispr};

pub mod iabr;
pub mod icer;
pub mod icpr;
pub mod ipr;
pub mod iser;
pub mod ispr;

/*
references:
- RM0440
- RM0490
- PM0214
- PM0223
- https://developer.arm.com/documentation/100166/0001/Nested-Vectored-Interrupt-Controller/NVIC-programmers-model/Table-of-NVIC-registers?lang=en
- https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Address-Map/Nested-Vectored-Interrupt-Controller--NVIC/NVIC-register-support-in-the-SCS?lang=en#BEHEBDBE
*/

#[derive(Debug)]
pub enum Configuration {
    M0,
    M4,
}

pub fn nvic(model: &mut Model, config: Configuration) {
    let mut nvic = model.add_peripheral(Peripheral::new("nvic", 0xe000_e000));

    match config {
        Configuration::M0 => {
            iser(&mut nvic, iser::Instance::I1);
            icer(&mut nvic, icer::Instance::I1);
            ispr(&mut nvic, ispr::Instance::I1);
            icpr(&mut nvic, icpr::Instance::I1);
            iabr(&mut nvic, iabr::Instance::I1);
        }
        Configuration::M4 => {
            // enable
            for instance in iser::Instance::iter() {
                iser(&mut nvic, instance);
            }

            // disable
            for instance in icer::Instance::iter() {
                icer(&mut nvic, instance);
            }

            // pend
            for instance in ispr::Instance::iter() {
                ispr(&mut nvic, instance);
            }

            // unpend
            for instance in icpr::Instance::iter() {
                icpr(&mut nvic, instance);
            }

            // active
            for instance in iabr::Instance::iter() {
                iabr(&mut nvic, instance);
            }
        }
    }

    // priority
    for instance in ipr::Instance::iter(config) {
        ipr(&mut nvic, instance);
    }
}
