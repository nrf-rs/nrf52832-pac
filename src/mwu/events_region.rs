#[doc = "Description cluster\\[0\\]: Write access to region 0 detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wa](wa) module"]
pub type WA = crate::Reg<u32, _WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WA;
#[doc = "`read()` method returns [wa::R](wa::R) reader structure"]
impl crate::Readable for WA {}
#[doc = "`write(|w| ..)` method takes [wa::W](wa::W) writer structure"]
impl crate::Writable for WA {}
#[doc = "Description cluster\\[0\\]: Write access to region 0 detected"]
pub mod wa;
#[doc = "Description cluster\\[0\\]: Read access to region 0 detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra](ra) module"]
pub type RA = crate::Reg<u32, _RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RA;
#[doc = "`read()` method returns [ra::R](ra::R) reader structure"]
impl crate::Readable for RA {}
#[doc = "`write(|w| ..)` method takes [ra::W](ra::W) writer structure"]
impl crate::Writable for RA {}
#[doc = "Description cluster\\[0\\]: Read access to region 0 detected"]
pub mod ra;
