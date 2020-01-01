#[doc = "Pin select for LED signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [led](led) module"]
pub type LED = crate::Reg<u32, _LED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LED;
#[doc = "`read()` method returns [led::R](led::R) reader structure"]
impl crate::Readable for LED {}
#[doc = "`write(|w| ..)` method takes [led::W](led::W) writer structure"]
impl crate::Writable for LED {}
#[doc = "Pin select for LED signal"]
pub mod led;
#[doc = "Pin select for A signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a](a) module"]
pub type A = crate::Reg<u32, _A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A;
#[doc = "`read()` method returns [a::R](a::R) reader structure"]
impl crate::Readable for A {}
#[doc = "`write(|w| ..)` method takes [a::W](a::W) writer structure"]
impl crate::Writable for A {}
#[doc = "Pin select for A signal"]
pub mod a;
#[doc = "Pin select for B signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b](b) module"]
pub type B = crate::Reg<u32, _B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B;
#[doc = "`read()` method returns [b::R](b::R) reader structure"]
impl crate::Readable for B {}
#[doc = "`write(|w| ..)` method takes [b::W](b::W) writer structure"]
impl crate::Writable for B {}
#[doc = "Pin select for B signal"]
pub mod b;
