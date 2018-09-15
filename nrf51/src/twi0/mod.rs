#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start 2-Wire master receive sequence."]
    pub tasks_startrx: TASKS_STARTRX,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Start 2-Wire master transmit sequence."]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - Stop 2-Wire transaction."]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x1c - Suspend 2-Wire transaction."]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume 2-Wire transaction."]
    pub tasks_resume: TASKS_RESUME,
    _reserved5: [u8; 224usize],
    #[doc = "0x104 - Two-wire stopped."]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - Two-wire ready to deliver new RXD byte received."]
    pub events_rxdready: EVENTS_RXDREADY,
    _reserved7: [u8; 16usize],
    #[doc = "0x11c - Two-wire finished sending last TXD byte."]
    pub events_txdsent: EVENTS_TXDSENT,
    _reserved8: [u8; 4usize],
    #[doc = "0x124 - Two-wire error detected."]
    pub events_error: EVENTS_ERROR,
    _reserved9: [u8; 16usize],
    #[doc = "0x138 - Two-wire byte boundary."]
    pub events_bb: EVENTS_BB,
    _reserved10: [u8; 12usize],
    #[doc = "0x148 - Two-wire suspended."]
    pub events_suspended: EVENTS_SUSPENDED,
    _reserved11: [u8; 180usize],
    #[doc = "0x200 - Shortcuts for TWI."]
    pub shorts: SHORTS,
    _reserved12: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 440usize],
    #[doc = "0x4c4 - Two-wire error source. Write error field to 1 to clear error."]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 56usize],
    #[doc = "0x500 - Enable two-wire master."]
    pub enable: ENABLE,
    _reserved16: [u8; 4usize],
    #[doc = "0x508 - Pin select for SCL."]
    pub pselscl: PSELSCL,
    #[doc = "0x50c - Pin select for SDA."]
    pub pselsda: PSELSDA,
    _reserved18: [u8; 8usize],
    #[doc = "0x518 - RX data register."]
    pub rxd: RXD,
    #[doc = "0x51c - TX data register."]
    pub txd: TXD,
    _reserved20: [u8; 4usize],
    #[doc = "0x524 - Two-wire frequency."]
    pub frequency: FREQUENCY,
    _reserved21: [u8; 96usize],
    #[doc = "0x588 - Address used in the two-wire transfer."]
    pub address: ADDRESS,
    _reserved22: [u8; 2672usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start 2-Wire master receive sequence."]
pub struct TASKS_STARTRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start 2-Wire master receive sequence."]
pub mod tasks_startrx;
#[doc = "Start 2-Wire master transmit sequence."]
pub struct TASKS_STARTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start 2-Wire master transmit sequence."]
pub mod tasks_starttx;
#[doc = "Stop 2-Wire transaction."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop 2-Wire transaction."]
pub mod tasks_stop;
#[doc = "Suspend 2-Wire transaction."]
pub struct TASKS_SUSPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend 2-Wire transaction."]
pub mod tasks_suspend;
#[doc = "Resume 2-Wire transaction."]
pub struct TASKS_RESUME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resume 2-Wire transaction."]
pub mod tasks_resume;
#[doc = "Two-wire stopped."]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Two-wire stopped."]
pub mod events_stopped;
#[doc = "Two-wire ready to deliver new RXD byte received."]
pub struct EVENTS_RXDREADY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Two-wire ready to deliver new RXD byte received."]
pub mod events_rxdready;
#[doc = "Two-wire finished sending last TXD byte."]
pub struct EVENTS_TXDSENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Two-wire finished sending last TXD byte."]
pub mod events_txdsent;
#[doc = "Two-wire error detected."]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Two-wire error detected."]
pub mod events_error;
#[doc = "Two-wire byte boundary."]
pub struct EVENTS_BB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Two-wire byte boundary."]
pub mod events_bb;
#[doc = "Two-wire suspended."]
pub struct EVENTS_SUSPENDED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Two-wire suspended."]
pub mod events_suspended;
#[doc = "Shortcuts for TWI."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts for TWI."]
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
#[doc = "Two-wire error source. Write error field to 1 to clear error."]
pub struct ERRORSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Two-wire error source. Write error field to 1 to clear error."]
pub mod errorsrc;
#[doc = "Enable two-wire master."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable two-wire master."]
pub mod enable;
#[doc = "Pin select for SCL."]
pub struct PSELSCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for SCL."]
pub mod pselscl;
#[doc = "Pin select for SDA."]
pub struct PSELSDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for SDA."]
pub mod pselsda;
#[doc = "RX data register."]
pub struct RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX data register."]
pub mod rxd;
#[doc = "TX data register."]
pub struct TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX data register."]
pub mod txd;
#[doc = "Two-wire frequency."]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Two-wire frequency."]
pub mod frequency;
#[doc = "Address used in the two-wire transfer."]
pub struct ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address used in the two-wire transfer."]
pub mod address;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
