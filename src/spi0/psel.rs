#[doc = "Pin select for SCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sck](sck) module"]
pub type SCK = crate::Reg<u32, _SCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCK;
#[doc = "`read()` method returns [sck::R](sck::R) reader structure"]
impl crate::Readable for SCK {}
#[doc = "`write(|w| ..)` method takes [sck::W](sck::W) writer structure"]
impl crate::Writable for SCK {}
#[doc = "Pin select for SCK"]
pub mod sck;
#[doc = "Pin select for MOSI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosi](mosi) module"]
pub type MOSI = crate::Reg<u32, _MOSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOSI;
#[doc = "`read()` method returns [mosi::R](mosi::R) reader structure"]
impl crate::Readable for MOSI {}
#[doc = "`write(|w| ..)` method takes [mosi::W](mosi::W) writer structure"]
impl crate::Writable for MOSI {}
#[doc = "Pin select for MOSI"]
pub mod mosi;
#[doc = "Pin select for MISO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miso](miso) module"]
pub type MISO = crate::Reg<u32, _MISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISO;
#[doc = "`read()` method returns [miso::R](miso::R) reader structure"]
impl crate::Readable for MISO {}
#[doc = "`write(|w| ..)` method takes [miso::W](miso::W) writer structure"]
impl crate::Writable for MISO {}
#[doc = "Pin select for MISO"]
pub mod miso;
