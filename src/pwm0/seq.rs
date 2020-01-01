#[doc = "Description cluster\\[0\\]: Beginning address in Data RAM of this sequence\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr](ptr) module"]
pub type PTR = crate::Reg<u32, _PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTR;
#[doc = "`read()` method returns [ptr::R](ptr::R) reader structure"]
impl crate::Readable for PTR {}
#[doc = "`write(|w| ..)` method takes [ptr::W](ptr::W) writer structure"]
impl crate::Writable for PTR {}
#[doc = "Description cluster\\[0\\]: Beginning address in Data RAM of this sequence"]
pub mod ptr;
#[doc = "Description cluster\\[0\\]: Amount of values (duty cycles) in this sequence\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Description cluster\\[0\\]: Amount of values (duty cycles) in this sequence"]
pub mod cnt;
#[doc = "Description cluster\\[0\\]: Amount of additional PWM periods between samples loaded into compare register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refresh](refresh) module"]
pub type REFRESH = crate::Reg<u32, _REFRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFRESH;
#[doc = "`read()` method returns [refresh::R](refresh::R) reader structure"]
impl crate::Readable for REFRESH {}
#[doc = "`write(|w| ..)` method takes [refresh::W](refresh::W) writer structure"]
impl crate::Writable for REFRESH {}
#[doc = "Description cluster\\[0\\]: Amount of additional PWM periods between samples loaded into compare register"]
pub mod refresh;
#[doc = "Description cluster\\[0\\]: Time added after the sequence\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enddelay](enddelay) module"]
pub type ENDDELAY = crate::Reg<u32, _ENDDELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDDELAY;
#[doc = "`read()` method returns [enddelay::R](enddelay::R) reader structure"]
impl crate::Readable for ENDDELAY {}
#[doc = "`write(|w| ..)` method takes [enddelay::W](enddelay::W) writer structure"]
impl crate::Writable for ENDDELAY {}
#[doc = "Description cluster\\[0\\]: Time added after the sequence"]
pub mod enddelay;
