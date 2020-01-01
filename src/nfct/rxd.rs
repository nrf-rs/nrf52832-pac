#[doc = "Configuration of incoming frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameconfig](frameconfig) module"]
pub type FRAMECONFIG = crate::Reg<u32, _FRAMECONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMECONFIG;
#[doc = "`read()` method returns [frameconfig::R](frameconfig::R) reader structure"]
impl crate::Readable for FRAMECONFIG {}
#[doc = "`write(|w| ..)` method takes [frameconfig::W](frameconfig::W) writer structure"]
impl crate::Writable for FRAMECONFIG {}
#[doc = "Configuration of incoming frames"]
pub mod frameconfig;
#[doc = "Size of last incoming frame\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amount](amount) module"]
pub type AMOUNT = crate::Reg<u32, _AMOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMOUNT;
#[doc = "`read()` method returns [amount::R](amount::R) reader structure"]
impl crate::Readable for AMOUNT {}
#[doc = "Size of last incoming frame"]
pub mod amount;
