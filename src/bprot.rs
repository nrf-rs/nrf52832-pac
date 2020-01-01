#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1536usize],
    #[doc = "0x600 - Block protect configuration register 0"]
    pub config0: CONFIG0,
    #[doc = "0x604 - Block protect configuration register 1"]
    pub config1: CONFIG1,
    #[doc = "0x608 - Disable protection mechanism in debug interface mode"]
    pub disableindebug: DISABLEINDEBUG,
    #[doc = "0x60c - Unspecified"]
    pub unused0: UNUSED0,
    #[doc = "0x610 - Block protect configuration register 2"]
    pub config2: CONFIG2,
    #[doc = "0x614 - Block protect configuration register 3"]
    pub config3: CONFIG3,
}
#[doc = "Block protect configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config0](config0) module"]
pub type CONFIG0 = crate::Reg<u32, _CONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG0;
#[doc = "`read()` method returns [config0::R](config0::R) reader structure"]
impl crate::Readable for CONFIG0 {}
#[doc = "`write(|w| ..)` method takes [config0::W](config0::W) writer structure"]
impl crate::Writable for CONFIG0 {}
#[doc = "Block protect configuration register 0"]
pub mod config0;
#[doc = "Block protect configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config1](config1) module"]
pub type CONFIG1 = crate::Reg<u32, _CONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG1;
#[doc = "`read()` method returns [config1::R](config1::R) reader structure"]
impl crate::Readable for CONFIG1 {}
#[doc = "`write(|w| ..)` method takes [config1::W](config1::W) writer structure"]
impl crate::Writable for CONFIG1 {}
#[doc = "Block protect configuration register 1"]
pub mod config1;
#[doc = "Disable protection mechanism in debug interface mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disableindebug](disableindebug) module"]
pub type DISABLEINDEBUG = crate::Reg<u32, _DISABLEINDEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DISABLEINDEBUG;
#[doc = "`read()` method returns [disableindebug::R](disableindebug::R) reader structure"]
impl crate::Readable for DISABLEINDEBUG {}
#[doc = "`write(|w| ..)` method takes [disableindebug::W](disableindebug::W) writer structure"]
impl crate::Writable for DISABLEINDEBUG {}
#[doc = "Disable protection mechanism in debug interface mode"]
pub mod disableindebug;
#[doc = "Unspecified\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused0](unused0) module"]
pub type UNUSED0 = crate::Reg<u32, _UNUSED0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNUSED0;
#[doc = "`read()` method returns [unused0::R](unused0::R) reader structure"]
impl crate::Readable for UNUSED0 {}
#[doc = "`write(|w| ..)` method takes [unused0::W](unused0::W) writer structure"]
impl crate::Writable for UNUSED0 {}
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "Block protect configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config2](config2) module"]
pub type CONFIG2 = crate::Reg<u32, _CONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG2;
#[doc = "`read()` method returns [config2::R](config2::R) reader structure"]
impl crate::Readable for CONFIG2 {}
#[doc = "`write(|w| ..)` method takes [config2::W](config2::W) writer structure"]
impl crate::Writable for CONFIG2 {}
#[doc = "Block protect configuration register 2"]
pub mod config2;
#[doc = "Block protect configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config3](config3) module"]
pub type CONFIG3 = crate::Reg<u32, _CONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG3;
#[doc = "`read()` method returns [config3::R](config3::R) reader structure"]
impl crate::Readable for CONFIG3 {}
#[doc = "`write(|w| ..)` method takes [config3::W](config3::W) writer structure"]
impl crate::Writable for CONFIG3 {}
#[doc = "Block protect configuration register 3"]
pub mod config3;
