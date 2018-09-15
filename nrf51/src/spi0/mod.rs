#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 264usize],
    #[doc = "0x108 - TXD byte sent and RXD byte received."]
    pub events_ready: EVENTS_READY,
    _reserved1: [u8; 504usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 500usize],
    #[doc = "0x500 - Enable SPI."]
    pub enable: ENABLE,
    _reserved4: [u8; 4usize],
    #[doc = "0x508 - Pin select for SCK."]
    pub pselsck: PSELSCK,
    #[doc = "0x50c - Pin select for MOSI."]
    pub pselmosi: PSELMOSI,
    #[doc = "0x510 - Pin select for MISO."]
    pub pselmiso: PSELMISO,
    _reserved7: [u8; 4usize],
    #[doc = "0x518 - RX data."]
    pub rxd: RXD,
    #[doc = "0x51c - TX data."]
    pub txd: TXD,
    _reserved9: [u8; 4usize],
    #[doc = "0x524 - SPI frequency"]
    pub frequency: FREQUENCY,
    _reserved10: [u8; 44usize],
    #[doc = "0x554 - Configuration register."]
    pub config: CONFIG,
    _reserved11: [u8; 2724usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TXD byte sent and RXD byte received."]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXD byte sent and RXD byte received."]
pub mod events_ready;
#[doc = "Interrupt enable set register."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "Interrupt enable clear register."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "Enable SPI."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable SPI."]
pub mod enable;
#[doc = "Pin select for SCK."]
pub struct PSELSCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for SCK."]
pub mod pselsck;
#[doc = "Pin select for MOSI."]
pub struct PSELMOSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for MOSI."]
pub mod pselmosi;
#[doc = "Pin select for MISO."]
pub struct PSELMISO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for MISO."]
pub mod pselmiso;
#[doc = "RX data."]
pub struct RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX data."]
pub mod rxd;
#[doc = "TX data."]
pub struct TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX data."]
pub mod txd;
#[doc = "SPI frequency"]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI frequency"]
pub mod frequency;
#[doc = "Configuration register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register."]
pub mod config;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
