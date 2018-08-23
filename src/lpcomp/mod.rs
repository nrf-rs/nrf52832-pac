#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start comparator"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop comparator"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Sample comparator value"]
    pub tasks_sample: TASKS_SAMPLE,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - LPCOMP is ready and output is valid"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Downward crossing"]
    pub events_down: EVENTS_DOWN,
    #[doc = "0x108 - Upward crossing"]
    pub events_up: EVENTS_UP,
    #[doc = "0x10c - Downward or upward crossing"]
    pub events_cross: EVENTS_CROSS,
    _reserved7: [u8; 240usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved8: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 244usize],
    #[doc = "0x400 - Compare result"]
    pub result: RESULT,
    _reserved11: [u8; 252usize],
    #[doc = "0x500 - Enable LPCOMP"]
    pub enable: ENABLE,
    #[doc = "0x504 - Input pin select"]
    pub psel: PSEL,
    #[doc = "0x508 - Reference select"]
    pub refsel: REFSEL,
    #[doc = "0x50c - External reference select"]
    pub extrefsel: EXTREFSEL,
    _reserved15: [u8; 16usize],
    #[doc = "0x520 - Analog detect configuration"]
    pub anadetect: ANADETECT,
    _reserved16: [u8; 20usize],
    #[doc = "0x538 - Comparator hysteresis enable"]
    pub hyst: HYST,
}
#[doc = "Start comparator"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start comparator"]
pub mod tasks_start;
#[doc = "Stop comparator"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop comparator"]
pub mod tasks_stop;
#[doc = "Sample comparator value"]
pub struct TASKS_SAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample comparator value"]
pub mod tasks_sample;
#[doc = "LPCOMP is ready and output is valid"]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPCOMP is ready and output is valid"]
pub mod events_ready;
#[doc = "Downward crossing"]
pub struct EVENTS_DOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Downward crossing"]
pub mod events_down;
#[doc = "Upward crossing"]
pub struct EVENTS_UP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Upward crossing"]
pub mod events_up;
#[doc = "Downward or upward crossing"]
pub struct EVENTS_CROSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Downward or upward crossing"]
pub mod events_cross;
#[doc = "Shortcut register"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Compare result"]
pub struct RESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare result"]
pub mod result;
#[doc = "Enable LPCOMP"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable LPCOMP"]
pub mod enable;
#[doc = "Input pin select"]
pub struct PSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input pin select"]
pub mod psel;
#[doc = "Reference select"]
pub struct REFSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reference select"]
pub mod refsel;
#[doc = "External reference select"]
pub struct EXTREFSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External reference select"]
pub mod extrefsel;
#[doc = "Analog detect configuration"]
pub struct ANADETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog detect configuration"]
pub mod anadetect;
#[doc = "Comparator hysteresis enable"]
pub struct HYST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator hysteresis enable"]
pub mod hyst;
