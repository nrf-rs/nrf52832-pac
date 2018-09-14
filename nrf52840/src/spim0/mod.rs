#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Start SPI transaction"]
    pub tasks_start: TASKS_START,
    #[doc = "0x14 - Stop SPI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 4usize],
    #[doc = "0x1c - Suspend SPI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume SPI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved4: [u8; 224usize],
    #[doc = "0x104 - SPI transaction has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved5: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - End of RXD buffer and TXD buffer reached"]
    pub events_end: EVENTS_END,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - End of TXD buffer reached"]
    pub events_endtx: EVENTS_ENDTX,
    _reserved8: [u8; 40usize],
    #[doc = "0x14c - Transaction started"]
    pub events_started: EVENTS_STARTED,
    _reserved9: [u8; 176usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved10: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved12: [u8; 244usize],
    #[doc = "0x400 - Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
    pub stallstat: STALLSTAT,
    _reserved13: [u8; 252usize],
    #[doc = "0x500 - Enable SPIM"]
    pub enable: ENABLE,
    _reserved14: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved15: [u8; 12usize],
    #[doc = "0x524 - SPI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: FREQUENCY,
    _reserved16: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved19: [u8; 8usize],
    #[doc = "0x560 - Unspecified"]
    pub iftiming: IFTIMING,
    #[doc = "0x568 - Polarity of CSN output"]
    pub csnpol: CSNPOL,
    #[doc = "0x56c - Pin select for DCX signal"]
    pub pseldcx: PSELDCX,
    #[doc = "0x570 - DCX configuration"]
    pub dcxcnt: DCXCNT,
    _reserved23: [u8; 76usize],
    #[doc = "0x5c0 - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
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
    #[doc = "0x0c - Pin select for CSN"]
    pub csn: self::psel::CSN,
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
    #[doc = "0x04 - Number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::txd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::txd::LIST,
}
#[doc = r" Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = r" Register block"]
#[repr(C)]
pub struct IFTIMING {
    #[doc = "0x00 - Sample delay for input serial data on MISO"]
    pub rxdelay: self::iftiming::RXDELAY,
    #[doc = "0x04 - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
    pub csndur: self::iftiming::CSNDUR,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod iftiming;
#[doc = "Start SPI transaction"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "Stop SPI transaction"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "Suspend SPI transaction"]
pub struct TASKS_SUSPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "Resume SPI transaction"]
pub struct TASKS_RESUME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "SPI transaction has stopped"]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "End of RXD buffer reached"]
pub struct EVENTS_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "End of TXD buffer reached"]
pub struct EVENTS_ENDTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "Transaction started"]
pub struct EVENTS_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transaction started"]
pub mod events_started;
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
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
pub struct STALLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
pub mod stallstat;
#[doc = "Enable SPIM"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Polarity of CSN output"]
pub struct CSNPOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Polarity of CSN output"]
pub mod csnpol;
#[doc = "Pin select for DCX signal"]
pub struct PSELDCX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for DCX signal"]
pub mod pseldcx;
#[doc = "DCX configuration"]
pub struct DCXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCX configuration"]
pub mod dcxcnt;
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
pub struct ORC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
pub mod orc;
