#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start RTC COUNTER"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop RTC COUNTER"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Clear RTC COUNTER"]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x0c - Set COUNTER to 0xFFFFF0"]
    pub tasks_trigovrflw: TASKS_TRIGOVRFLW,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - Event on COUNTER increment"]
    pub events_tick: EVENTS_TICK,
    #[doc = "0x104 - Event on COUNTER overflow"]
    pub events_ovrflw: EVENTS_OVRFLW,
    _reserved6: [u8; 56usize],
    #[doc = "0x140 - Description collection[n]: Compare event on CC[n] match"]
    pub events_compare: [EVENTS_COMPARE; 4],
    _reserved7: [u8; 436usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 52usize],
    #[doc = "0x340 - Enable or disable event routing"]
    pub evten: EVTEN,
    #[doc = "0x344 - Enable event routing"]
    pub evtenset: EVTENSET,
    #[doc = "0x348 - Disable event routing"]
    pub evtenclr: EVTENCLR,
    _reserved12: [u8; 440usize],
    #[doc = "0x504 - Current COUNTER value"]
    pub counter: COUNTER,
    #[doc = "0x508 - 12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
    pub prescaler: PRESCALER,
    _reserved14: [u8; 52usize],
    #[doc = "0x540 - Description collection[n]: Compare register n"]
    pub cc: [CC; 4],
}
#[doc = "Start RTC COUNTER"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start RTC COUNTER"]
pub mod tasks_start;
#[doc = "Stop RTC COUNTER"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop RTC COUNTER"]
pub mod tasks_stop;
#[doc = "Clear RTC COUNTER"]
pub struct TASKS_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear RTC COUNTER"]
pub mod tasks_clear;
#[doc = "Set COUNTER to 0xFFFFF0"]
pub struct TASKS_TRIGOVRFLW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set COUNTER to 0xFFFFF0"]
pub mod tasks_trigovrflw;
#[doc = "Event on COUNTER increment"]
pub struct EVENTS_TICK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event on COUNTER increment"]
pub mod events_tick;
#[doc = "Event on COUNTER overflow"]
pub struct EVENTS_OVRFLW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event on COUNTER overflow"]
pub mod events_ovrflw;
#[doc = "Description collection[n]: Compare event on CC[n] match"]
pub struct EVENTS_COMPARE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Compare event on CC[n] match"]
pub mod events_compare;
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
#[doc = "Enable or disable event routing"]
pub struct EVTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable event routing"]
pub mod evten;
#[doc = "Enable event routing"]
pub struct EVTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable event routing"]
pub mod evtenset;
#[doc = "Disable event routing"]
pub struct EVTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable event routing"]
pub mod evtenclr;
#[doc = "Current COUNTER value"]
pub struct COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current COUNTER value"]
pub mod counter;
#[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
pub struct PRESCALER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
pub mod prescaler;
#[doc = "Description collection[n]: Compare register n"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Compare register n"]
pub mod cc;
