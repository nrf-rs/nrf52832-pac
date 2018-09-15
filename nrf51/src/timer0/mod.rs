#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start Timer."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop Timer."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Increment Timer (In counter mode)."]
    pub tasks_count: TASKS_COUNT,
    #[doc = "0x0c - Clear timer."]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x10 - Shutdown timer."]
    pub tasks_shutdown: TASKS_SHUTDOWN,
    _reserved5: [u8; 44usize],
    #[doc = "0x40 - Capture Timer value to CC[n] registers."]
    pub tasks_capture: [TASKS_CAPTURE; 4],
    _reserved6: [u8; 240usize],
    #[doc = "0x140 - Compare event on CC[n] match."]
    pub events_compare: [EVENTS_COMPARE; 4],
    _reserved7: [u8; 176usize],
    #[doc = "0x200 - Shortcuts for Timer."]
    pub shorts: SHORTS,
    _reserved8: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 504usize],
    #[doc = "0x504 - Timer Mode selection."]
    pub mode: MODE,
    #[doc = "0x508 - Sets timer behaviour."]
    pub bitmode: BITMODE,
    _reserved12: [u8; 4usize],
    #[doc = "0x510 - 4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE."]
    pub prescaler: PRESCALER,
    _reserved13: [u8; 44usize],
    #[doc = "0x540 - Capture/compare registers."]
    pub cc: [CC; 4],
    _reserved14: [u8; 2732usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start Timer."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Timer."]
pub mod tasks_start;
#[doc = "Stop Timer."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop Timer."]
pub mod tasks_stop;
#[doc = "Increment Timer (In counter mode)."]
pub struct TASKS_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Increment Timer (In counter mode)."]
pub mod tasks_count;
#[doc = "Clear timer."]
pub struct TASKS_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear timer."]
pub mod tasks_clear;
#[doc = "Shutdown timer."]
pub struct TASKS_SHUTDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shutdown timer."]
pub mod tasks_shutdown;
#[doc = "Capture Timer value to CC[n] registers."]
pub struct TASKS_CAPTURE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Timer value to CC[n] registers."]
pub mod tasks_capture;
#[doc = "Compare event on CC[n] match."]
pub struct EVENTS_COMPARE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare event on CC[n] match."]
pub mod events_compare;
#[doc = "Shortcuts for Timer."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts for Timer."]
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
#[doc = "Timer Mode selection."]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Mode selection."]
pub mod mode;
#[doc = "Sets timer behaviour."]
pub struct BITMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sets timer behaviour."]
pub mod bitmode;
#[doc = "4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE."]
pub struct PRESCALER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE."]
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
