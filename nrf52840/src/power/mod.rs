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
    #[doc = "0x11c - Voltage supply detected on VBUS"]
    pub events_usbdetected: EVENTS_USBDETECTED,
    #[doc = "0x120 - Voltage supply removed from VBUS"]
    pub events_usbremoved: EVENTS_USBREMOVED,
    #[doc = "0x124 - USB 3.3 V supply ready"]
    pub events_usbpwrrdy: EVENTS_USBPWRRDY,
    _reserved8: [u8; 476usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 244usize],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved11: [u8; 36usize],
    #[doc = "0x428 - Deprecated register - RAM status register"]
    pub ramstatus: RAMSTATUS,
    _reserved12: [u8; 12usize],
    #[doc = "0x438 - USB supply status"]
    pub usbregstatus: USBREGSTATUS,
    _reserved13: [u8; 196usize],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved14: [u8; 12usize],
    #[doc = "0x510 - Power-fail comparator configuration"]
    pub pofcon: POFCON,
    _reserved15: [u8; 8usize],
    #[doc = "0x51c - General purpose retention register"]
    pub gpregret: GPREGRET,
    #[doc = "0x520 - General purpose retention register"]
    pub gpregret2: GPREGRET2,
    _reserved17: [u8; 84usize],
    #[doc = "0x578 - Enable DC/DC converter for REG1 stage."]
    pub dcdcen: DCDCEN,
    _reserved18: [u8; 4usize],
    #[doc = "0x580 - Enable DC/DC converter for REG0 stage."]
    pub dcdcen0: DCDCEN0,
    _reserved19: [u8; 188usize],
    #[doc = "0x640 - Main supply status"]
    pub mainregstatus: MAINREGSTATUS,
    _reserved20: [u8; 700usize],
    #[doc = "0x900 - Unspecified"]
    pub ram0: RAM,
    _reserved21: [u8; 4usize],
    #[doc = "0x910 - Unspecified"]
    pub ram1: RAM,
    _reserved22: [u8; 4usize],
    #[doc = "0x920 - Unspecified"]
    pub ram2: RAM,
    _reserved23: [u8; 4usize],
    #[doc = "0x930 - Unspecified"]
    pub ram3: RAM,
    _reserved24: [u8; 4usize],
    #[doc = "0x940 - Unspecified"]
    pub ram4: RAM,
    _reserved25: [u8; 4usize],
    #[doc = "0x950 - Unspecified"]
    pub ram5: RAM,
    _reserved26: [u8; 4usize],
    #[doc = "0x960 - Unspecified"]
    pub ram6: RAM,
    _reserved27: [u8; 4usize],
    #[doc = "0x970 - Unspecified"]
    pub ram7: RAM,
    _reserved28: [u8; 4usize],
    #[doc = "0x980 - Unspecified"]
    pub ram8: RAM,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster[n]: RAMn power control register"]
    pub power: self::ram::POWER,
    #[doc = "0x04 - Description cluster[n]: RAMn power control set register"]
    pub powerset: self::ram::POWERSET,
    #[doc = "0x08 - Description cluster[n]: RAMn power control clear register"]
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
#[doc = "Voltage supply detected on VBUS"]
pub struct EVENTS_USBDETECTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage supply detected on VBUS"]
pub mod events_usbdetected;
#[doc = "Voltage supply removed from VBUS"]
pub struct EVENTS_USBREMOVED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage supply removed from VBUS"]
pub mod events_usbremoved;
#[doc = "USB 3.3 V supply ready"]
pub struct EVENTS_USBPWRRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB 3.3 V supply ready"]
pub mod events_usbpwrrdy;
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
#[doc = "USB supply status"]
pub struct USBREGSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB supply status"]
pub mod usbregstatus;
#[doc = "System OFF register"]
pub struct SYSTEMOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "Power-fail comparator configuration"]
pub struct POFCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-fail comparator configuration"]
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
#[doc = "Enable DC/DC converter for REG1 stage."]
pub struct DCDCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable DC/DC converter for REG1 stage."]
pub mod dcdcen;
#[doc = "Enable DC/DC converter for REG0 stage."]
pub struct DCDCEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable DC/DC converter for REG0 stage."]
pub mod dcdcen0;
#[doc = "Main supply status"]
pub struct MAINREGSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main supply status"]
pub mod mainregstatus;
