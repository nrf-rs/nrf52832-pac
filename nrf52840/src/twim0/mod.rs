#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start TWI receive sequence"]
    pub tasks_startrx: TASKS_STARTRX,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Start TWI transmit sequence"]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - Stop TWI transaction. Must be issued while the TWI master is not suspended."]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved5: [u8; 224usize],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved6: [u8; 28usize],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved7: [u8; 32usize],
    #[doc = "0x148 - Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
    pub events_suspended: EVENTS_SUSPENDED,
    #[doc = "0x14c - Receive sequence started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - Transmit sequence started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved10: [u8; 8usize],
    #[doc = "0x15c - Byte boundary, starting to receive the last byte"]
    pub events_lastrx: EVENTS_LASTRX,
    #[doc = "0x160 - Byte boundary, starting to transmit the last byte"]
    pub events_lasttx: EVENTS_LASTTX,
    _reserved12: [u8; 156usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved13: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved16: [u8; 440usize],
    #[doc = "0x4c4 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved17: [u8; 56usize],
    #[doc = "0x500 - Enable TWIM"]
    pub enable: ENABLE,
    _reserved18: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved19: [u8; 20usize],
    #[doc = "0x524 - TWI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: FREQUENCY,
    _reserved20: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved22: [u8; 52usize],
    #[doc = "0x588 - Address used in the TWI transfer"]
    pub address: ADDRESS,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCL signal"]
    pub scl: self::psel::SCL,
    #[doc = "0x04 - Pin select for SDA signal"]
    pub sda: self::psel::SDA,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::rxd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::rxd::LIST,
}
#[doc = r" Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::txd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::txd::LIST,
}
#[doc = r" Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
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
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
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
#[doc = "TWI error"]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TWI error"]
pub mod events_error;
#[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
pub struct EVENTS_SUSPENDED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
pub mod events_suspended;
#[doc = "Receive sequence started"]
pub struct EVENTS_RXSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "Transmit sequence started"]
pub struct EVENTS_TXSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "Byte boundary, starting to receive the last byte"]
pub struct EVENTS_LASTRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Byte boundary, starting to receive the last byte"]
pub mod events_lastrx;
#[doc = "Byte boundary, starting to transmit the last byte"]
pub struct EVENTS_LASTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Byte boundary, starting to transmit the last byte"]
pub mod events_lasttx;
#[doc = "Shortcut register"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcut register"]
pub mod shorts;
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
#[doc = "Error source"]
pub struct ERRORSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "Enable TWIM"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable TWIM"]
pub mod enable;
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
