#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 264usize],
    #[doc = "0x108 - TXD byte sent and RXD byte received"]
    pub events_ready: EVENTS_READY,
    _reserved1: [u8; 504usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved2: [u8; 500usize],
    #[doc = "0x500 - Enable SPI"]
    pub enable: ENABLE,
    _reserved3: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved4: [u8; 4usize],
    #[doc = "0x518 - RXD register"]
    pub rxd: RXD,
    #[doc = "0x51c - TXD register"]
    pub txd: TXD,
    _reserved5: [u8; 4usize],
    #[doc = "0x524 - SPI frequency"]
    pub frequency: FREQUENCY,
    _reserved6: [u8; 44usize],
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MOSI"]
    pub mosi: self::psel::MOSI,
    #[doc = "0x08 - Pin select for MISO"]
    pub miso: self::psel::MISO,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TXD byte sent and RXD byte received"]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXD byte sent and RXD byte received"]
pub mod events_ready;
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
#[doc = "Enable SPI"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable SPI"]
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
#[doc = "SPI frequency"]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI frequency"]
pub mod frequency;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
