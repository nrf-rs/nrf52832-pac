#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK crystal oscillator"]
    pub tasks_hfclkstart: TASKS_HFCLKSTART,
    #[doc = "0x04 - Stop HFCLK crystal oscillator"]
    pub tasks_hfclkstop: TASKS_HFCLKSTOP,
    #[doc = "0x08 - Start LFCLK source"]
    pub tasks_lfclkstart: TASKS_LFCLKSTART,
    #[doc = "0x0c - Stop LFCLK source"]
    pub tasks_lfclkstop: TASKS_LFCLKSTOP,
    #[doc = "0x10 - Start calibration of LFRC oscillator"]
    pub tasks_cal: TASKS_CAL,
    #[doc = "0x14 - Start calibration timer"]
    pub tasks_ctstart: TASKS_CTSTART,
    #[doc = "0x18 - Stop calibration timer"]
    pub tasks_ctstop: TASKS_CTSTOP,
    _reserved0: [u8; 228usize],
    #[doc = "0x100 - HFCLK oscillator started"]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK started"]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved1: [u8; 4usize],
    #[doc = "0x10c - Calibration of LFCLK RC oscillator complete event"]
    pub events_done: EVENTS_DONE,
    #[doc = "0x110 - Calibration timer timeout"]
    pub events_ctto: EVENTS_CTTO,
    _reserved2: [u8; 496usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 252usize],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: HFCLKRUN,
    #[doc = "0x40c - HFCLK status"]
    pub hfclkstat: HFCLKSTAT,
    _reserved4: [u8; 4usize],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: LFCLKRUN,
    #[doc = "0x418 - LFCLK status"]
    pub lfclkstat: LFCLKSTAT,
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    pub lfclksrccopy: LFCLKSRCCOPY,
    _reserved5: [u8; 248usize],
    #[doc = "0x518 - Clock source for the LFCLK"]
    pub lfclksrc: LFCLKSRC,
    _reserved6: [u8; 28usize],
    #[doc = "0x538 - Calibration timer interval"]
    pub ctiv: CTIV,
    _reserved7: [u8; 32usize],
    #[doc = "0x55c - Clocking options for the Trace Port debug interface"]
    pub traceconfig: TRACECONFIG,
}
#[doc = "Start HFCLK crystal oscillator"]
pub struct TASKS_HFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start HFCLK crystal oscillator"]
pub mod tasks_hfclkstart;
#[doc = "Stop HFCLK crystal oscillator"]
pub struct TASKS_HFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop HFCLK crystal oscillator"]
pub mod tasks_hfclkstop;
#[doc = "Start LFCLK source"]
pub struct TASKS_LFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start LFCLK source"]
pub mod tasks_lfclkstart;
#[doc = "Stop LFCLK source"]
pub struct TASKS_LFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "Start calibration of LFRC oscillator"]
pub struct TASKS_CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start calibration of LFRC oscillator"]
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
#[doc = "HFCLK oscillator started"]
pub struct EVENTS_HFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFCLK oscillator started"]
pub mod events_hfclkstarted;
#[doc = "LFCLK started"]
pub struct EVENTS_LFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "Calibration of LFCLK RC oscillator complete event"]
pub struct EVENTS_DONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration of LFCLK RC oscillator complete event"]
pub mod events_done;
#[doc = "Calibration timer timeout"]
pub struct EVENTS_CTTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration timer timeout"]
pub mod events_ctto;
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
#[doc = "Calibration timer interval"]
pub struct CTIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration timer interval"]
pub mod ctiv;
#[doc = "Clocking options for the Trace Port debug interface"]
pub struct TRACECONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clocking options for the Trace Port debug interface"]
pub mod traceconfig;
