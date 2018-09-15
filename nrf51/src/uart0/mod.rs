#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start UART receiver."]
    pub tasks_startrx: TASKS_STARTRX,
    #[doc = "0x04 - Stop UART receiver."]
    pub tasks_stoprx: TASKS_STOPRX,
    #[doc = "0x08 - Start UART transmitter."]
    pub tasks_starttx: TASKS_STARTTX,
    #[doc = "0x0c - Stop UART transmitter."]
    pub tasks_stoptx: TASKS_STOPTX,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - Suspend UART."]
    pub tasks_suspend: TASKS_SUSPEND,
    _reserved5: [u8; 224usize],
    #[doc = "0x100 - CTS activated."]
    pub events_cts: EVENTS_CTS,
    #[doc = "0x104 - CTS deactivated."]
    pub events_ncts: EVENTS_NCTS,
    #[doc = "0x108 - Data received in RXD."]
    pub events_rxdrdy: EVENTS_RXDRDY,
    _reserved8: [u8; 16usize],
    #[doc = "0x11c - Data sent from TXD."]
    pub events_txdrdy: EVENTS_TXDRDY,
    _reserved9: [u8; 4usize],
    #[doc = "0x124 - Error detected."]
    pub events_error: EVENTS_ERROR,
    _reserved10: [u8; 28usize],
    #[doc = "0x144 - Receiver timeout."]
    pub events_rxto: EVENTS_RXTO,
    _reserved11: [u8; 184usize],
    #[doc = "0x200 - Shortcuts for UART."]
    pub shorts: SHORTS,
    _reserved12: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 372usize],
    #[doc = "0x480 - Error source. Write error field to 1 to clear error."]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 124usize],
    #[doc = "0x500 - Enable UART and acquire IOs."]
    pub enable: ENABLE,
    _reserved16: [u8; 4usize],
    #[doc = "0x508 - Pin select for RTS."]
    pub pselrts: PSELRTS,
    #[doc = "0x50c - Pin select for TXD."]
    pub pseltxd: PSELTXD,
    #[doc = "0x510 - Pin select for CTS."]
    pub pselcts: PSELCTS,
    #[doc = "0x514 - Pin select for RXD."]
    pub pselrxd: PSELRXD,
    #[doc = "0x518 - RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working."]
    pub rxd: RXD,
    #[doc = "0x51c - TXD register."]
    pub txd: TXD,
    _reserved22: [u8; 4usize],
    #[doc = "0x524 - UART Baudrate."]
    pub baudrate: BAUDRATE,
    _reserved23: [u8; 68usize],
    #[doc = "0x56c - Configuration of parity and hardware flow control register."]
    pub config: CONFIG,
    _reserved24: [u8; 2700usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start UART receiver."]
pub struct TASKS_STARTRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start UART receiver."]
pub mod tasks_startrx;
#[doc = "Stop UART receiver."]
pub struct TASKS_STOPRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop UART receiver."]
pub mod tasks_stoprx;
#[doc = "Start UART transmitter."]
pub struct TASKS_STARTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start UART transmitter."]
pub mod tasks_starttx;
#[doc = "Stop UART transmitter."]
pub struct TASKS_STOPTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop UART transmitter."]
pub mod tasks_stoptx;
#[doc = "Suspend UART."]
pub struct TASKS_SUSPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend UART."]
pub mod tasks_suspend;
#[doc = "CTS activated."]
pub struct EVENTS_CTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTS activated."]
pub mod events_cts;
#[doc = "CTS deactivated."]
pub struct EVENTS_NCTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTS deactivated."]
pub mod events_ncts;
#[doc = "Data received in RXD."]
pub struct EVENTS_RXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data received in RXD."]
pub mod events_rxdrdy;
#[doc = "Data sent from TXD."]
pub struct EVENTS_TXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data sent from TXD."]
pub mod events_txdrdy;
#[doc = "Error detected."]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error detected."]
pub mod events_error;
#[doc = "Receiver timeout."]
pub struct EVENTS_RXTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver timeout."]
pub mod events_rxto;
#[doc = "Shortcuts for UART."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts for UART."]
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
#[doc = "Error source. Write error field to 1 to clear error."]
pub struct ERRORSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error source. Write error field to 1 to clear error."]
pub mod errorsrc;
#[doc = "Enable UART and acquire IOs."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable UART and acquire IOs."]
pub mod enable;
#[doc = "Pin select for RTS."]
pub struct PSELRTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for RTS."]
pub mod pselrts;
#[doc = "Pin select for TXD."]
pub struct PSELTXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for TXD."]
pub mod pseltxd;
#[doc = "Pin select for CTS."]
pub struct PSELCTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for CTS."]
pub mod pselcts;
#[doc = "Pin select for RXD."]
pub struct PSELRXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for RXD."]
pub mod pselrxd;
#[doc = "RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working."]
pub struct RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working."]
pub mod rxd;
#[doc = "TXD register."]
pub struct TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXD register."]
pub mod txd;
#[doc = "UART Baudrate."]
pub struct BAUDRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Baudrate."]
pub mod baudrate;
#[doc = "Configuration of parity and hardware flow control register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of parity and hardware flow control register."]
pub mod config;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
