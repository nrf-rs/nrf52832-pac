#[doc = "Result of last incoming frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx](rx) module"]
pub type RX = crate::Reg<u32, _RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX;
#[doc = "`read()` method returns [rx::R](rx::R) reader structure"]
impl crate::Readable for RX {}
#[doc = "`write(|w| ..)` method takes [rx::W](rx::W) writer structure"]
impl crate::Writable for RX {}
#[doc = "Result of last incoming frames"]
pub mod rx;
