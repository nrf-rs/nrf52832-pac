#[doc = "Description cluster\\[0\\]: Last results is equal or above CH\\[0\\].LIMIT.HIGH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limith](limith) module"]
pub type LIMITH = crate::Reg<u32, _LIMITH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIMITH;
#[doc = "`read()` method returns [limith::R](limith::R) reader structure"]
impl crate::Readable for LIMITH {}
#[doc = "`write(|w| ..)` method takes [limith::W](limith::W) writer structure"]
impl crate::Writable for LIMITH {}
#[doc = "Description cluster\\[0\\]: Last results is equal or above CH\\[0\\].LIMIT.HIGH"]
pub mod limith;
#[doc = "Description cluster\\[0\\]: Last results is equal or below CH\\[0\\].LIMIT.LOW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limitl](limitl) module"]
pub type LIMITL = crate::Reg<u32, _LIMITL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIMITL;
#[doc = "`read()` method returns [limitl::R](limitl::R) reader structure"]
impl crate::Readable for LIMITL {}
#[doc = "`write(|w| ..)` method takes [limitl::W](limitl::W) writer structure"]
impl crate::Writable for LIMITL {}
#[doc = "Description cluster\\[0\\]: Last results is equal or below CH\\[0\\].LIMIT.LOW"]
pub mod limitl;
