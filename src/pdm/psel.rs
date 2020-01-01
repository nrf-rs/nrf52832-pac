#[doc = "Pin number configuration for PDM CLK signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](clk) module"]
pub type CLK = crate::Reg<u32, _CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK;
#[doc = "`read()` method returns [clk::R](clk::R) reader structure"]
impl crate::Readable for CLK {}
#[doc = "`write(|w| ..)` method takes [clk::W](clk::W) writer structure"]
impl crate::Writable for CLK {}
#[doc = "Pin number configuration for PDM CLK signal"]
pub mod clk;
#[doc = "Pin number configuration for PDM DIN signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din](din) module"]
pub type DIN = crate::Reg<u32, _DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN;
#[doc = "`read()` method returns [din::R](din::R) reader structure"]
impl crate::Readable for DIN {}
#[doc = "`write(|w| ..)` method takes [din::W](din::W) writer structure"]
impl crate::Writable for DIN {}
#[doc = "Pin number configuration for PDM DIN signal"]
pub mod din;
