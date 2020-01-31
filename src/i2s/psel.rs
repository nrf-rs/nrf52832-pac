#[doc = "Pin select for MCK signal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mck](mck) module"]
pub type MCK = crate::Reg<u32, _MCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCK;
#[doc = "`read()` method returns [mck::R](mck::R) reader structure"]
impl crate::Readable for MCK {}
#[doc = "`write(|w| ..)` method takes [mck::W](mck::W) writer structure"]
impl crate::Writable for MCK {}
#[doc = "Pin select for MCK signal."]
pub mod mck;
#[doc = "Pin select for SCK signal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sck](sck) module"]
pub type SCK = crate::Reg<u32, _SCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCK;
#[doc = "`read()` method returns [sck::R](sck::R) reader structure"]
impl crate::Readable for SCK {}
#[doc = "`write(|w| ..)` method takes [sck::W](sck::W) writer structure"]
impl crate::Writable for SCK {}
#[doc = "Pin select for SCK signal."]
pub mod sck;
#[doc = "Pin select for LRCK signal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lrck](lrck) module"]
pub type LRCK = crate::Reg<u32, _LRCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LRCK;
#[doc = "`read()` method returns [lrck::R](lrck::R) reader structure"]
impl crate::Readable for LRCK {}
#[doc = "`write(|w| ..)` method takes [lrck::W](lrck::W) writer structure"]
impl crate::Writable for LRCK {}
#[doc = "Pin select for LRCK signal."]
pub mod lrck;
#[doc = "Pin select for SDIN signal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdin](sdin) module"]
pub type SDIN = crate::Reg<u32, _SDIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIN;
#[doc = "`read()` method returns [sdin::R](sdin::R) reader structure"]
impl crate::Readable for SDIN {}
#[doc = "`write(|w| ..)` method takes [sdin::W](sdin::W) writer structure"]
impl crate::Writable for SDIN {}
#[doc = "Pin select for SDIN signal."]
pub mod sdin;
#[doc = "Pin select for SDOUT signal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdout](sdout) module"]
pub type SDOUT = crate::Reg<u32, _SDOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDOUT;
#[doc = "`read()` method returns [sdout::R](sdout::R) reader structure"]
impl crate::Readable for SDOUT {}
#[doc = "`write(|w| ..)` method takes [sdout::W](sdout::W) writer structure"]
impl crate::Writable for SDOUT {}
#[doc = "Pin select for SDOUT signal."]
pub mod sdout;
