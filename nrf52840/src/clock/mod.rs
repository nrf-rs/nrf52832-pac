#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFXO crystal oscillator"]
    pub tasks_hfclkstart: TASKS_HFCLKSTART,
    #[doc = "0x04 - Stop HFXO crystal oscillator"]
    pub tasks_hfclkstop: TASKS_HFCLKSTOP,
    #[doc = "0x08 - Start LFCLK"]
    pub tasks_lfclkstart: TASKS_LFCLKSTART,
    #[doc = "0x0c - Stop LFCLK"]
    pub tasks_lfclkstop: TASKS_LFCLKSTOP,
    #[doc = "0x10 - Start calibration of LFRC"]
    pub tasks_cal: TASKS_CAL,
    #[doc = "0x14 - Start calibration timer"]
    pub tasks_ctstart: TASKS_CTSTART,
    #[doc = "0x18 - Stop calibration timer"]
    pub tasks_ctstop: TASKS_CTSTOP,
    _reserved7: [u8; 228usize],
    #[doc = "0x100 - HFXO crystal oscillator started"]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK started"]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved9: [u8; 4usize],
    #[doc = "0x10c - Calibration of LFRC completed"]
    pub events_done: EVENTS_DONE,
    #[doc = "0x110 - Calibration timer timeout"]
    pub events_ctto: EVENTS_CTTO,
    _reserved11: [u8; 20usize],
    #[doc = "0x128 - Calibration timer has been started and is ready to process new tasks"]
    pub events_ctstarted: EVENTS_CTSTARTED,
    #[doc = "0x12c - Calibration timer has been stopped and is ready to process new tasks"]
    pub events_ctstopped: EVENTS_CTSTOPPED,
    _reserved13: [u8; 468usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved15: [u8; 252usize],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: HFCLKRUN,
    #[doc = "0x40c - HFCLK status"]
    pub hfclkstat: HFCLKSTAT,
    _reserved17: [u8; 4usize],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: LFCLKRUN,
    #[doc = "0x418 - LFCLK status"]
    pub lfclkstat: LFCLKSTAT,
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    pub lfclksrccopy: LFCLKSRCCOPY,
    _reserved20: [u8; 248usize],
    #[doc = "0x518 - Clock source for the LFCLK"]
    pub lfclksrc: LFCLKSRC,
    _reserved21: [u8; 12usize],
    #[doc = "0x528 - HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
    pub hfxodebounce: HFXODEBOUNCE,
    _reserved22: [u8; 12usize],
    #[doc = "0x538 - Calibration timer interval"]
    pub ctiv: CTIV,
    _reserved23: [u8; 32usize],
    #[doc = "0x55c - Clocking options for the trace port debug interface"]
    pub traceconfig: TRACECONFIG,
    _reserved24: [u8; 84usize],
    #[doc = "0x5b4 - LFRC mode configuration"]
    pub lfrcmode: LFRCMODE,
}
#[doc = "Start HFXO crystal oscillator"]
pub struct TASKS_HFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start HFXO crystal oscillator"]
pub mod tasks_hfclkstart;
#[doc = "Stop HFXO crystal oscillator"]
pub struct TASKS_HFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop HFXO crystal oscillator"]
pub mod tasks_hfclkstop;
#[doc = "Start LFCLK"]
pub struct TASKS_LFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start LFCLK"]
pub mod tasks_lfclkstart;
#[doc = "Stop LFCLK"]
pub struct TASKS_LFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop LFCLK"]
pub mod tasks_lfclkstop;
#[doc = "Start calibration of LFRC"]
pub struct TASKS_CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start calibration of LFRC"]
pub mod tasks_cal;
#[doc = "Start calibration timer"]
pub struct TASKS_CTSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start calibration timer"]
pub mod tasks_ctstart;
#[doc = "Stop calibration timer"]
pub struct TASKS_CTSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop calibration timer"]
pub mod tasks_ctstop;
#[doc = "HFXO crystal oscillator started"]
pub struct EVENTS_HFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO crystal oscillator started"]
pub mod events_hfclkstarted;
#[doc = "LFCLK started"]
pub struct EVENTS_LFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "Calibration of LFRC completed"]
pub struct EVENTS_DONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration of LFRC completed"]
pub mod events_done;
#[doc = "Calibration timer timeout"]
pub struct EVENTS_CTTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration timer timeout"]
pub mod events_ctto;
#[doc = "Calibration timer has been started and is ready to process new tasks"]
pub struct EVENTS_CTSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration timer has been started and is ready to process new tasks"]
pub mod events_ctstarted;
#[doc = "Calibration timer has been stopped and is ready to process new tasks"]
pub struct EVENTS_CTSTOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration timer has been stopped and is ready to process new tasks"]
pub mod events_ctstopped;
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
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub struct HFCLKRUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLK status"]
pub struct HFCLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFCLK status"]
pub mod hfclkstat;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub struct LFCLKRUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLK status"]
pub struct LFCLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFCLK status"]
pub mod lfclkstat;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub struct LFCLKSRCCOPY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "Clock source for the LFCLK"]
pub struct LFCLKSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock source for the LFCLK"]
pub mod lfclksrc;
#[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
pub struct HFXODEBOUNCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
pub mod hfxodebounce;
#[doc = "Calibration timer interval"]
pub struct CTIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration timer interval"]
pub mod ctiv;
#[doc = "Clocking options for the trace port debug interface"]
pub struct TRACECONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clocking options for the trace port debug interface"]
pub mod traceconfig;
#[doc = "LFRC mode configuration"]
pub struct LFRCMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFRC mode configuration"]
pub mod lfrcmode;
