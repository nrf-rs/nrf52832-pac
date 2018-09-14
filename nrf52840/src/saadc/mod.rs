#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts the SAADC and prepares the result buffer in RAM"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Takes one SAADC sample"]
    pub tasks_sample: TASKS_SAMPLE,
    #[doc = "0x08 - Stops the SAADC and terminates all on-going conversions"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x0c - Starts offset auto-calibration"]
    pub tasks_calibrateoffset: TASKS_CALIBRATEOFFSET,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - The SAADC has started"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x104 - The SAADC has filled up the result buffer"]
    pub events_end: EVENTS_END,
    #[doc = "0x108 - A conversion task has been completed. Depending on the configuration, multiple conversions might be needed for a result to be transferred to RAM."]
    pub events_done: EVENTS_DONE,
    #[doc = "0x10c - Result ready for transfer to RAM"]
    pub events_resultdone: EVENTS_RESULTDONE,
    #[doc = "0x110 - Calibration is complete"]
    pub events_calibratedone: EVENTS_CALIBRATEDONE,
    #[doc = "0x114 - The SAADC has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x118 - Unspecified"]
    pub events_ch: [EVENTS_CH; 8],
    _reserved11: [u8; 424usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 244usize],
    #[doc = "0x400 - Status"]
    pub status: STATUS,
    _reserved15: [u8; 252usize],
    #[doc = "0x500 - Enable or disable SAADC"]
    pub enable: ENABLE,
    _reserved16: [u8; 12usize],
    #[doc = "0x510 - Unspecified"]
    pub ch: [CH; 8],
    _reserved17: [u8; 96usize],
    #[doc = "0x5f0 - Resolution configuration"]
    pub resolution: RESOLUTION,
    #[doc = "0x5f4 - Oversampling configuration. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    pub oversample: OVERSAMPLE,
    #[doc = "0x5f8 - Controls normal or continuous sample rate"]
    pub samplerate: SAMPLERATE,
    _reserved20: [u8; 48usize],
    #[doc = "0x62c - RESULT EasyDMA channel"]
    pub result: RESULT,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct EVENTS_CH {
    #[doc = "0x00 - Description cluster[n]: Last result is equal or above CH[n].LIMIT.HIGH"]
    pub limith: self::events_ch::LIMITH,
    #[doc = "0x04 - Description cluster[n]: Last result is equal or below CH[n].LIMIT.LOW"]
    pub limitl: self::events_ch::LIMITL,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod events_ch;
#[doc = r" Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster[n]: Input positive pin selection for CH[n]"]
    pub pselp: self::ch::PSELP,
    #[doc = "0x04 - Description cluster[n]: Input negative pin selection for CH[n]"]
    pub pseln: self::ch::PSELN,
    #[doc = "0x08 - Description cluster[n]: Input configuration for CH[n]"]
    pub config: self::ch::CONFIG,
    #[doc = "0x0c - Description cluster[n]: High/low limits for event monitoring of a channel"]
    pub limit: self::ch::LIMIT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod ch;
#[doc = r" Register block"]
#[repr(C)]
pub struct RESULT {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::result::PTR,
    #[doc = "0x04 - Maximum number of 16-bit samples to be written to output RAM buffer"]
    pub maxcnt: self::result::MAXCNT,
    #[doc = "0x08 - Number of 16-bit samples written to output RAM buffer since the previous START task"]
    pub amount: self::result::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "RESULT EasyDMA channel"]
pub mod result;
#[doc = "Starts the SAADC and prepares the result buffer in RAM"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Starts the SAADC and prepares the result buffer in RAM"]
pub mod tasks_start;
#[doc = "Takes one SAADC sample"]
pub struct TASKS_SAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Takes one SAADC sample"]
pub mod tasks_sample;
#[doc = "Stops the SAADC and terminates all on-going conversions"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stops the SAADC and terminates all on-going conversions"]
pub mod tasks_stop;
#[doc = "Starts offset auto-calibration"]
pub struct TASKS_CALIBRATEOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Starts offset auto-calibration"]
pub mod tasks_calibrateoffset;
#[doc = "The SAADC has started"]
pub struct EVENTS_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The SAADC has started"]
pub mod events_started;
#[doc = "The SAADC has filled up the result buffer"]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The SAADC has filled up the result buffer"]
pub mod events_end;
#[doc = "A conversion task has been completed. Depending on the configuration, multiple conversions might be needed for a result to be transferred to RAM."]
pub struct EVENTS_DONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A conversion task has been completed. Depending on the configuration, multiple conversions might be needed for a result to be transferred to RAM."]
pub mod events_done;
#[doc = "Result ready for transfer to RAM"]
pub struct EVENTS_RESULTDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result ready for transfer to RAM"]
pub mod events_resultdone;
#[doc = "Calibration is complete"]
pub struct EVENTS_CALIBRATEDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration is complete"]
pub mod events_calibratedone;
#[doc = "The SAADC has stopped"]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The SAADC has stopped"]
pub mod events_stopped;
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
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Enable or disable SAADC"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable SAADC"]
pub mod enable;
#[doc = "Resolution configuration"]
pub struct RESOLUTION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resolution configuration"]
pub mod resolution;
#[doc = "Oversampling configuration. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub struct OVERSAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oversampling configuration. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub mod oversample;
#[doc = "Controls normal or continuous sample rate"]
pub struct SAMPLERATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls normal or continuous sample rate"]
pub mod samplerate;
