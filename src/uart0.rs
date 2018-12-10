#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start UART receiver"]
    pub tasks_startrx: TASKS_STARTRX,
    #[doc = "0x04 - Stop UART receiver"]
    pub tasks_stoprx: TASKS_STOPRX,
    #[doc = "0x08 - Start UART transmitter"]
    pub tasks_starttx: TASKS_STARTTX,
    #[doc = "0x0c - Stop UART transmitter"]
    pub tasks_stoptx: TASKS_STOPTX,
    _reserved0: [u8; 12usize],
    #[doc = "0x1c - Suspend UART"]
    pub tasks_suspend: TASKS_SUSPEND,
    _reserved1: [u8; 224usize],
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    pub events_cts: EVENTS_CTS,
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    pub events_ncts: EVENTS_NCTS,
    #[doc = "0x108 - Data received in RXD"]
    pub events_rxdrdy: EVENTS_RXDRDY,
    _reserved2: [u8; 16usize],
    #[doc = "0x11c - Data sent from TXD"]
    pub events_txdrdy: EVENTS_TXDRDY,
    _reserved3: [u8; 4usize],
    #[doc = "0x124 - Error detected"]
    pub events_error: EVENTS_ERROR,
    _reserved4: [u8; 28usize],
    #[doc = "0x144 - Receiver timeout"]
    pub events_rxto: EVENTS_RXTO,
    _reserved5: [u8; 184usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved6: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 372usize],
    #[doc = "0x480 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved8: [u8; 124usize],
    #[doc = "0x500 - Enable UART"]
    pub enable: ENABLE,
    _reserved9: [u8; 4usize],
    #[doc = "0x508 - Pin select for RTS"]
    pub pselrts: PSELRTS,
    #[doc = "0x50c - Pin select for TXD"]
    pub pseltxd: PSELTXD,
    #[doc = "0x510 - Pin select for CTS"]
    pub pselcts: PSELCTS,
    #[doc = "0x514 - Pin select for RXD"]
    pub pselrxd: PSELRXD,
    #[doc = "0x518 - RXD register"]
    pub rxd: RXD,
    #[doc = "0x51c - TXD register"]
    pub txd: TXD,
    _reserved10: [u8; 4usize],
    #[doc = "0x524 - Baud rate"]
    pub baudrate: BAUDRATE,
    _reserved11: [u8; 68usize],
    #[doc = "0x56c - Configuration of parity and hardware flow control"]
    pub config: CONFIG,
}
#[doc = "Start UART receiver"]
pub struct TASKS_STARTRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start UART receiver"]
pub mod tasks_startrx;
#[doc = "Stop UART receiver"]
pub struct TASKS_STOPRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop UART receiver"]
pub mod tasks_stoprx;
#[doc = "Start UART transmitter"]
pub struct TASKS_STARTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start UART transmitter"]
pub mod tasks_starttx;
#[doc = "Stop UART transmitter"]
pub struct TASKS_STOPTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop UART transmitter"]
pub mod tasks_stoptx;
#[doc = "Suspend UART"]
pub struct TASKS_SUSPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend UART"]
pub mod tasks_suspend;
#[doc = "CTS is activated (set low). Clear To Send."]
pub struct EVENTS_CTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTS is activated (set low). Clear To Send."]
pub mod events_cts;
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub struct EVENTS_NCTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub mod events_ncts;
#[doc = "Data received in RXD"]
pub struct EVENTS_RXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data received in RXD"]
pub mod events_rxdrdy;
#[doc = "Data sent from TXD"]
pub struct EVENTS_TXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "Error detected"]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error detected"]
pub mod events_error;
#[doc = "Receiver timeout"]
pub struct EVENTS_RXTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver timeout"]
pub mod events_rxto;
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
#[doc = "Enable UART"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable UART"]
pub mod enable;
#[doc = "Pin select for RTS"]
pub struct PSELRTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for RTS"]
pub mod pselrts;
#[doc = "Pin select for TXD"]
pub struct PSELTXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for TXD"]
pub mod pseltxd;
#[doc = "Pin select for CTS"]
pub struct PSELCTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for CTS"]
pub mod pselcts;
#[doc = "Pin select for RXD"]
pub struct PSELRXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for RXD"]
pub mod pselrxd;
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
#[doc = "Baud rate"]
pub struct BAUDRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud rate"]
pub mod baudrate;
#[doc = "Configuration of parity and hardware flow control"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of parity and hardware flow control"]
pub mod config;
