#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start resolving addresses based on IRKs specified in the IRK data structure"]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Stop resolving addresses"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 244usize],
    #[doc = "0x100 - Address resolution procedure complete"]
    pub events_end: EVENTS_END,
    #[doc = "0x104 - Address resolved"]
    pub events_resolved: EVENTS_RESOLVED,
    #[doc = "0x108 - Address not resolved"]
    pub events_notresolved: EVENTS_NOTRESOLVED,
    _reserved5: [u8; 504usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 244usize],
    #[doc = "0x400 - Resolution status"]
    pub status: STATUS,
    _reserved8: [u8; 252usize],
    #[doc = "0x500 - Enable AAR"]
    pub enable: ENABLE,
    #[doc = "0x504 - Number of IRKs"]
    pub nirk: NIRK,
    #[doc = "0x508 - Pointer to IRK data structure"]
    pub irkptr: IRKPTR,
    _reserved11: [u8; 4usize],
    #[doc = "0x510 - Pointer to the resolvable address"]
    pub addrptr: ADDRPTR,
    #[doc = "0x514 - Pointer to data area used for temporary storage"]
    pub scratchptr: SCRATCHPTR,
}
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure"]
pub mod tasks_start;
#[doc = "Stop resolving addresses"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop resolving addresses"]
pub mod tasks_stop;
#[doc = "Address resolution procedure complete"]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address resolution procedure complete"]
pub mod events_end;
#[doc = "Address resolved"]
pub struct EVENTS_RESOLVED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address resolved"]
pub mod events_resolved;
#[doc = "Address not resolved"]
pub struct EVENTS_NOTRESOLVED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address not resolved"]
pub mod events_notresolved;
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
#[doc = "Resolution status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resolution status"]
pub mod status;
#[doc = "Enable AAR"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable AAR"]
pub mod enable;
#[doc = "Number of IRKs"]
pub struct NIRK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of IRKs"]
pub mod nirk;
#[doc = "Pointer to IRK data structure"]
pub struct IRKPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to IRK data structure"]
pub mod irkptr;
#[doc = "Pointer to the resolvable address"]
pub struct ADDRPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to the resolvable address"]
pub mod addrptr;
#[doc = "Pointer to data area used for temporary storage"]
pub struct SCRATCHPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to data area used for temporary storage"]
pub mod scratchptr;
