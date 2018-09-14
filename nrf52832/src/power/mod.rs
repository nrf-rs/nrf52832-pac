#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 120usize],
    #[doc = "0x78 - Enable constant latency mode"]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)"]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 136usize],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved3: [u8; 8usize],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: EVENTS_SLEEPENTER,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: EVENTS_SLEEPEXIT,
    _reserved5: [u8; 488usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 244usize],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved8: [u8; 36usize],
    #[doc = "0x428 - Deprecated register - RAM status register"]
    pub ramstatus: RAMSTATUS,
    _reserved9: [u8; 212usize],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved10: [u8; 12usize],
    #[doc = "0x510 - Power failure comparator configuration"]
    pub pofcon: POFCON,
    _reserved11: [u8; 8usize],
    #[doc = "0x51c - General purpose retention register"]
    pub gpregret: GPREGRET,
    #[doc = "0x520 - General purpose retention register"]
    pub gpregret2: GPREGRET2,
    #[doc = "0x524 - Deprecated register - RAM on/off register (this register is retained)"]
    pub ramon: RAMON,
    _reserved14: [u8; 44usize],
    #[doc = "0x554 - Deprecated register - RAM on/off register (this register is retained)"]
    pub ramonb: RAMONB,
    _reserved15: [u8; 32usize],
    #[doc = "0x578 - DC/DC enable register"]
    pub dcdcen: DCDCEN,
    _reserved16: [u8; 900usize],
    #[doc = "0x900 - Unspecified"]
    pub ram0: RAM,
    _reserved17: [u8; 4usize],
    #[doc = "0x910 - Unspecified"]
    pub ram1: RAM,
    _reserved18: [u8; 4usize],
    #[doc = "0x920 - Unspecified"]
    pub ram2: RAM,
    _reserved19: [u8; 4usize],
    #[doc = "0x930 - Unspecified"]
    pub ram3: RAM,
    _reserved20: [u8; 4usize],
    #[doc = "0x940 - Unspecified"]
    pub ram4: RAM,
    _reserved21: [u8; 4usize],
    #[doc = "0x950 - Unspecified"]
    pub ram5: RAM,
    _reserved22: [u8; 4usize],
    #[doc = "0x960 - Unspecified"]
    pub ram6: RAM,
    _reserved23: [u8; 4usize],
    #[doc = "0x970 - Unspecified"]
    pub ram7: RAM,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster[0]: RAM0 power control register"]
    pub power: self::ram::POWER,
    #[doc = "0x04 - Description cluster[0]: RAM0 power control set register"]
    pub powerset: self::ram::POWERSET,
    #[doc = "0x08 - Description cluster[0]: RAM0 power control clear register"]
    pub powerclr: self::ram::POWERCLR,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod ram;
#[doc = "Enable constant latency mode"]
pub struct TASKS_CONSTLAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable constant latency mode"]
pub mod tasks_constlat;
#[doc = "Enable low power mode (variable latency)"]
pub struct TASKS_LOWPWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable low power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "Power failure warning"]
pub struct EVENTS_POFWARN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "CPU entered WFI/WFE sleep"]
pub struct EVENTS_SLEEPENTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "CPU exited WFI/WFE sleep"]
pub struct EVENTS_SLEEPEXIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
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
#[doc = "Reset reason"]
pub struct RESETREAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "Deprecated register - RAM status register"]
pub struct RAMSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - RAM status register"]
pub mod ramstatus;
#[doc = "System OFF register"]
pub struct SYSTEMOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "Power failure comparator configuration"]
pub struct POFCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power failure comparator configuration"]
pub mod pofcon;
#[doc = "General purpose retention register"]
pub struct GPREGRET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose retention register"]
pub mod gpregret;
#[doc = "General purpose retention register"]
pub struct GPREGRET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose retention register"]
pub mod gpregret2;
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub struct RAMON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub mod ramon;
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub struct RAMONB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub mod ramonb;
#[doc = "DC/DC enable register"]
pub struct DCDCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DC/DC enable register"]
pub mod dcdcen;
