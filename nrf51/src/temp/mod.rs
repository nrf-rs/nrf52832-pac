#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start temperature measurement."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop temperature measurement."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - Temperature measurement complete, data ready event."]
    pub events_datardy: EVENTS_DATARDY,
    _reserved3: [u8; 512usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 508usize],
    #[doc = "0x508 - Die temperature in degC, 2's complement format, 0.25 degC pecision."]
    pub temp: TEMP,
    _reserved6: [u8; 2800usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start temperature measurement."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start temperature measurement."]
pub mod tasks_start;
#[doc = "Stop temperature measurement."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop temperature measurement."]
pub mod tasks_stop;
#[doc = "Temperature measurement complete, data ready event."]
pub struct EVENTS_DATARDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Temperature measurement complete, data ready event."]
pub mod events_datardy;
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
#[doc = "Die temperature in degC, 2's complement format, 0.25 degC pecision."]
pub struct TEMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die temperature in degC, 2's complement format, 0.25 degC pecision."]
pub mod temp;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
