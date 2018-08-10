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
    _reserved0: [u8; 28usize],
    #[doc = "0x2c - Flush RX FIFO into RX buffer"]
    pub tasks_flushrx: TASKS_FLUSHRX,
    _reserved1: [u8; 208usize],
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    pub events_cts: EVENTS_CTS,
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    pub events_ncts: EVENTS_NCTS,
    #[doc = "0x108 - Data received in RXD (but potentially not yet transferred to Data RAM)"]
    pub events_rxdrdy: EVENTS_RXDRDY,
    _reserved2: [u8; 4usize],
    #[doc = "0x110 - Receive buffer is filled up"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved3: [u8; 8usize],
    #[doc = "0x11c - Data sent from TXD"]
    pub events_txdrdy: EVENTS_TXDRDY,
    #[doc = "0x120 - Last TX byte transmitted"]
    pub events_endtx: EVENTS_ENDTX,
    #[doc = "0x124 - Error detected"]
    pub events_error: EVENTS_ERROR,
    _reserved4: [u8; 28usize],
    #[doc = "0x144 - Receiver timeout"]
    pub events_rxto: EVENTS_RXTO,
    _reserved5: [u8; 4usize],
    #[doc = "0x14c - UART receiver has started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - UART transmitter has started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved6: [u8; 4usize],
    #[doc = "0x158 - Transmitter stopped"]
    pub events_txstopped: EVENTS_TXSTOPPED,
    _reserved7: [u8; 164usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved8: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 372usize],
    #[doc = "0x480 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved10: [u8; 124usize],
    #[doc = "0x500 - Enable UART"]
    pub enable: ENABLE,
    _reserved11: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved12: [u8; 12usize],
    #[doc = "0x524 - Baud rate. Accuracy depends on the HFCLK source selected."]
    pub baudrate: BAUDRATE,
    _reserved13: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    _reserved14: [u8; 4usize],
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved15: [u8; 28usize],
    #[doc = "0x56c - Configuration of parity and hardware flow control"]
    pub config: CONFIG,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for RTS signal"]
    pub rts: self::psel::RTS,
    #[doc = "0x04 - Pin select for TXD signal"]
    pub txd: self::psel::TXD,
    #[doc = "0x08 - Pin select for CTS signal"]
    pub cts: self::psel::CTS,
    #[doc = "0x0c - Pin select for RXD signal"]
    pub rxd: self::psel::RXD,
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
}
#[doc = r" Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
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
#[doc = "Flush RX FIFO into RX buffer"]
pub struct TASKS_FLUSHRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flush RX FIFO into RX buffer"]
pub mod tasks_flushrx;
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
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub struct EVENTS_RXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub mod events_rxdrdy;
#[doc = "Receive buffer is filled up"]
pub struct EVENTS_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive buffer is filled up"]
pub mod events_endrx;
#[doc = "Data sent from TXD"]
pub struct EVENTS_TXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "Last TX byte transmitted"]
pub struct EVENTS_ENDTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last TX byte transmitted"]
pub mod events_endtx;
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
#[doc = "UART receiver has started"]
pub struct EVENTS_RXSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART receiver has started"]
pub mod events_rxstarted;
#[doc = "UART transmitter has started"]
pub struct EVENTS_TXSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART transmitter has started"]
pub mod events_txstarted;
#[doc = "Transmitter stopped"]
pub struct EVENTS_TXSTOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter stopped"]
pub mod events_txstopped;
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
#[doc = "Enable UART"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable UART"]
pub mod enable;
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
pub struct BAUDRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
pub mod baudrate;
#[doc = "Configuration of parity and hardware flow control"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of parity and hardware flow control"]
pub mod config;
