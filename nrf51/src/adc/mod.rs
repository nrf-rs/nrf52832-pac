#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start an ADC conversion."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop ADC."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - ADC conversion complete."]
    pub events_end: EVENTS_END,
    _reserved3: [u8; 512usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 244usize],
    #[doc = "0x400 - ADC busy register."]
    pub busy: BUSY,
    _reserved6: [u8; 252usize],
    #[doc = "0x500 - ADC enable."]
    pub enable: ENABLE,
    #[doc = "0x504 - ADC configuration register."]
    pub config: CONFIG,
    #[doc = "0x508 - Result of ADC conversion."]
    pub result: RESULT,
    _reserved9: [u8; 2800usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start an ADC conversion."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start an ADC conversion."]
pub mod tasks_start;
#[doc = "Stop ADC."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop ADC."]
pub mod tasks_stop;
#[doc = "ADC conversion complete."]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC conversion complete."]
pub mod events_end;
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
#[doc = "ADC busy register."]
pub struct BUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC busy register."]
pub mod busy;
#[doc = "ADC enable."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC enable."]
pub mod enable;
#[doc = "ADC configuration register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC configuration register."]
pub mod config;
#[doc = "Result of ADC conversion."]
pub struct RESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result of ADC conversion."]
pub mod result;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
