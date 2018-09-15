#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the random number generator."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop the random number generator."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - New random number generated and written to VALUE register."]
    pub events_valrdy: EVENTS_VALRDY,
    _reserved3: [u8; 252usize],
    #[doc = "0x200 - Shortcuts for the RNG."]
    pub shorts: SHORTS,
    _reserved4: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register"]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 504usize],
    #[doc = "0x504 - Configuration register."]
    pub config: CONFIG,
    #[doc = "0x508 - RNG random number."]
    pub value: VALUE,
    _reserved8: [u8; 2800usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start the random number generator."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the random number generator."]
pub mod tasks_start;
#[doc = "Stop the random number generator."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop the random number generator."]
pub mod tasks_stop;
#[doc = "New random number generated and written to VALUE register."]
pub struct EVENTS_VALRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New random number generated and written to VALUE register."]
pub mod events_valrdy;
#[doc = "Shortcuts for the RNG."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts for the RNG."]
pub mod shorts;
#[doc = "Interrupt enable set register"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register"]
pub mod intenset;
#[doc = "Interrupt enable clear register"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register"]
pub mod intenclr;
#[doc = "Configuration register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register."]
pub mod config;
#[doc = "RNG random number."]
pub struct VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG random number."]
pub mod value;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
