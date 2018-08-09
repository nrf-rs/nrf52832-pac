#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start Timer"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop Timer"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Increment Timer (Counter mode only)"]
    pub tasks_count: TASKS_COUNT,
    #[doc = "0x0c - Clear time"]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x10 - Deprecated register - Shut down timer"]
    pub tasks_shutdown: TASKS_SHUTDOWN,
    _reserved0: [u8; 44usize],
    #[doc = "0x40 - Description collection[0]: Capture Timer value to CC[0] register"]
    pub tasks_capture: [TASKS_CAPTURE; 6],
    _reserved1: [u8; 232usize],
    #[doc = "0x140 - Description collection[0]: Compare event on CC[0] match"]
    pub events_compare: [EVENTS_COMPARE; 6],
    _reserved2: [u8; 168usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved3: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 504usize],
    #[doc = "0x504 - Timer mode selection"]
    pub mode: MODE,
    #[doc = "0x508 - Configure the number of bits used by the TIMER"]
    pub bitmode: BITMODE,
    _reserved5: [u8; 4usize],
    #[doc = "0x510 - Timer prescaler register"]
    pub prescaler: PRESCALER,
    _reserved6: [u8; 44usize],
    #[doc = "0x540 - Description collection[0]: Capture/Compare register 0"]
    pub cc: [CC; 6],
}
#[doc = "Start Timer"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Timer"]
pub mod tasks_start;
#[doc = "Stop Timer"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop Timer"]
pub mod tasks_stop;
#[doc = "Increment Timer (Counter mode only)"]
pub struct TASKS_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Increment Timer (Counter mode only)"]
pub mod tasks_count;
#[doc = "Clear time"]
pub struct TASKS_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear time"]
pub mod tasks_clear;
#[doc = "Deprecated register - Shut down timer"]
pub struct TASKS_SHUTDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - Shut down timer"]
pub mod tasks_shutdown;
#[doc = "Description collection[0]: Capture Timer value to CC[0] register"]
pub struct TASKS_CAPTURE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Capture Timer value to CC[0] register"]
pub mod tasks_capture;
#[doc = "Description collection[0]: Compare event on CC[0] match"]
pub struct EVENTS_COMPARE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Compare event on CC[0] match"]
pub mod events_compare;
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
#[doc = "Timer mode selection"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer mode selection"]
pub mod mode;
#[doc = "Configure the number of bits used by the TIMER"]
pub struct BITMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configure the number of bits used by the TIMER"]
pub mod bitmode;
#[doc = "Timer prescaler register"]
pub struct PRESCALER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer prescaler register"]
pub mod prescaler;
#[doc = "Description collection[0]: Capture/Compare register 0"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Capture/Compare register 0"]
pub mod cc;
