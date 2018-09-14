#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Activate QSPI interface"]
    pub tasks_activate: TASKS_ACTIVATE,
    #[doc = "0x04 - Start transfer from external flash memory to internal RAM"]
    pub tasks_readstart: TASKS_READSTART,
    #[doc = "0x08 - Start transfer from internal RAM to external flash memory"]
    pub tasks_writestart: TASKS_WRITESTART,
    #[doc = "0x0c - Start external flash memory erase operation"]
    pub tasks_erasestart: TASKS_ERASESTART,
    #[doc = "0x10 - Deactivate QSPI interface"]
    pub tasks_deactivate: TASKS_DEACTIVATE,
    _reserved5: [u8; 236usize],
    #[doc = "0x100 - QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
    pub events_ready: EVENTS_READY,
    _reserved6: [u8; 508usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 500usize],
    #[doc = "0x500 - Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
    pub enable: ENABLE,
    #[doc = "0x504 - Unspecified"]
    pub read: READ,
    #[doc = "0x510 - Unspecified"]
    pub write: WRITE,
    #[doc = "0x51c - Unspecified"]
    pub erase: ERASE,
    #[doc = "0x524 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x540 - Address offset into the external memory for Execute in Place operation."]
    pub xipoffset: XIPOFFSET,
    #[doc = "0x544 - Interface configuration."]
    pub ifconfig0: IFCONFIG0,
    _reserved16: [u8; 184usize],
    #[doc = "0x600 - Interface configuration."]
    pub ifconfig1: IFCONFIG1,
    #[doc = "0x604 - Status register."]
    pub status: STATUS,
    _reserved18: [u8; 12usize],
    #[doc = "0x614 - Set the duration required to enter/exit deep power-down mode (DPM)."]
    pub dpmdur: DPMDUR,
    _reserved19: [u8; 12usize],
    #[doc = "0x624 - Extended address configuration."]
    pub addrconf: ADDRCONF,
    _reserved20: [u8; 12usize],
    #[doc = "0x634 - Custom instruction configuration register."]
    pub cinstrconf: CINSTRCONF,
    #[doc = "0x638 - Custom instruction data register 0."]
    pub cinstrdat0: CINSTRDAT0,
    #[doc = "0x63c - Custom instruction data register 1."]
    pub cinstrdat1: CINSTRDAT1,
    #[doc = "0x640 - SPI interface timing."]
    pub iftiming: IFTIMING,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct READ {
    #[doc = "0x00 - Flash memory source address"]
    pub src: self::read::SRC,
    #[doc = "0x04 - RAM destination address"]
    pub dst: self::read::DST,
    #[doc = "0x08 - Read transfer length"]
    pub cnt: self::read::CNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod read;
#[doc = r" Register block"]
#[repr(C)]
pub struct WRITE {
    #[doc = "0x00 - Flash destination address"]
    pub dst: self::write::DST,
    #[doc = "0x04 - RAM source address"]
    pub src: self::write::SRC,
    #[doc = "0x08 - Write transfer length"]
    pub cnt: self::write::CNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod write;
#[doc = r" Register block"]
#[repr(C)]
pub struct ERASE {
    #[doc = "0x00 - Start address of flash block to be erased"]
    pub ptr: self::erase::PTR,
    #[doc = "0x04 - Size of block to be erased."]
    pub len: self::erase::LEN,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod erase;
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for serial clock SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for chip select signal CSN."]
    pub csn: self::psel::CSN,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Pin select for serial data MOSI/IO0."]
    pub io0: self::psel::IO0,
    #[doc = "0x10 - Pin select for serial data MISO/IO1."]
    pub io1: self::psel::IO1,
    #[doc = "0x14 - Pin select for serial data IO2."]
    pub io2: self::psel::IO2,
    #[doc = "0x18 - Pin select for serial data IO3."]
    pub io3: self::psel::IO3,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Activate QSPI interface"]
pub struct TASKS_ACTIVATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Activate QSPI interface"]
pub mod tasks_activate;
#[doc = "Start transfer from external flash memory to internal RAM"]
pub struct TASKS_READSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start transfer from external flash memory to internal RAM"]
pub mod tasks_readstart;
#[doc = "Start transfer from internal RAM to external flash memory"]
pub struct TASKS_WRITESTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start transfer from internal RAM to external flash memory"]
pub mod tasks_writestart;
#[doc = "Start external flash memory erase operation"]
pub struct TASKS_ERASESTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start external flash memory erase operation"]
pub mod tasks_erasestart;
#[doc = "Deactivate QSPI interface"]
pub struct TASKS_DEACTIVATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deactivate QSPI interface"]
pub mod tasks_deactivate;
#[doc = "QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
pub mod events_ready;
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
#[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
pub mod enable;
#[doc = "Address offset into the external memory for Execute in Place operation."]
pub struct XIPOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address offset into the external memory for Execute in Place operation."]
pub mod xipoffset;
#[doc = "Interface configuration."]
pub struct IFCONFIG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface configuration."]
pub mod ifconfig0;
#[doc = "Interface configuration."]
pub struct IFCONFIG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface configuration."]
pub mod ifconfig1;
#[doc = "Status register."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register."]
pub mod status;
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM)."]
pub struct DPMDUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM)."]
pub mod dpmdur;
#[doc = "Extended address configuration."]
pub struct ADDRCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended address configuration."]
pub mod addrconf;
#[doc = "Custom instruction configuration register."]
pub struct CINSTRCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Custom instruction configuration register."]
pub mod cinstrconf;
#[doc = "Custom instruction data register 0."]
pub struct CINSTRDAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Custom instruction data register 0."]
pub mod cinstrdat0;
#[doc = "Custom instruction data register 1."]
pub struct CINSTRDAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Custom instruction data register 1."]
pub mod cinstrdat1;
#[doc = "SPI interface timing."]
pub struct IFTIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI interface timing."]
pub mod iftiming;
