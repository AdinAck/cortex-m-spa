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
    Peripheral::new("nvic", 0xe000_0000, {
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

// #[block(base_addr = 0xe000_e000, erase_mod)]
// mod nvic {
//     #[schema(width = 1)]
//     mod enable {
//         #[variant(bits = 0)]
//         struct Disabled;
//         #[variant(bits = 1)]
//         struct Enabled;
//     }

//     // TODO: wish there was 'register_array'
//     // but how would that be done?

//     // TODO: write(values = [Enabled])
//     #[register(offset = 0x100, schema = enable, read, write, reset = Disabled)]
//     mod iser0 {
//         #[field_array(offset = 0, range = ..32)]
//         mod setenaX {}
//     }

//     #[register(offset = 0x104, schema = enable, read, write, reset = Disabled)]
//     mod iser1 {
//         #[field_array(offset = 0, range = 32..64)]
//         mod setenaX {}
//     }

//     #[register(offset = 0x108, schema = enable, read, write, reset = Disabled)]
//     mod iser2 {
//         #[field_array(offset = 0, range = 64..96)]
//         mod setenaX {}
//     }

//     #[register(offset = 0x10c, schema = enable, read, write, reset = Disabled)]
//     mod iser3 {
//         #[field_array(offset = 0, range = 96..128)]
//         mod setenaX {}
//     }

//     #[register(offset = 0x110, schema = enable, read, write, reset = Disabled)]
//     mod iser4 {
//         #[field_array(offset = 0, range = 128..160)]
//         mod setenaX {}
//     }

//     #[register(offset = 0x114, schema = enable, read, write, reset = Disabled)]
//     mod iser5 {
//         #[field_array(offset = 0, range = 160..192)]
//         mod setenaX {}
//     }

//     #[register(offset = 0x118, schema = enable, read, write, reset = Disabled)]
//     mod iser6 {
//         #[field_array(offset = 0, range = 192..224)]
//         mod setenaX {}
//     }

//     #[register(offset = 0x11c, schema = enable, read, write, reset = Disabled)]
//     mod iser7 {
//         #[field_array(offset = 0, range = 224..240)]
//         mod setenaX {}
//     }

//     #[register(offset = 0x180, schema = enable, read, write, reset = Disabled)]
//     mod icer0 {
//         #[field_array(offset = 0, range = ..32)]
//         mod clrenaX {}
//     }

//     #[register(offset = 0x184, schema = enable, read, write, reset = Disabled)]
//     mod icer1 {
//         #[field_array(offset = 0, range = 32..64)]
//         mod clrenaX {}
//     }

//     #[register(offset = 0x188, schema = enable, read, write, reset = Disabled)]
//     mod icer2 {
//         #[field_array(offset = 0, range = 64..96)]
//         mod clrenaX {}
//     }

//     #[register(offset = 0x18c, schema = enable, read, write, reset = Disabled)]
//     mod icer3 {
//         #[field_array(offset = 0, range = 96..128)]
//         mod clrenaX {}
//     }

//     #[register(offset = 0x190, schema = enable, read, write, reset = Disabled)]
//     mod icer4 {
//         #[field_array(offset = 0, range = 128..160)]
//         mod clrenaX {}
//     }

//     #[register(offset = 0x194, schema = enable, read, write, reset = Disabled)]
//     mod icer5 {
//         #[field_array(offset = 0, range = 160..192)]
//         mod clrenaX {}
//     }

//     #[register(offset = 0x198, schema = enable, read, write, reset = Disabled)]
//     mod icer6 {
//         #[field_array(offset = 0, range = 192..224)]
//         mod clrenaX {}
//     }

//     #[register(offset = 0x19c, schema = enable, read, write, reset = Disabled)]
//     mod icer7 {
//         #[field_array(offset = 0, range = 224..240)]
//         mod clrenaX {}
//     }

//     // TODO...
// }
