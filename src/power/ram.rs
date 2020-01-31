#[doc = "Description cluster\\[0\\]: RAM0 power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "Description cluster\\[0\\]: RAM0 power control register"]
pub mod power;
#[doc = "Description cluster\\[0\\]: RAM0 power control set register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerset](powerset) module"]
pub type POWERSET = crate::Reg<u32, _POWERSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWERSET;
#[doc = "`write(|w| ..)` method takes [powerset::W](powerset::W) writer structure"]
impl crate::Writable for POWERSET {}
#[doc = "Description cluster\\[0\\]: RAM0 power control set register"]
pub mod powerset;
#[doc = "Description cluster\\[0\\]: RAM0 power control clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerclr](powerclr) module"]
pub type POWERCLR = crate::Reg<u32, _POWERCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWERCLR;
#[doc = "`write(|w| ..)` method takes [powerclr::W](powerclr::W) writer structure"]
impl crate::Writable for POWERCLR {}
#[doc = "Description cluster\\[0\\]: RAM0 power control clear register"]
pub mod powerclr;
