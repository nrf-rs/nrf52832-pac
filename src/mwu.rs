#[doc = r"Register block"]
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
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_REGION {
    #[doc = "0x00 - Description cluster\\[0\\]: Write access to region 0 detected"]
    pub wa: self::events_region::WA,
    #[doc = "0x04 - Description cluster\\[0\\]: Read access to region 0 detected"]
    pub ra: self::events_region::RA,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod events_region;
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_PREGION {
    #[doc = "0x00 - Description cluster\\[0\\]: Write access to peripheral region 0 detected"]
    pub wa: self::events_pregion::WA,
    #[doc = "0x04 - Description cluster\\[0\\]: Read access to peripheral region 0 detected"]
    pub ra: self::events_pregion::RA,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod events_pregion;
#[doc = r"Register block"]
#[repr(C)]
pub struct PERREGION {
    #[doc = "0x00 - Description cluster\\[0\\]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching"]
    pub substatwa: self::perregion::SUBSTATWA,
    #[doc = "0x04 - Description cluster\\[0\\]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching"]
    pub substatra: self::perregion::SUBSTATRA,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod perregion;
#[doc = r"Register block"]
#[repr(C)]
pub struct REGION {
    #[doc = "0x00 - Description cluster\\[0\\]: Start address for region 0"]
    pub start: self::region::START,
    #[doc = "0x04 - Description cluster\\[0\\]: End address of region 0"]
    pub end: self::region::END,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod region;
#[doc = r"Register block"]
#[repr(C)]
pub struct PREGION {
    #[doc = "0x00 - Description cluster\\[0\\]: Reserved for future use"]
    pub start: self::pregion::START,
    #[doc = "0x04 - Description cluster\\[0\\]: Reserved for future use"]
    pub end: self::pregion::END,
    #[doc = "0x08 - Description cluster\\[0\\]: Subregions of region 0"]
    pub subs: self::pregion::SUBS,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod pregion;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Enable or disable non-maskable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmien](nmien) module"]
pub type NMIEN = crate::Reg<u32, _NMIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMIEN;
#[doc = "`read()` method returns [nmien::R](nmien::R) reader structure"]
impl crate::Readable for NMIEN {}
#[doc = "`write(|w| ..)` method takes [nmien::W](nmien::W) writer structure"]
impl crate::Writable for NMIEN {}
#[doc = "Enable or disable non-maskable interrupt"]
pub mod nmien;
#[doc = "Enable non-maskable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmienset](nmienset) module"]
pub type NMIENSET = crate::Reg<u32, _NMIENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMIENSET;
#[doc = "`read()` method returns [nmienset::R](nmienset::R) reader structure"]
impl crate::Readable for NMIENSET {}
#[doc = "`write(|w| ..)` method takes [nmienset::W](nmienset::W) writer structure"]
impl crate::Writable for NMIENSET {}
#[doc = "Enable non-maskable interrupt"]
pub mod nmienset;
#[doc = "Disable non-maskable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmienclr](nmienclr) module"]
pub type NMIENCLR = crate::Reg<u32, _NMIENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMIENCLR;
#[doc = "`read()` method returns [nmienclr::R](nmienclr::R) reader structure"]
impl crate::Readable for NMIENCLR {}
#[doc = "`write(|w| ..)` method takes [nmienclr::W](nmienclr::W) writer structure"]
impl crate::Writable for NMIENCLR {}
#[doc = "Disable non-maskable interrupt"]
pub mod nmienclr;
#[doc = "Enable/disable regions watch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regionen](regionen) module"]
pub type REGIONEN = crate::Reg<u32, _REGIONEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGIONEN;
#[doc = "`read()` method returns [regionen::R](regionen::R) reader structure"]
impl crate::Readable for REGIONEN {}
#[doc = "`write(|w| ..)` method takes [regionen::W](regionen::W) writer structure"]
impl crate::Writable for REGIONEN {}
#[doc = "Enable/disable regions watch"]
pub mod regionen;
#[doc = "Enable regions watch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regionenset](regionenset) module"]
pub type REGIONENSET = crate::Reg<u32, _REGIONENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGIONENSET;
#[doc = "`read()` method returns [regionenset::R](regionenset::R) reader structure"]
impl crate::Readable for REGIONENSET {}
#[doc = "`write(|w| ..)` method takes [regionenset::W](regionenset::W) writer structure"]
impl crate::Writable for REGIONENSET {}
#[doc = "Enable regions watch"]
pub mod regionenset;
#[doc = "Disable regions watch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regionenclr](regionenclr) module"]
pub type REGIONENCLR = crate::Reg<u32, _REGIONENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGIONENCLR;
#[doc = "`read()` method returns [regionenclr::R](regionenclr::R) reader structure"]
impl crate::Readable for REGIONENCLR {}
#[doc = "`write(|w| ..)` method takes [regionenclr::W](regionenclr::W) writer structure"]
impl crate::Writable for REGIONENCLR {}
#[doc = "Disable regions watch"]
pub mod regionenclr;
