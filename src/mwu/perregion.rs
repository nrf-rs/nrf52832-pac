#[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [substatwa](substatwa) module"]
pub type SUBSTATWA = crate::Reg<u32, _SUBSTATWA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSTATWA;
#[doc = "`read()` method returns [substatwa::R](substatwa::R) reader structure"]
impl crate::Readable for SUBSTATWA {}
#[doc = "`write(|w| ..)` method takes [substatwa::W](substatwa::W) writer structure"]
impl crate::Writable for SUBSTATWA {}
#[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching"]
pub mod substatwa;
#[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [substatra](substatra) module"]
pub type SUBSTATRA = crate::Reg<u32, _SUBSTATRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSTATRA;
#[doc = "`read()` method returns [substatra::R](substatra::R) reader structure"]
impl crate::Readable for SUBSTATRA {}
#[doc = "`write(|w| ..)` method takes [substatra::W](substatra::W) writer structure"]
impl crate::Writable for SUBSTATRA {}
#[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching"]
pub mod substatra;
