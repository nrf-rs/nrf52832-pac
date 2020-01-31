#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel group tasks"]
    pub tasks_chg: [TASKS_CHG; 6],
    _reserved1: [u8; 1232usize],
    #[doc = "0x500 - Channel enable register"]
    pub chen: CHEN,
    #[doc = "0x504 - Channel enable set register"]
    pub chenset: CHENSET,
    #[doc = "0x508 - Channel enable clear register"]
    pub chenclr: CHENCLR,
    _reserved4: [u8; 4usize],
    #[doc = "0x510 - PPI Channel"]
    pub ch: [CH; 20],
    _reserved5: [u8; 592usize],
    #[doc = "0x800 - Description collection\\[0\\]: Channel group 0"]
    pub chg: [CHG; 6],
    _reserved6: [u8; 248usize],
    #[doc = "0x910 - Fork"]
    pub fork: [FORK; 32],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Description cluster\\[0\\]: Enable channel group 0"]
    pub en: self::tasks_chg::EN,
    #[doc = "0x04 - Description cluster\\[0\\]: Disable channel group 0"]
    pub dis: self::tasks_chg::DIS,
}
#[doc = r"Register block"]
#[doc = "Channel group tasks"]
pub mod tasks_chg;
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster\\[0\\]: Channel 0 event end-point"]
    pub eep: self::ch::EEP,
    #[doc = "0x04 - Description cluster\\[0\\]: Channel 0 task end-point"]
    pub tep: self::ch::TEP,
}
#[doc = r"Register block"]
#[doc = "PPI Channel"]
pub mod ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct FORK {
    #[doc = "0x00 - Description cluster\\[0\\]: Channel 0 task end-point"]
    pub tep: self::fork::TEP,
}
#[doc = r"Register block"]
#[doc = "Fork"]
pub mod fork;
#[doc = "Channel enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chen](chen) module"]
pub type CHEN = crate::Reg<u32, _CHEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEN;
#[doc = "`read()` method returns [chen::R](chen::R) reader structure"]
impl crate::Readable for CHEN {}
#[doc = "`write(|w| ..)` method takes [chen::W](chen::W) writer structure"]
impl crate::Writable for CHEN {}
#[doc = "Channel enable register"]
pub mod chen;
#[doc = "Channel enable set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chenset](chenset) module"]
pub type CHENSET = crate::Reg<u32, _CHENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHENSET;
#[doc = "`read()` method returns [chenset::R](chenset::R) reader structure"]
impl crate::Readable for CHENSET {}
#[doc = "`write(|w| ..)` method takes [chenset::W](chenset::W) writer structure"]
impl crate::Writable for CHENSET {}
#[doc = "Channel enable set register"]
pub mod chenset;
#[doc = "Channel enable clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chenclr](chenclr) module"]
pub type CHENCLR = crate::Reg<u32, _CHENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHENCLR;
#[doc = "`read()` method returns [chenclr::R](chenclr::R) reader structure"]
impl crate::Readable for CHENCLR {}
#[doc = "`write(|w| ..)` method takes [chenclr::W](chenclr::W) writer structure"]
impl crate::Writable for CHENCLR {}
#[doc = "Channel enable clear register"]
pub mod chenclr;
#[doc = "Description collection\\[0\\]: Channel group 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chg](chg) module"]
pub type CHG = crate::Reg<u32, _CHG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHG;
#[doc = "`read()` method returns [chg::R](chg::R) reader structure"]
impl crate::Readable for CHG {}
#[doc = "`write(|w| ..)` method takes [chg::W](chg::W) writer structure"]
impl crate::Writable for CHG {}
#[doc = "Description collection\\[0\\]: Channel group 0"]
pub mod chg;
