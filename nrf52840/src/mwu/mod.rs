#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Unspecified"]
    pub events_region: [EVENTS_REGION; 4],
    _reserved1: [u8; 64usize],
    #[doc = "0x160 - Unspecified"]
    pub events_pregion: [EVENTS_PREGION; 2],
    _reserved2: [u8; 400usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 20usize],
    #[doc = "0x320 - Enable or disable non-maskable interrupt"]
    pub nmien: NMIEN,
    #[doc = "0x324 - Enable non-maskable interrupt"]
    pub nmienset: NMIENSET,
    #[doc = "0x328 - Disable non-maskable interrupt"]
    pub nmienclr: NMIENCLR,
    _reserved8: [u8; 212usize],
    #[doc = "0x400 - Unspecified"]
    pub perregion: [PERREGION; 2],
    _reserved9: [u8; 256usize],
    #[doc = "0x510 - Enable/disable regions watch"]
    pub regionen: REGIONEN,
    #[doc = "0x514 - Enable regions watch"]
    pub regionenset: REGIONENSET,
    #[doc = "0x518 - Disable regions watch"]
    pub regionenclr: REGIONENCLR,
    _reserved12: [u8; 228usize],
    #[doc = "0x600 - Unspecified"]
    pub region0: REGION,
    _reserved13: [u8; 8usize],
    #[doc = "0x610 - Unspecified"]
    pub region1: REGION,
    _reserved14: [u8; 8usize],
    #[doc = "0x620 - Unspecified"]
    pub region2: REGION,
    _reserved15: [u8; 8usize],
    #[doc = "0x630 - Unspecified"]
    pub region3: REGION,
    _reserved16: [u8; 136usize],
    #[doc = "0x6c0 - Unspecified"]
    pub pregion0: PREGION,
    _reserved17: [u8; 4usize],
    #[doc = "0x6d0 - Unspecified"]
    pub pregion1: PREGION,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct EVENTS_REGION {
    #[doc = "0x00 - Description cluster[n]: Write access to region n detected"]
    pub wa: self::events_region::WA,
    #[doc = "0x04 - Description cluster[n]: Read access to region n detected"]
    pub ra: self::events_region::RA,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod events_region;
#[doc = r" Register block"]
#[repr(C)]
pub struct EVENTS_PREGION {
    #[doc = "0x00 - Description cluster[n]: Write access to peripheral region n detected"]
    pub wa: self::events_pregion::WA,
    #[doc = "0x04 - Description cluster[n]: Read access to peripheral region n detected"]
    pub ra: self::events_pregion::RA,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod events_pregion;
#[doc = r" Register block"]
#[repr(C)]
pub struct PERREGION {
    #[doc = "0x00 - Description cluster[n]: Source of event/interrupt in region n, write access detected while corresponding subregion was enabled for watching"]
    pub substatwa: self::perregion::SUBSTATWA,
    #[doc = "0x04 - Description cluster[n]: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching"]
    pub substatra: self::perregion::SUBSTATRA,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod perregion;
#[doc = r" Register block"]
#[repr(C)]
pub struct REGION {
    #[doc = "0x00 - Description cluster[n]: Start address for region n"]
    pub start: self::region::START,
    #[doc = "0x04 - Description cluster[n]: End address of region n"]
    pub end: self::region::END,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod region;
#[doc = r" Register block"]
#[repr(C)]
pub struct PREGION {
    #[doc = "0x00 - Description cluster[n]: Reserved for future use"]
    pub start: self::pregion::START,
    #[doc = "0x04 - Description cluster[n]: Reserved for future use"]
    pub end: self::pregion::END,
    #[doc = "0x08 - Description cluster[n]: Subregions of region n"]
    pub subs: self::pregion::SUBS,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod pregion;
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
#[doc = "Enable or disable non-maskable interrupt"]
pub struct NMIEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable non-maskable interrupt"]
pub mod nmien;
#[doc = "Enable non-maskable interrupt"]
pub struct NMIENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable non-maskable interrupt"]
pub mod nmienset;
#[doc = "Disable non-maskable interrupt"]
pub struct NMIENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable non-maskable interrupt"]
pub mod nmienclr;
#[doc = "Enable/disable regions watch"]
pub struct REGIONEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable/disable regions watch"]
pub mod regionen;
#[doc = "Enable regions watch"]
pub struct REGIONENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable regions watch"]
pub mod regionenset;
#[doc = "Disable regions watch"]
pub struct REGIONENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable regions watch"]
pub mod regionenclr;
