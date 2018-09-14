#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous PDM transfer"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stops PDM transfer"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - PDM transfer has started"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x104 - PDM transfer has finished"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
    pub events_end: EVENTS_END,
    _reserved5: [u8; 500usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 500usize],
    #[doc = "0x500 - PDM module enable register"]
    pub enable: ENABLE,
    #[doc = "0x504 - PDM clock generator control"]
    pub pdmclkctrl: PDMCLKCTRL,
    #[doc = "0x508 - Defines the routing of the connected PDM microphones' signals"]
    pub mode: MODE,
    _reserved11: [u8; 12usize],
    #[doc = "0x518 - Left output gain adjustment"]
    pub gainl: GAINL,
    #[doc = "0x51c - Right output gain adjustment"]
    pub gainr: GAINR,
    #[doc = "0x520 - Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
    pub ratio: RATIO,
    _reserved14: [u8; 28usize],
    #[doc = "0x540 - Unspecified"]
    pub psel: PSEL,
    _reserved15: [u8; 24usize],
    #[doc = "0x560 - Unspecified"]
    pub sample: SAMPLE,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin number configuration for PDM CLK signal"]
    pub clk: self::psel::CLK,
    #[doc = "0x04 - Pin number configuration for PDM DIN signal"]
    pub din: self::psel::DIN,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r" Register block"]
#[repr(C)]
pub struct SAMPLE {
    #[doc = "0x00 - RAM address pointer to write samples to with EasyDMA"]
    pub ptr: self::sample::PTR,
    #[doc = "0x04 - Number of samples to allocate memory for in EasyDMA mode"]
    pub maxcnt: self::sample::MAXCNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod sample;
#[doc = "Starts continuous PDM transfer"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Starts continuous PDM transfer"]
pub mod tasks_start;
#[doc = "Stops PDM transfer"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stops PDM transfer"]
pub mod tasks_stop;
#[doc = "PDM transfer has started"]
pub struct EVENTS_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDM transfer has started"]
pub mod events_started;
#[doc = "PDM transfer has finished"]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDM transfer has finished"]
pub mod events_stopped;
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
pub mod events_end;
#[doc = "Enable or disable interrupt"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable interrupt"]
pub mod inten;
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
#[doc = "PDM module enable register"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDM module enable register"]
pub mod enable;
#[doc = "PDM clock generator control"]
pub struct PDMCLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDM clock generator control"]
pub mod pdmclkctrl;
#[doc = "Defines the routing of the connected PDM microphones' signals"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Defines the routing of the connected PDM microphones' signals"]
pub mod mode;
#[doc = "Left output gain adjustment"]
pub struct GAINL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Left output gain adjustment"]
pub mod gainl;
#[doc = "Right output gain adjustment"]
pub struct GAINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Right output gain adjustment"]
pub mod gainr;
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
pub struct RATIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
pub mod ratio;
