use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Task starting the random number generator"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Task stopping the random number generator"]
    pub tasks_stop: TASKS_STOP,
    _reserved0: [u8; 248usize],
    #[doc = "0x100 - Event being generated for every new random number written to the VALUE register"]
    pub events_valrdy: EVENTS_VALRDY,
    _reserved1: [u8; 252usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved2: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 504usize],
    #[doc = "0x504 - Configuration register"]
    pub config: CONFIG,
    #[doc = "0x508 - Output random number"]
    pub value: VALUE,
}
#[doc = "Task starting the random number generator"]
pub struct TASKS_START {
    register: VolatileCell<u32>,
}
#[doc = "Task starting the random number generator"]
pub mod tasks_start;
#[doc = "Task stopping the random number generator"]
pub struct TASKS_STOP {
    register: VolatileCell<u32>,
}
#[doc = "Task stopping the random number generator"]
pub mod tasks_stop;
#[doc = "Event being generated for every new random number written to the VALUE register"]
pub struct EVENTS_VALRDY {
    register: VolatileCell<u32>,
}
#[doc = "Event being generated for every new random number written to the VALUE register"]
pub mod events_valrdy;
#[doc = "Shortcut register"]
pub struct SHORTS {
    register: VolatileCell<u32>,
}
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Output random number"]
pub struct VALUE {
    register: VolatileCell<u32>,
}
#[doc = "Output random number"]
pub mod value;
