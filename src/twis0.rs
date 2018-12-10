#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 20usize],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved2: [u8; 12usize],
    #[doc = "0x30 - Prepare the TWI slave to respond to a write command"]
    pub tasks_preparerx: TASKS_PREPARERX,
    #[doc = "0x34 - Prepare the TWI slave to respond to a read command"]
    pub tasks_preparetx: TASKS_PREPARETX,
    _reserved3: [u8; 204usize],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved4: [u8; 28usize],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved5: [u8; 36usize],
    #[doc = "0x14c - Receive sequence started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - Transmit sequence started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved6: [u8; 16usize],
    #[doc = "0x164 - Write command received"]
    pub events_write: EVENTS_WRITE,
    #[doc = "0x168 - Read command received"]
    pub events_read: EVENTS_READ,
    _reserved7: [u8; 148usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved8: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 452usize],
    #[doc = "0x4d0 - Error source"]
    pub errorsrc: ERRORSRC,
    #[doc = "0x4d4 - Status register indicating which address had a match"]
    pub match_: MATCH,
    _reserved10: [u8; 40usize],
    #[doc = "0x500 - Enable TWIS"]
    pub enable: ENABLE,
    _reserved11: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved12: [u8; 36usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    _reserved13: [u8; 4usize],
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved14: [u8; 56usize],
    #[doc = "0x588 - Description collection\\[0\\]: TWI slave address 0"]
    pub address: [ADDRESS; 2],
    _reserved15: [u8; 4usize],
    #[doc = "0x594 - Configuration register for the address match mechanism"]
    pub config: CONFIG,
    _reserved16: [u8; 40usize],
    #[doc = "0x5c0 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    pub orc: ORC,
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
    #[doc = "0x00 - RXD Data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in RXD buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last RXD transaction"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - TXD Data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in TXD buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last TXD transaction"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
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
#[doc = "Prepare the TWI slave to respond to a write command"]
pub struct TASKS_PREPARERX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prepare the TWI slave to respond to a write command"]
pub mod tasks_preparerx;
#[doc = "Prepare the TWI slave to respond to a read command"]
pub struct TASKS_PREPARETX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prepare the TWI slave to respond to a read command"]
pub mod tasks_preparetx;
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
#[doc = "Write command received"]
pub struct EVENTS_WRITE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write command received"]
pub mod events_write;
#[doc = "Read command received"]
pub struct EVENTS_READ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read command received"]
pub mod events_read;
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
#[doc = "Status register indicating which address had a match"]
pub struct MATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register indicating which address had a match"]
pub mod match_;
#[doc = "Enable TWIS"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable TWIS"]
pub mod enable;
#[doc = "Description collection\\[0\\]: TWI slave address 0"]
pub struct ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection\\[0\\]: TWI slave address 0"]
pub mod address;
#[doc = "Configuration register for the address match mechanism"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register for the address match mechanism"]
pub mod config;
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub struct ORC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub mod orc;
