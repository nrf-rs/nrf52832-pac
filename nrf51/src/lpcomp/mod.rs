#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the comparator."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop the comparator."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Sample comparator value."]
    pub tasks_sample: TASKS_SAMPLE,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - LPCOMP is ready and output is valid."]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Input voltage crossed the threshold going down."]
    pub events_down: EVENTS_DOWN,
    #[doc = "0x108 - Input voltage crossed the threshold going up."]
    pub events_up: EVENTS_UP,
    #[doc = "0x10c - Input voltage crossed the threshold in any direction."]
    pub events_cross: EVENTS_CROSS,
    _reserved7: [u8; 240usize],
    #[doc = "0x200 - Shortcuts for the LPCOMP."]
    pub shorts: SHORTS,
    _reserved8: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 244usize],
    #[doc = "0x400 - Result of last compare."]
    pub result: RESULT,
    _reserved11: [u8; 252usize],
    #[doc = "0x500 - Enable the LPCOMP."]
    pub enable: ENABLE,
    #[doc = "0x504 - Input pin select."]
    pub psel: PSEL,
    #[doc = "0x508 - Reference select."]
    pub refsel: REFSEL,
    #[doc = "0x50c - External reference select."]
    pub extrefsel: EXTREFSEL,
    _reserved15: [u8; 16usize],
    #[doc = "0x520 - Analog detect configuration."]
    pub anadetect: ANADETECT,
    _reserved16: [u8; 2776usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start the comparator."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the comparator."]
pub mod tasks_start;
#[doc = "Stop the comparator."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop the comparator."]
pub mod tasks_stop;
#[doc = "Sample comparator value."]
pub struct TASKS_SAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample comparator value."]
pub mod tasks_sample;
#[doc = "LPCOMP is ready and output is valid."]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPCOMP is ready and output is valid."]
pub mod events_ready;
#[doc = "Input voltage crossed the threshold going down."]
pub struct EVENTS_DOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input voltage crossed the threshold going down."]
pub mod events_down;
#[doc = "Input voltage crossed the threshold going up."]
pub struct EVENTS_UP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input voltage crossed the threshold going up."]
pub mod events_up;
#[doc = "Input voltage crossed the threshold in any direction."]
pub struct EVENTS_CROSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input voltage crossed the threshold in any direction."]
pub mod events_cross;
#[doc = "Shortcuts for the LPCOMP."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts for the LPCOMP."]
pub mod shorts;
#[doc = "Interrupt enable set register."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "Interrupt enable clear register."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "Result of last compare."]
pub struct RESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result of last compare."]
pub mod result;
#[doc = "Enable the LPCOMP."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable the LPCOMP."]
pub mod enable;
#[doc = "Input pin select."]
pub struct PSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input pin select."]
pub mod psel;
#[doc = "Reference select."]
pub struct REFSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reference select."]
pub mod refsel;
#[doc = "External reference select."]
pub struct EXTREFSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External reference select."]
pub mod extrefsel;
#[doc = "Analog detect configuration."]
pub struct ANADETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog detect configuration."]
pub mod anadetect;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
