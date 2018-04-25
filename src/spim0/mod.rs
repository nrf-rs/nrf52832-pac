use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Start SPI transaction"]
    pub tasks_start: TASKS_START,
    #[doc = "0x14 - Stop SPI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - Suspend SPI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume SPI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved2: [u8; 224usize],
    #[doc = "0x104 - SPI transaction has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved3: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved4: [u8; 4usize],
    #[doc = "0x118 - End of RXD buffer and TXD buffer reached"]
    pub events_end: EVENTS_END,
    _reserved5: [u8; 4usize],
    #[doc = "0x120 - End of TXD buffer reached"]
    pub events_endtx: EVENTS_ENDTX,
    _reserved6: [u8; 40usize],
    #[doc = "0x14c - Transaction started"]
    pub events_started: EVENTS_STARTED,
    _reserved7: [u8; 176usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved8: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 500usize],
    #[doc = "0x500 - Enable SPIM"]
    pub enable: ENABLE,
    _reserved10: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved11: [u8; 16usize],
    #[doc = "0x524 - SPI frequency"]
    pub frequency: FREQUENCY,
    _reserved12: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved13: [u8; 104usize],
    #[doc = "0x5c0 - Over-read character. Character clocked out in case and over-read of the TXD buffer."]
    pub orc: ORC,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MOSI signal"]
    pub mosi: self::psel::MOSI,
    #[doc = "0x08 - Pin select for MISO signal"]
    pub miso: self::psel::MISO,
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
#[doc = "Start SPI transaction"]
pub struct TASKS_START {
    register: VolatileCell<u32>,
}
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "Stop SPI transaction"]
pub struct TASKS_STOP {
    register: VolatileCell<u32>,
}
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "Suspend SPI transaction"]
pub struct TASKS_SUSPEND {
    register: VolatileCell<u32>,
}
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "Resume SPI transaction"]
pub struct TASKS_RESUME {
    register: VolatileCell<u32>,
}
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "SPI transaction has stopped"]
pub struct EVENTS_STOPPED {
    register: VolatileCell<u32>,
}
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "End of RXD buffer reached"]
pub struct EVENTS_ENDRX {
    register: VolatileCell<u32>,
}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub struct EVENTS_END {
    register: VolatileCell<u32>,
}
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "End of TXD buffer reached"]
pub struct EVENTS_ENDTX {
    register: VolatileCell<u32>,
}
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "Transaction started"]
pub struct EVENTS_STARTED {
    register: VolatileCell<u32>,
}
#[doc = "Transaction started"]
pub mod events_started;
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
#[doc = "Enable SPIM"]
pub struct ENABLE {
    register: VolatileCell<u32>,
}
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "SPI frequency"]
pub struct FREQUENCY {
    register: VolatileCell<u32>,
}
#[doc = "SPI frequency"]
pub mod frequency;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub struct ORC {
    register: VolatileCell<u32>,
}
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub mod orc;
