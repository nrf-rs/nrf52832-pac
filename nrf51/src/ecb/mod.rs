#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
    pub tasks_startecb: TASKS_STARTECB,
    #[doc = "0x04 - Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
    pub tasks_stopecb: TASKS_STOPECB,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - ECB block encrypt complete."]
    pub events_endecb: EVENTS_ENDECB,
    #[doc = "0x104 - ECB block encrypt aborted due to a STOPECB task or due to an error."]
    pub events_errorecb: EVENTS_ERRORECB,
    _reserved4: [u8; 508usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 504usize],
    #[doc = "0x504 - ECB block encrypt memory pointer."]
    pub ecbdataptr: ECBDATAPTR,
    _reserved7: [u8; 2804usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
pub struct TASKS_STARTECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
pub mod tasks_startecb;
#[doc = "Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
pub struct TASKS_STOPECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
pub mod tasks_stopecb;
#[doc = "ECB block encrypt complete."]
pub struct EVENTS_ENDECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECB block encrypt complete."]
pub mod events_endecb;
#[doc = "ECB block encrypt aborted due to a STOPECB task or due to an error."]
pub struct EVENTS_ERRORECB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECB block encrypt aborted due to a STOPECB task or due to an error."]
pub mod events_errorecb;
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
#[doc = "ECB block encrypt memory pointer."]
pub struct ECBDATAPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECB block encrypt memory pointer."]
pub mod ecbdataptr;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
