#[doc = "Part code\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [part](part) module"]
pub type PART = crate::Reg<u32, _PART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PART;
#[doc = "`read()` method returns [part::R](part::R) reader structure"]
impl crate::Readable for PART {}
#[doc = "Part code"]
pub mod part;
#[doc = "Part Variant, Hardware version and Production configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [variant](variant) module"]
pub type VARIANT = crate::Reg<u32, _VARIANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VARIANT;
#[doc = "`read()` method returns [variant::R](variant::R) reader structure"]
impl crate::Readable for VARIANT {}
#[doc = "Part Variant, Hardware version and Production configuration"]
pub mod variant;
#[doc = "Package option\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [package](package) module"]
pub type PACKAGE = crate::Reg<u32, _PACKAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKAGE;
#[doc = "`read()` method returns [package::R](package::R) reader structure"]
impl crate::Readable for PACKAGE {}
#[doc = "Package option"]
pub mod package;
#[doc = "RAM variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram](ram) module"]
pub type RAM = crate::Reg<u32, _RAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM;
#[doc = "`read()` method returns [ram::R](ram::R) reader structure"]
impl crate::Readable for RAM {}
#[doc = "RAM variant"]
pub mod ram;
#[doc = "Flash variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash](flash) module"]
pub type FLASH = crate::Reg<u32, _FLASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH;
#[doc = "`read()` method returns [flash::R](flash::R) reader structure"]
impl crate::Readable for FLASH {}
#[doc = "Flash variant"]
pub mod flash;
#[doc = "Description collection\\[0\\]: Unspecified\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused0](unused0) module"]
pub type UNUSED0 = crate::Reg<u32, _UNUSED0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNUSED0;
#[doc = "`read()` method returns [unused0::R](unused0::R) reader structure"]
impl crate::Readable for UNUSED0 {}
#[doc = "`write(|w| ..)` method takes [unused0::W](unused0::W) writer structure"]
impl crate::Writable for UNUSED0 {}
#[doc = "Description collection\\[0\\]: Unspecified"]
pub mod unused0;
