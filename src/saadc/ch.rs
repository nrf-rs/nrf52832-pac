#[doc = "Description cluster\\[0\\]: Input positive pin selection for CH\\[0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselp](pselp) module"]
pub type PSELP = crate::Reg<u32, _PSELP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELP;
#[doc = "`read()` method returns [pselp::R](pselp::R) reader structure"]
impl crate::Readable for PSELP {}
#[doc = "`write(|w| ..)` method takes [pselp::W](pselp::W) writer structure"]
impl crate::Writable for PSELP {}
#[doc = "Description cluster\\[0\\]: Input positive pin selection for CH\\[0\\]"]
pub mod pselp;
#[doc = "Description cluster\\[0\\]: Input negative pin selection for CH\\[0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pseln](pseln) module"]
pub type PSELN = crate::Reg<u32, _PSELN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELN;
#[doc = "`read()` method returns [pseln::R](pseln::R) reader structure"]
impl crate::Readable for PSELN {}
#[doc = "`write(|w| ..)` method takes [pseln::W](pseln::W) writer structure"]
impl crate::Writable for PSELN {}
#[doc = "Description cluster\\[0\\]: Input negative pin selection for CH\\[0\\]"]
pub mod pseln;
#[doc = "Description cluster\\[0\\]: Input configuration for CH\\[0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Description cluster\\[0\\]: Input configuration for CH\\[0\\]"]
pub mod config;
#[doc = "Description cluster\\[0\\]: High/low limits for event monitoring a channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit](limit) module"]
pub type LIMIT = crate::Reg<u32, _LIMIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIMIT;
#[doc = "`read()` method returns [limit::R](limit::R) reader structure"]
impl crate::Readable for LIMIT {}
#[doc = "`write(|w| ..)` method takes [limit::W](limit::W) writer structure"]
impl crate::Writable for LIMIT {}
#[doc = "Description cluster\\[0\\]: High/low limits for event monitoring a channel"]
pub mod limit;
