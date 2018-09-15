#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start resolving addresses based on IRKs specified in the IRK data structure."]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Stop resolving addresses."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 244usize],
    #[doc = "0x100 - Address resolution procedure completed."]
    pub events_end: EVENTS_END,
    #[doc = "0x104 - Address resolved."]
    pub events_resolved: EVENTS_RESOLVED,
    #[doc = "0x108 - Address not resolved."]
    pub events_notresolved: EVENTS_NOTRESOLVED,
    _reserved5: [u8; 504usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 244usize],
    #[doc = "0x400 - Resolution status."]
    pub status: STATUS,
    _reserved8: [u8; 252usize],
    #[doc = "0x500 - Enable AAR."]
    pub enable: ENABLE,
    #[doc = "0x504 - Number of Identity root Keys in the IRK data structure."]
    pub nirk: NIRK,
    #[doc = "0x508 - Pointer to the IRK data structure."]
    pub irkptr: IRKPTR,
    _reserved11: [u8; 4usize],
    #[doc = "0x510 - Pointer to the resolvable address (6 bytes)."]
    pub addrptr: ADDRPTR,
    #[doc = "0x514 - Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
    pub scratchptr: SCRATCHPTR,
    _reserved13: [u8; 2788usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure."]
pub mod tasks_start;
#[doc = "Stop resolving addresses."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop resolving addresses."]
pub mod tasks_stop;
#[doc = "Address resolution procedure completed."]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address resolution procedure completed."]
pub mod events_end;
#[doc = "Address resolved."]
pub struct EVENTS_RESOLVED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address resolved."]
pub mod events_resolved;
#[doc = "Address not resolved."]
pub struct EVENTS_NOTRESOLVED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address not resolved."]
pub mod events_notresolved;
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
#[doc = "Resolution status."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resolution status."]
pub mod status;
#[doc = "Enable AAR."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable AAR."]
pub mod enable;
#[doc = "Number of Identity root Keys in the IRK data structure."]
pub struct NIRK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of Identity root Keys in the IRK data structure."]
pub mod nirk;
#[doc = "Pointer to the IRK data structure."]
pub struct IRKPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to the IRK data structure."]
pub mod irkptr;
#[doc = "Pointer to the resolvable address (6 bytes)."]
pub struct ADDRPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to the resolvable address (6 bytes)."]
pub mod addrptr;
#[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
pub struct SCRATCHPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
pub mod scratchptr;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
