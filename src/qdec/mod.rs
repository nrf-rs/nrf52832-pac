use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Task starting the quadrature decoder"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Task stopping the quadrature decoder"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Read and clear ACC and ACCDBL"]
    pub tasks_readclracc: TASKS_READCLRACC,
    #[doc = "0x0c - Read and clear ACC"]
    pub tasks_rdclracc: TASKS_RDCLRACC,
    #[doc = "0x10 - Read and clear ACCDBL"]
    pub tasks_rdclrdbl: TASKS_RDCLRDBL,
    _reserved0: [u8; 236usize],
    #[doc = "0x100 - Event being generated for every new sample value written to the SAMPLE register"]
    pub events_samplerdy: EVENTS_SAMPLERDY,
    #[doc = "0x104 - Non-null report ready"]
    pub events_reportrdy: EVENTS_REPORTRDY,
    #[doc = "0x108 - ACC or ACCDBL register overflow"]
    pub events_accof: EVENTS_ACCOF,
    #[doc = "0x10c - Double displacement(s) detected"]
    pub events_dblrdy: EVENTS_DBLRDY,
    #[doc = "0x110 - QDEC has been stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved1: [u8; 236usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved2: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 500usize],
    #[doc = "0x500 - Enable the quadrature decoder"]
    pub enable: ENABLE,
    #[doc = "0x504 - LED output pin polarity"]
    pub ledpol: LEDPOL,
    #[doc = "0x508 - Sample period"]
    pub sampleper: SAMPLEPER,
    #[doc = "0x50c - Motion sample value"]
    pub sample: SAMPLE,
    #[doc = "0x510 - Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
    pub reportper: REPORTPER,
    #[doc = "0x514 - Register accumulating the valid transitions"]
    pub acc: ACC,
    #[doc = "0x518 - Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
    pub accread: ACCREAD,
    #[doc = "0x51c - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x528 - Enable input debounce filters"]
    pub dbfen: DBFEN,
    _reserved4: [u8; 20usize],
    #[doc = "0x540 - Time period the LED is switched ON prior to sampling"]
    pub ledpre: LEDPRE,
    #[doc = "0x544 - Register accumulating the number of detected double transitions"]
    pub accdbl: ACCDBL,
    #[doc = "0x548 - Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
    pub accdblread: ACCDBLREAD,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for LED signal"]
    pub led: self::psel::LED,
    #[doc = "0x04 - Pin select for A signal"]
    pub a: self::psel::A,
    #[doc = "0x08 - Pin select for B signal"]
    pub b: self::psel::B,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Task starting the quadrature decoder"]
pub struct TASKS_START {
    register: VolatileCell<u32>,
}
#[doc = "Task starting the quadrature decoder"]
pub mod tasks_start;
#[doc = "Task stopping the quadrature decoder"]
pub struct TASKS_STOP {
    register: VolatileCell<u32>,
}
#[doc = "Task stopping the quadrature decoder"]
pub mod tasks_stop;
#[doc = "Read and clear ACC and ACCDBL"]
pub struct TASKS_READCLRACC {
    register: VolatileCell<u32>,
}
#[doc = "Read and clear ACC and ACCDBL"]
pub mod tasks_readclracc;
#[doc = "Read and clear ACC"]
pub struct TASKS_RDCLRACC {
    register: VolatileCell<u32>,
}
#[doc = "Read and clear ACC"]
pub mod tasks_rdclracc;
#[doc = "Read and clear ACCDBL"]
pub struct TASKS_RDCLRDBL {
    register: VolatileCell<u32>,
}
#[doc = "Read and clear ACCDBL"]
pub mod tasks_rdclrdbl;
#[doc = "Event being generated for every new sample value written to the SAMPLE register"]
pub struct EVENTS_SAMPLERDY {
    register: VolatileCell<u32>,
}
#[doc = "Event being generated for every new sample value written to the SAMPLE register"]
pub mod events_samplerdy;
#[doc = "Non-null report ready"]
pub struct EVENTS_REPORTRDY {
    register: VolatileCell<u32>,
}
#[doc = "Non-null report ready"]
pub mod events_reportrdy;
#[doc = "ACC or ACCDBL register overflow"]
pub struct EVENTS_ACCOF {
    register: VolatileCell<u32>,
}
#[doc = "ACC or ACCDBL register overflow"]
pub mod events_accof;
#[doc = "Double displacement(s) detected"]
pub struct EVENTS_DBLRDY {
    register: VolatileCell<u32>,
}
#[doc = "Double displacement(s) detected"]
pub mod events_dblrdy;
#[doc = "QDEC has been stopped"]
pub struct EVENTS_STOPPED {
    register: VolatileCell<u32>,
}
#[doc = "QDEC has been stopped"]
pub mod events_stopped;
#[doc = "Shortcut register"]
pub struct SHORTS {
    register: VolatileCell<u32>,
}
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Enable the quadrature decoder"]
pub struct ENABLE {
    register: VolatileCell<u32>,
}
#[doc = "Enable the quadrature decoder"]
pub mod enable;
#[doc = "LED output pin polarity"]
pub struct LEDPOL {
    register: VolatileCell<u32>,
}
#[doc = "LED output pin polarity"]
pub mod ledpol;
#[doc = "Sample period"]
pub struct SAMPLEPER {
    register: VolatileCell<u32>,
}
#[doc = "Sample period"]
pub mod sampleper;
#[doc = "Motion sample value"]
pub struct SAMPLE {
    register: VolatileCell<u32>,
}
#[doc = "Motion sample value"]
pub mod sample;
#[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
pub struct REPORTPER {
    register: VolatileCell<u32>,
}
#[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
pub mod reportper;
#[doc = "Register accumulating the valid transitions"]
pub struct ACC {
    register: VolatileCell<u32>,
}
#[doc = "Register accumulating the valid transitions"]
pub mod acc;
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
pub struct ACCREAD {
    register: VolatileCell<u32>,
}
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
pub mod accread;
#[doc = "Enable input debounce filters"]
pub struct DBFEN {
    register: VolatileCell<u32>,
}
#[doc = "Enable input debounce filters"]
pub mod dbfen;
#[doc = "Time period the LED is switched ON prior to sampling"]
pub struct LEDPRE {
    register: VolatileCell<u32>,
}
#[doc = "Time period the LED is switched ON prior to sampling"]
pub mod ledpre;
#[doc = "Register accumulating the number of detected double transitions"]
pub struct ACCDBL {
    register: VolatileCell<u32>,
}
#[doc = "Register accumulating the number of detected double transitions"]
pub mod accdbl;
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
pub struct ACCDBLREAD {
    register: VolatileCell<u32>,
}
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
pub mod accdblread;
