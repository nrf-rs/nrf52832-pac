#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Tasks asssociated with GPIOTE channels."]
    pub tasks_out: [TASKS_OUT; 4],
    _reserved1: [u8; 240usize],
    #[doc = "0x100 - Tasks asssociated with GPIOTE channels."]
    pub events_in: [EVENTS_IN; 4],
    _reserved2: [u8; 108usize],
    #[doc = "0x17c - Event generated from multiple pins."]
    pub events_port: EVENTS_PORT,
    _reserved3: [u8; 388usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 516usize],
    #[doc = "0x510 - Channel configuration registers."]
    pub config: [CONFIG; 4],
    _reserved6: [u8; 2780usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Tasks asssociated with GPIOTE channels."]
pub struct TASKS_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tasks asssociated with GPIOTE channels."]
pub mod tasks_out;
#[doc = "Tasks asssociated with GPIOTE channels."]
pub struct EVENTS_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tasks asssociated with GPIOTE channels."]
pub mod events_in;
#[doc = "Event generated from multiple pins."]
pub struct EVENTS_PORT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event generated from multiple pins."]
pub mod events_port;
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
#[doc = "Channel configuration registers."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel configuration registers."]
pub mod config;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
