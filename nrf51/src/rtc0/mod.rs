#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start RTC Counter."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop RTC Counter."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Clear RTC Counter."]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x0c - Set COUNTER to 0xFFFFFFF0."]
    pub tasks_trigovrflw: TASKS_TRIGOVRFLW,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - Event on COUNTER increment."]
    pub events_tick: EVENTS_TICK,
    #[doc = "0x104 - Event on COUNTER overflow."]
    pub events_ovrflw: EVENTS_OVRFLW,
    _reserved6: [u8; 56usize],
    #[doc = "0x140 - Compare event on CC[n] match."]
    pub events_compare: [EVENTS_COMPARE; 4],
    _reserved7: [u8; 436usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 52usize],
    #[doc = "0x340 - Configures event enable routing to PPI for each RTC event."]
    pub evten: EVTEN,
    #[doc = "0x344 - Enable events routing to PPI. The reading of this register gives the value of EVTEN."]
    pub evtenset: EVTENSET,
    #[doc = "0x348 - Disable events routing to PPI. The reading of this register gives the value of EVTEN."]
    pub evtenclr: EVTENCLR,
    _reserved12: [u8; 440usize],
    #[doc = "0x504 - Current COUNTER value."]
    pub counter: COUNTER,
    #[doc = "0x508 - 12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
    pub prescaler: PRESCALER,
    _reserved14: [u8; 52usize],
    #[doc = "0x540 - Capture/compare registers."]
    pub cc: [CC; 4],
    _reserved15: [u8; 2732usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start RTC Counter."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start RTC Counter."]
pub mod tasks_start;
#[doc = "Stop RTC Counter."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop RTC Counter."]
pub mod tasks_stop;
#[doc = "Clear RTC Counter."]
pub struct TASKS_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear RTC Counter."]
pub mod tasks_clear;
#[doc = "Set COUNTER to 0xFFFFFFF0."]
pub struct TASKS_TRIGOVRFLW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set COUNTER to 0xFFFFFFF0."]
pub mod tasks_trigovrflw;
#[doc = "Event on COUNTER increment."]
pub struct EVENTS_TICK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event on COUNTER increment."]
pub mod events_tick;
#[doc = "Event on COUNTER overflow."]
pub struct EVENTS_OVRFLW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event on COUNTER overflow."]
pub mod events_ovrflw;
#[doc = "Compare event on CC[n] match."]
pub struct EVENTS_COMPARE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare event on CC[n] match."]
pub mod events_compare;
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
#[doc = "Configures event enable routing to PPI for each RTC event."]
pub struct EVTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures event enable routing to PPI for each RTC event."]
pub mod evten;
#[doc = "Enable events routing to PPI. The reading of this register gives the value of EVTEN."]
pub struct EVTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable events routing to PPI. The reading of this register gives the value of EVTEN."]
pub mod evtenset;
#[doc = "Disable events routing to PPI. The reading of this register gives the value of EVTEN."]
pub struct EVTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable events routing to PPI. The reading of this register gives the value of EVTEN."]
pub mod evtenclr;
#[doc = "Current COUNTER value."]
pub struct COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current COUNTER value."]
pub mod counter;
#[doc = "12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
pub struct PRESCALER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
pub mod prescaler;
#[doc = "Capture/compare registers."]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/compare registers."]
pub mod cc;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
