#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start TWI receive sequence"]
    pub tasks_startrx: TASKS_STARTRX,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Start TWI transmit sequence"]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved5: [u8; 224usize],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - TWI RXD byte received"]
    pub events_rxdready: EVENTS_RXDREADY,
    _reserved7: [u8; 16usize],
    #[doc = "0x11c - TWI TXD byte sent"]
    pub events_txdsent: EVENTS_TXDSENT,
    _reserved8: [u8; 4usize],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved9: [u8; 16usize],
    #[doc = "0x138 - TWI byte boundary, generated before each byte that is sent or received"]
    pub events_bb: EVENTS_BB,
    _reserved10: [u8; 12usize],
    #[doc = "0x148 - TWI entered the suspended state"]
    pub events_suspended: EVENTS_SUSPENDED,
    _reserved11: [u8; 180usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved12: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 440usize],
    #[doc = "0x4c4 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 56usize],
    #[doc = "0x500 - Enable TWI"]
    pub enable: ENABLE,
    _reserved16: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved17: [u8; 8usize],
    #[doc = "0x518 - RXD register"]
    pub rxd: RXD,
    #[doc = "0x51c - TXD register"]
    pub txd: TXD,
    _reserved19: [u8; 4usize],
    #[doc = "0x524 - TWI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: FREQUENCY,
    _reserved20: [u8; 96usize],
    #[doc = "0x588 - Address used in the TWI transfer"]
    pub address: ADDRESS,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCL"]
    pub scl: self::psel::SCL,
    #[doc = "0x04 - Pin select for SDA"]
    pub sda: self::psel::SDA,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Start TWI receive sequence"]
pub struct TASKS_STARTRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start TWI receive sequence"]
pub mod tasks_startrx;
#[doc = "Start TWI transmit sequence"]
pub struct TASKS_STARTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start TWI transmit sequence"]
pub mod tasks_starttx;
#[doc = "Stop TWI transaction"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop TWI transaction"]
pub mod tasks_stop;
#[doc = "Suspend TWI transaction"]
pub struct TASKS_SUSPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "Resume TWI transaction"]
pub struct TASKS_RESUME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "TWI stopped"]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "TWI RXD byte received"]
pub struct EVENTS_RXDREADY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TWI RXD byte received"]
pub mod events_rxdready;
#[doc = "TWI TXD byte sent"]
pub struct EVENTS_TXDSENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TWI TXD byte sent"]
pub mod events_txdsent;
#[doc = "TWI error"]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TWI error"]
pub mod events_error;
#[doc = "TWI byte boundary, generated before each byte that is sent or received"]
pub struct EVENTS_BB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TWI byte boundary, generated before each byte that is sent or received"]
pub mod events_bb;
#[doc = "TWI entered the suspended state"]
pub struct EVENTS_SUSPENDED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TWI entered the suspended state"]
pub mod events_suspended;
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
#[doc = "Error source"]
pub struct ERRORSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "Enable TWI"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable TWI"]
pub mod enable;
#[doc = "RXD register"]
pub struct RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RXD register"]
pub mod rxd;
#[doc = "TXD register"]
pub struct TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXD register"]
pub mod txd;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "Address used in the TWI transfer"]
pub struct ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address used in the TWI transfer"]
pub mod address;
