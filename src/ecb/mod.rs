#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start ECB block encrypt"]
    pub tasks_startecb: TASKS_STARTECB,
    #[doc = "0x04 - Abort a possible executing ECB operation"]
    pub tasks_stopecb: TASKS_STOPECB,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - ECB block encrypt complete"]
    pub events_endecb: EVENTS_ENDECB,
    #[doc = "0x104 - ECB block encrypt aborted because of a STOPECB task or due to an error"]
    pub events_errorecb: EVENTS_ERRORECB,
    _reserved4: [u8; 508usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 504usize],
    #[doc = "0x504 - ECB block encrypt memory pointers"]
    pub ecbdataptr: ECBDATAPTR,
}
#[doc = "Start ECB block encrypt"]
pub struct TASKS_STARTECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start ECB block encrypt"]
pub mod tasks_startecb;
#[doc = "Abort a possible executing ECB operation"]
pub struct TASKS_STOPECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Abort a possible executing ECB operation"]
pub mod tasks_stopecb;
#[doc = "ECB block encrypt complete"]
pub struct EVENTS_ENDECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECB block encrypt complete"]
pub mod events_endecb;
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub struct EVENTS_ERRORECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub mod events_errorecb;
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
#[doc = "ECB block encrypt memory pointers"]
pub struct ECBDATAPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECB block encrypt memory pointers"]
pub mod ecbdataptr;
