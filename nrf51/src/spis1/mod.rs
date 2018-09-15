#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 36usize],
    #[doc = "0x24 - Acquire SPI semaphore."]
    pub tasks_acquire: TASKS_ACQUIRE,
    #[doc = "0x28 - Release SPI semaphore."]
    pub tasks_release: TASKS_RELEASE,
    _reserved2: [u8; 216usize],
    #[doc = "0x104 - Granted transaction completed."]
    pub events_end: EVENTS_END,
    _reserved3: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved4: [u8; 20usize],
    #[doc = "0x128 - Semaphore acquired."]
    pub events_acquired: EVENTS_ACQUIRED,
    _reserved5: [u8; 212usize],
    #[doc = "0x200 - Shortcuts for SPIS."]
    pub shorts: SHORTS,
    _reserved6: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 244usize],
    #[doc = "0x400 - Semaphore status."]
    pub semstat: SEMSTAT,
    _reserved9: [u8; 60usize],
    #[doc = "0x440 - Status from last transaction."]
    pub status: STATUS,
    _reserved10: [u8; 188usize],
    #[doc = "0x500 - Enable SPIS."]
    pub enable: ENABLE,
    _reserved11: [u8; 4usize],
    #[doc = "0x508 - Pin select for SCK."]
    pub pselsck: PSELSCK,
    #[doc = "0x50c - Pin select for MISO."]
    pub pselmiso: PSELMISO,
    #[doc = "0x510 - Pin select for MOSI."]
    pub pselmosi: PSELMOSI,
    #[doc = "0x514 - Pin select for CSN."]
    pub pselcsn: PSELCSN,
    _reserved15: [u8; 28usize],
    #[doc = "0x534 - RX data pointer."]
    pub rxdptr: RXDPTR,
    #[doc = "0x538 - Maximum number of bytes in the receive buffer."]
    pub maxrx: MAXRX,
    #[doc = "0x53c - Number of bytes received in last granted transaction."]
    pub amountrx: AMOUNTRX,
    _reserved18: [u8; 4usize],
    #[doc = "0x544 - TX data pointer."]
    pub txdptr: TXDPTR,
    #[doc = "0x548 - Maximum number of bytes in the transmit buffer."]
    pub maxtx: MAXTX,
    #[doc = "0x54c - Number of bytes transmitted in last granted transaction."]
    pub amounttx: AMOUNTTX,
    _reserved21: [u8; 4usize],
    #[doc = "0x554 - Configuration register."]
    pub config: CONFIG,
    _reserved22: [u8; 4usize],
    #[doc = "0x55c - Default character."]
    pub def: DEF,
    _reserved23: [u8; 96usize],
    #[doc = "0x5c0 - Over-read character."]
    pub orc: ORC,
    _reserved24: [u8; 2616usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Acquire SPI semaphore."]
pub struct TASKS_ACQUIRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Acquire SPI semaphore."]
pub mod tasks_acquire;
#[doc = "Release SPI semaphore."]
pub struct TASKS_RELEASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Release SPI semaphore."]
pub mod tasks_release;
#[doc = "Granted transaction completed."]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Granted transaction completed."]
pub mod events_end;
#[doc = "End of RXD buffer reached"]
pub struct EVENTS_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "Semaphore acquired."]
pub struct EVENTS_ACQUIRED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore acquired."]
pub mod events_acquired;
#[doc = "Shortcuts for SPIS."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts for SPIS."]
pub mod shorts;
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
#[doc = "Semaphore status."]
pub struct SEMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore status."]
pub mod semstat;
#[doc = "Status from last transaction."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status from last transaction."]
pub mod status;
#[doc = "Enable SPIS."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable SPIS."]
pub mod enable;
#[doc = "Pin select for SCK."]
pub struct PSELSCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for SCK."]
pub mod pselsck;
#[doc = "Pin select for MISO."]
pub struct PSELMISO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for MISO."]
pub mod pselmiso;
#[doc = "Pin select for MOSI."]
pub struct PSELMOSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for MOSI."]
pub mod pselmosi;
#[doc = "Pin select for CSN."]
pub struct PSELCSN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for CSN."]
pub mod pselcsn;
#[doc = "RX data pointer."]
pub struct RXDPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX data pointer."]
pub mod rxdptr;
#[doc = "Maximum number of bytes in the receive buffer."]
pub struct MAXRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in the receive buffer."]
pub mod maxrx;
#[doc = "Number of bytes received in last granted transaction."]
pub struct AMOUNTRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of bytes received in last granted transaction."]
pub mod amountrx;
#[doc = "TX data pointer."]
pub struct TXDPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX data pointer."]
pub mod txdptr;
#[doc = "Maximum number of bytes in the transmit buffer."]
pub struct MAXTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in the transmit buffer."]
pub mod maxtx;
#[doc = "Number of bytes transmitted in last granted transaction."]
pub struct AMOUNTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of bytes transmitted in last granted transaction."]
pub mod amounttx;
#[doc = "Configuration register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register."]
pub mod config;
#[doc = "Default character."]
pub struct DEF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Default character."]
pub mod def;
#[doc = "Over-read character."]
pub struct ORC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over-read character."]
pub mod orc;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
