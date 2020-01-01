#[doc = "Pin select for SCL signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl](scl) module"]
pub type SCL = crate::Reg<u32, _SCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL;
#[doc = "`read()` method returns [scl::R](scl::R) reader structure"]
impl crate::Readable for SCL {}
#[doc = "`write(|w| ..)` method takes [scl::W](scl::W) writer structure"]
impl crate::Writable for SCL {}
#[doc = "Pin select for SCL signal"]
pub mod scl;
#[doc = "Pin select for SDA signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda](sda) module"]
pub type SDA = crate::Reg<u32, _SDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA;
#[doc = "`read()` method returns [sda::R](sda::R) reader structure"]
impl crate::Readable for SDA {}
#[doc = "`write(|w| ..)` method takes [sda::W](sda::W) writer structure"]
impl crate::Writable for SDA {}
#[doc = "Pin select for SDA signal"]
pub mod sda;
