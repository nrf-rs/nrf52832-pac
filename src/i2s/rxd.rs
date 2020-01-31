#[doc = "Receive buffer RAM start address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr](ptr) module"]
pub type PTR = crate::Reg<u32, _PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTR;
#[doc = "`read()` method returns [ptr::R](ptr::R) reader structure"]
impl crate::Readable for PTR {}
#[doc = "`write(|w| ..)` method takes [ptr::W](ptr::W) writer structure"]
impl crate::Writable for PTR {}
#[doc = "Receive buffer RAM start address."]
pub mod ptr;
