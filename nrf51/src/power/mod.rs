#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 120usize],
    #[doc = "0x78 - Enable constant latency mode."]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)."]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 136usize],
    #[doc = "0x108 - Power failure warning."]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved3: [u8; 504usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 244usize],
    #[doc = "0x400 - Reset reason."]
    pub resetreas: RESETREAS,
    _reserved6: [u8; 36usize],
    #[doc = "0x428 - Ram status register."]
    pub ramstatus: RAMSTATUS,
    _reserved7: [u8; 212usize],
    #[doc = "0x500 - System off register."]
    pub systemoff: SYSTEMOFF,
    _reserved8: [u8; 12usize],
    #[doc = "0x510 - Power failure configuration."]
    pub pofcon: POFCON,
    _reserved9: [u8; 8usize],
    #[doc = "0x51c - General purpose retention register. This register is a retained register."]
    pub gpregret: GPREGRET,
    _reserved10: [u8; 4usize],
    #[doc = "0x524 - Ram on/off."]
    pub ramon: RAMON,
    _reserved11: [u8; 28usize],
    #[doc = "0x544 - Pin reset functionality configuration register. This register is a retained register."]
    pub reset: RESET,
    _reserved12: [u8; 12usize],
    #[doc = "0x554 - Ram on/off."]
    pub ramonb: RAMONB,
    _reserved13: [u8; 32usize],
    #[doc = "0x578 - DCDC converter enable configuration register."]
    pub dcdcen: DCDCEN,
    _reserved14: [u8; 1164usize],
    #[doc = "0xa08 - DCDC power-up force register."]
    pub dcdcforce: DCDCFORCE,
}
#[doc = "Enable constant latency mode."]
pub struct TASKS_CONSTLAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable constant latency mode."]
pub mod tasks_constlat;
#[doc = "Enable low power mode (variable latency)."]
pub struct TASKS_LOWPWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable low power mode (variable latency)."]
pub mod tasks_lowpwr;
#[doc = "Power failure warning."]
pub struct EVENTS_POFWARN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power failure warning."]
pub mod events_pofwarn;
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
#[doc = "Reset reason."]
pub struct RESETREAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset reason."]
pub mod resetreas;
#[doc = "Ram status register."]
pub struct RAMSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram status register."]
pub mod ramstatus;
#[doc = "System off register."]
pub struct SYSTEMOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System off register."]
pub mod systemoff;
#[doc = "Power failure configuration."]
pub struct POFCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power failure configuration."]
pub mod pofcon;
#[doc = "General purpose retention register. This register is a retained register."]
pub struct GPREGRET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose retention register. This register is a retained register."]
pub mod gpregret;
#[doc = "Ram on/off."]
pub struct RAMON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram on/off."]
pub mod ramon;
#[doc = "Pin reset functionality configuration register. This register is a retained register."]
pub struct RESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin reset functionality configuration register. This register is a retained register."]
pub mod reset;
#[doc = "Ram on/off."]
pub struct RAMONB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram on/off."]
pub mod ramonb;
#[doc = "DCDC converter enable configuration register."]
pub struct DCDCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC converter enable configuration register."]
pub mod dcdcen;
#[doc = "DCDC power-up force register."]
pub struct DCDCFORCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC power-up force register."]
pub mod dcdcforce;
