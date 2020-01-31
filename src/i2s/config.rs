#[doc = "I2S mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "I2S mode."]
pub mod mode;
#[doc = "Reception (RX) enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxen](rxen) module"]
pub type RXEN = crate::Reg<u32, _RXEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXEN;
#[doc = "`read()` method returns [rxen::R](rxen::R) reader structure"]
impl crate::Readable for RXEN {}
#[doc = "`write(|w| ..)` method takes [rxen::W](rxen::W) writer structure"]
impl crate::Writable for RXEN {}
#[doc = "Reception (RX) enable."]
pub mod rxen;
#[doc = "Transmission (TX) enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txen](txen) module"]
pub type TXEN = crate::Reg<u32, _TXEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEN;
#[doc = "`read()` method returns [txen::R](txen::R) reader structure"]
impl crate::Readable for TXEN {}
#[doc = "`write(|w| ..)` method takes [txen::W](txen::W) writer structure"]
impl crate::Writable for TXEN {}
#[doc = "Transmission (TX) enable."]
pub mod txen;
#[doc = "Master clock generator enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcken](mcken) module"]
pub type MCKEN = crate::Reg<u32, _MCKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCKEN;
#[doc = "`read()` method returns [mcken::R](mcken::R) reader structure"]
impl crate::Readable for MCKEN {}
#[doc = "`write(|w| ..)` method takes [mcken::W](mcken::W) writer structure"]
impl crate::Writable for MCKEN {}
#[doc = "Master clock generator enable."]
pub mod mcken;
#[doc = "Master clock generator frequency.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mckfreq](mckfreq) module"]
pub type MCKFREQ = crate::Reg<u32, _MCKFREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCKFREQ;
#[doc = "`read()` method returns [mckfreq::R](mckfreq::R) reader structure"]
impl crate::Readable for MCKFREQ {}
#[doc = "`write(|w| ..)` method takes [mckfreq::W](mckfreq::W) writer structure"]
impl crate::Writable for MCKFREQ {}
#[doc = "Master clock generator frequency."]
pub mod mckfreq;
#[doc = "MCK / LRCK ratio.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratio](ratio) module"]
pub type RATIO = crate::Reg<u32, _RATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATIO;
#[doc = "`read()` method returns [ratio::R](ratio::R) reader structure"]
impl crate::Readable for RATIO {}
#[doc = "`write(|w| ..)` method takes [ratio::W](ratio::W) writer structure"]
impl crate::Writable for RATIO {}
#[doc = "MCK / LRCK ratio."]
pub mod ratio;
#[doc = "Sample width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swidth](swidth) module"]
pub type SWIDTH = crate::Reg<u32, _SWIDTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIDTH;
#[doc = "`read()` method returns [swidth::R](swidth::R) reader structure"]
impl crate::Readable for SWIDTH {}
#[doc = "`write(|w| ..)` method takes [swidth::W](swidth::W) writer structure"]
impl crate::Writable for SWIDTH {}
#[doc = "Sample width."]
pub mod swidth;
#[doc = "Alignment of sample within a frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [align](align) module"]
pub type ALIGN = crate::Reg<u32, _ALIGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALIGN;
#[doc = "`read()` method returns [align::R](align::R) reader structure"]
impl crate::Readable for ALIGN {}
#[doc = "`write(|w| ..)` method takes [align::W](align::W) writer structure"]
impl crate::Writable for ALIGN {}
#[doc = "Alignment of sample within a frame."]
pub mod align;
#[doc = "Frame format.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [format](format) module"]
pub type FORMAT = crate::Reg<u32, _FORMAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORMAT;
#[doc = "`read()` method returns [format::R](format::R) reader structure"]
impl crate::Readable for FORMAT {}
#[doc = "`write(|w| ..)` method takes [format::W](format::W) writer structure"]
impl crate::Writable for FORMAT {}
#[doc = "Frame format."]
pub mod format;
#[doc = "Enable channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channels](channels) module"]
pub type CHANNELS = crate::Reg<u32, _CHANNELS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNELS;
#[doc = "`read()` method returns [channels::R](channels::R) reader structure"]
impl crate::Readable for CHANNELS {}
#[doc = "`write(|w| ..)` method takes [channels::W](channels::W) writer structure"]
impl crate::Writable for CHANNELS {}
#[doc = "Enable channels."]
pub mod channels;
