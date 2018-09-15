#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the watchdog."]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - Watchdog timeout."]
    pub events_timeout: EVENTS_TIMEOUT,
    _reserved2: [u8; 512usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 244usize],
    #[doc = "0x400 - Watchdog running status."]
    pub runstatus: RUNSTATUS,
    #[doc = "0x404 - Request status."]
    pub reqstatus: REQSTATUS,
    _reserved6: [u8; 252usize],
    #[doc = "0x504 - Counter reload value in number of 32kiHz clock cycles."]
    pub crv: CRV,
    #[doc = "0x508 - Reload request enable."]
    pub rren: RREN,
    #[doc = "0x50c - Configuration register."]
    pub config: CONFIG,
    _reserved9: [u8; 240usize],
    #[doc = "0x600 - Reload requests registers."]
    pub rr: [RR; 8],
    _reserved10: [u8; 2524usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start the watchdog."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the watchdog."]
pub mod tasks_start;
#[doc = "Watchdog timeout."]
pub struct EVENTS_TIMEOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog timeout."]
pub mod events_timeout;
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
#[doc = "Watchdog running status."]
pub struct RUNSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog running status."]
pub mod runstatus;
#[doc = "Request status."]
pub struct REQSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request status."]
pub mod reqstatus;
#[doc = "Counter reload value in number of 32kiHz clock cycles."]
pub struct CRV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter reload value in number of 32kiHz clock cycles."]
pub mod crv;
#[doc = "Reload request enable."]
pub struct RREN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reload request enable."]
pub mod rren;
#[doc = "Configuration register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register."]
pub mod config;
#[doc = "Reload requests registers."]
pub struct RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reload requests registers."]
pub mod rr;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
