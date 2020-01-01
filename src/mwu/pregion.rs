#[doc = "Description cluster\\[0\\]: Reserved for future use\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start](start) module"]
pub type START = crate::Reg<u32, _START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _START;
#[doc = "`read()` method returns [start::R](start::R) reader structure"]
impl crate::Readable for START {}
#[doc = "Description cluster\\[0\\]: Reserved for future use"]
pub mod start;
#[doc = "Description cluster\\[0\\]: Reserved for future use\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [end](end) module"]
pub type END = crate::Reg<u32, _END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _END;
#[doc = "`read()` method returns [end::R](end::R) reader structure"]
impl crate::Readable for END {}
#[doc = "Description cluster\\[0\\]: Reserved for future use"]
pub mod end;
#[doc = "Description cluster\\[0\\]: Subregions of region 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subs](subs) module"]
pub type SUBS = crate::Reg<u32, _SUBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBS;
#[doc = "`read()` method returns [subs::R](subs::R) reader structure"]
impl crate::Readable for SUBS {}
#[doc = "`write(|w| ..)` method takes [subs::W](subs::W) writer structure"]
impl crate::Writable for SUBS {}
#[doc = "Description cluster\\[0\\]: Subregions of region 0"]
pub mod subs;
