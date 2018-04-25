use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 36usize],
    #[doc = "0x24 - Acquire SPI semaphore"]
    pub tasks_acquire: TASKS_ACQUIRE,
    #[doc = "0x28 - Release SPI semaphore, enabling the SPI slave to acquire it"]
    pub tasks_release: TASKS_RELEASE,
    _reserved1: [u8; 216usize],
    #[doc = "0x104 - Granted transaction completed"]
    pub events_end: EVENTS_END,
    _reserved2: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved3: [u8; 20usize],
    #[doc = "0x128 - Semaphore acquired"]
    pub events_acquired: EVENTS_ACQUIRED,
    _reserved4: [u8; 212usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved5: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 244usize],
    #[doc = "0x400 - Semaphore status register"]
    pub semstat: SEMSTAT,
    _reserved7: [u8; 60usize],
    #[doc = "0x440 - Status from last transaction"]
    pub status: STATUS,
    _reserved8: [u8; 188usize],
    #[doc = "0x500 - Enable SPI slave"]
    pub enable: ENABLE,
    _reserved9: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved10: [u8; 28usize],
    #[doc = "0x534 - Unspecified"]
    pub rxd: RXD,
    _reserved11: [u8; 4usize],
    #[doc = "0x544 - Unspecified"]
    pub txd: TXD,
    _reserved12: [u8; 4usize],
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved13: [u8; 4usize],
    #[doc = "0x55c - Default character. Character clocked out in case of an ignored transaction."]
    pub def: DEF,
    _reserved14: [u8; 96usize],
    #[doc = "0x5c0 - Over-read character"]
    pub orc: ORC,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MISO signal"]
    pub miso: self::psel::MISO,
    #[doc = "0x08 - Pin select for MOSI signal"]
    pub mosi: self::psel::MOSI,
    #[doc = "0x0c - Pin select for CSN signal"]
    pub csn: self::psel::CSN,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - RXD data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes received in last granted transaction"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - TXD data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transmitted in last granted transaction"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "Acquire SPI semaphore"]
pub struct TASKS_ACQUIRE {
    register: VolatileCell<u32>,
}
#[doc = "Acquire SPI semaphore"]
pub mod tasks_acquire;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub struct TASKS_RELEASE {
    register: VolatileCell<u32>,
}
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub mod tasks_release;
#[doc = "Granted transaction completed"]
pub struct EVENTS_END {
    register: VolatileCell<u32>,
}
#[doc = "Granted transaction completed"]
pub mod events_end;
#[doc = "End of RXD buffer reached"]
pub struct EVENTS_ENDRX {
    register: VolatileCell<u32>,
}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "Semaphore acquired"]
pub struct EVENTS_ACQUIRED {
    register: VolatileCell<u32>,
}
#[doc = "Semaphore acquired"]
pub mod events_acquired;
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
#[doc = "Semaphore status register"]
pub struct SEMSTAT {
    register: VolatileCell<u32>,
}
#[doc = "Semaphore status register"]
pub mod semstat;
#[doc = "Status from last transaction"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status from last transaction"]
pub mod status;
#[doc = "Enable SPI slave"]
pub struct ENABLE {
    register: VolatileCell<u32>,
}
#[doc = "Enable SPI slave"]
pub mod enable;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub struct DEF {
    register: VolatileCell<u32>,
}
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub mod def;
#[doc = "Over-read character"]
pub struct ORC {
    register: VolatileCell<u32>,
}
#[doc = "Over-read character"]
pub mod orc;
