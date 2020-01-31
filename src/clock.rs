#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK crystal oscillator"]
    pub tasks_hfclkstart: TASKS_HFCLKSTART,
    #[doc = "0x04 - Stop HFCLK crystal oscillator"]
    pub tasks_hfclkstop: TASKS_HFCLKSTOP,
    #[doc = "0x08 - Start LFCLK source"]
    pub tasks_lfclkstart: TASKS_LFCLKSTART,
    #[doc = "0x0c - Stop LFCLK source"]
    pub tasks_lfclkstop: TASKS_LFCLKSTOP,
    #[doc = "0x10 - Start calibration of LFRC oscillator"]
    pub tasks_cal: TASKS_CAL,
    #[doc = "0x14 - Start calibration timer"]
    pub tasks_ctstart: TASKS_CTSTART,
    #[doc = "0x18 - Stop calibration timer"]
    pub tasks_ctstop: TASKS_CTSTOP,
    _reserved7: [u8; 228usize],
    #[doc = "0x100 - HFCLK oscillator started"]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK started"]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved9: [u8; 4usize],
    #[doc = "0x10c - Calibration of LFCLK RC oscillator complete event"]
    pub events_done: EVENTS_DONE,
    #[doc = "0x110 - Calibration timer timeout"]
    pub events_ctto: EVENTS_CTTO,
    _reserved11: [u8; 496usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved13: [u8; 252usize],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: HFCLKRUN,
    #[doc = "0x40c - HFCLK status"]
    pub hfclkstat: HFCLKSTAT,
    _reserved15: [u8; 4usize],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: LFCLKRUN,
    #[doc = "0x418 - LFCLK status"]
    pub lfclkstat: LFCLKSTAT,
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    pub lfclksrccopy: LFCLKSRCCOPY,
    _reserved18: [u8; 248usize],
    #[doc = "0x518 - Clock source for the LFCLK"]
    pub lfclksrc: LFCLKSRC,
    _reserved19: [u8; 28usize],
    #[doc = "0x538 - Calibration timer interval"]
    pub ctiv: CTIV,
    _reserved20: [u8; 32usize],
    #[doc = "0x55c - Clocking options for the Trace Port debug interface"]
    pub traceconfig: TRACECONFIG,
}
#[doc = "Start HFCLK crystal oscillator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_hfclkstart](tasks_hfclkstart) module"]
pub type TASKS_HFCLKSTART = crate::Reg<u32, _TASKS_HFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLKSTART;
#[doc = "`write(|w| ..)` method takes [tasks_hfclkstart::W](tasks_hfclkstart::W) writer structure"]
impl crate::Writable for TASKS_HFCLKSTART {}
#[doc = "Start HFCLK crystal oscillator"]
pub mod tasks_hfclkstart;
#[doc = "Stop HFCLK crystal oscillator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_hfclkstop](tasks_hfclkstop) module"]
pub type TASKS_HFCLKSTOP = crate::Reg<u32, _TASKS_HFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLKSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_hfclkstop::W](tasks_hfclkstop::W) writer structure"]
impl crate::Writable for TASKS_HFCLKSTOP {}
#[doc = "Stop HFCLK crystal oscillator"]
pub mod tasks_hfclkstop;
#[doc = "Start LFCLK source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_lfclkstart](tasks_lfclkstart) module"]
pub type TASKS_LFCLKSTART = crate::Reg<u32, _TASKS_LFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LFCLKSTART;
#[doc = "`write(|w| ..)` method takes [tasks_lfclkstart::W](tasks_lfclkstart::W) writer structure"]
impl crate::Writable for TASKS_LFCLKSTART {}
#[doc = "Start LFCLK source"]
pub mod tasks_lfclkstart;
#[doc = "Stop LFCLK source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_lfclkstop](tasks_lfclkstop) module"]
pub type TASKS_LFCLKSTOP = crate::Reg<u32, _TASKS_LFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LFCLKSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_lfclkstop::W](tasks_lfclkstop::W) writer structure"]
impl crate::Writable for TASKS_LFCLKSTOP {}
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "Start calibration of LFRC oscillator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_cal](tasks_cal) module"]
pub type TASKS_CAL = crate::Reg<u32, _TASKS_CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CAL;
#[doc = "`write(|w| ..)` method takes [tasks_cal::W](tasks_cal::W) writer structure"]
impl crate::Writable for TASKS_CAL {}
#[doc = "Start calibration of LFRC oscillator"]
pub mod tasks_cal;
#[doc = "Start calibration timer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ctstart](tasks_ctstart) module"]
pub type TASKS_CTSTART = crate::Reg<u32, _TASKS_CTSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CTSTART;
#[doc = "`write(|w| ..)` method takes [tasks_ctstart::W](tasks_ctstart::W) writer structure"]
impl crate::Writable for TASKS_CTSTART {}
#[doc = "Start calibration timer"]
pub mod tasks_ctstart;
#[doc = "Stop calibration timer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ctstop](tasks_ctstop) module"]
pub type TASKS_CTSTOP = crate::Reg<u32, _TASKS_CTSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CTSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_ctstop::W](tasks_ctstop::W) writer structure"]
impl crate::Writable for TASKS_CTSTOP {}
#[doc = "Stop calibration timer"]
pub mod tasks_ctstop;
#[doc = "HFCLK oscillator started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_hfclkstarted](events_hfclkstarted) module"]
pub type EVENTS_HFCLKSTARTED = crate::Reg<u32, _EVENTS_HFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_HFCLKSTARTED;
#[doc = "`read()` method returns [events_hfclkstarted::R](events_hfclkstarted::R) reader structure"]
impl crate::Readable for EVENTS_HFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_hfclkstarted::W](events_hfclkstarted::W) writer structure"]
impl crate::Writable for EVENTS_HFCLKSTARTED {}
#[doc = "HFCLK oscillator started"]
pub mod events_hfclkstarted;
#[doc = "LFCLK started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_lfclkstarted](events_lfclkstarted) module"]
pub type EVENTS_LFCLKSTARTED = crate::Reg<u32, _EVENTS_LFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_LFCLKSTARTED;
#[doc = "`read()` method returns [events_lfclkstarted::R](events_lfclkstarted::R) reader structure"]
impl crate::Readable for EVENTS_LFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_lfclkstarted::W](events_lfclkstarted::W) writer structure"]
impl crate::Writable for EVENTS_LFCLKSTARTED {}
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "Calibration of LFCLK RC oscillator complete event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_done](events_done) module"]
pub type EVENTS_DONE = crate::Reg<u32, _EVENTS_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DONE;
#[doc = "`read()` method returns [events_done::R](events_done::R) reader structure"]
impl crate::Readable for EVENTS_DONE {}
#[doc = "`write(|w| ..)` method takes [events_done::W](events_done::W) writer structure"]
impl crate::Writable for EVENTS_DONE {}
#[doc = "Calibration of LFCLK RC oscillator complete event"]
pub mod events_done;
#[doc = "Calibration timer timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ctto](events_ctto) module"]
pub type EVENTS_CTTO = crate::Reg<u32, _EVENTS_CTTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CTTO;
#[doc = "`read()` method returns [events_ctto::R](events_ctto::R) reader structure"]
impl crate::Readable for EVENTS_CTTO {}
#[doc = "`write(|w| ..)` method takes [events_ctto::W](events_ctto::W) writer structure"]
impl crate::Writable for EVENTS_CTTO {}
#[doc = "Calibration timer timeout"]
pub mod events_ctto;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Status indicating that HFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkrun](hfclkrun) module"]
pub type HFCLKRUN = crate::Reg<u32, _HFCLKRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKRUN;
#[doc = "`read()` method returns [hfclkrun::R](hfclkrun::R) reader structure"]
impl crate::Readable for HFCLKRUN {}
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLK status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkstat](hfclkstat) module"]
pub type HFCLKSTAT = crate::Reg<u32, _HFCLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKSTAT;
#[doc = "`read()` method returns [hfclkstat::R](hfclkstat::R) reader structure"]
impl crate::Readable for HFCLKSTAT {}
#[doc = "HFCLK status"]
pub mod hfclkstat;
#[doc = "Status indicating that LFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclkrun](lfclkrun) module"]
pub type LFCLKRUN = crate::Reg<u32, _LFCLKRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKRUN;
#[doc = "`read()` method returns [lfclkrun::R](lfclkrun::R) reader structure"]
impl crate::Readable for LFCLKRUN {}
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLK status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclkstat](lfclkstat) module"]
pub type LFCLKSTAT = crate::Reg<u32, _LFCLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSTAT;
#[doc = "`read()` method returns [lfclkstat::R](lfclkstat::R) reader structure"]
impl crate::Readable for LFCLKSTAT {}
#[doc = "LFCLK status"]
pub mod lfclkstat;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksrccopy](lfclksrccopy) module"]
pub type LFCLKSRCCOPY = crate::Reg<u32, _LFCLKSRCCOPY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSRCCOPY;
#[doc = "`read()` method returns [lfclksrccopy::R](lfclksrccopy::R) reader structure"]
impl crate::Readable for LFCLKSRCCOPY {}
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "Clock source for the LFCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksrc](lfclksrc) module"]
pub type LFCLKSRC = crate::Reg<u32, _LFCLKSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSRC;
#[doc = "`read()` method returns [lfclksrc::R](lfclksrc::R) reader structure"]
impl crate::Readable for LFCLKSRC {}
#[doc = "`write(|w| ..)` method takes [lfclksrc::W](lfclksrc::W) writer structure"]
impl crate::Writable for LFCLKSRC {}
#[doc = "Clock source for the LFCLK"]
pub mod lfclksrc;
#[doc = "Calibration timer interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiv](ctiv) module"]
pub type CTIV = crate::Reg<u32, _CTIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIV;
#[doc = "`read()` method returns [ctiv::R](ctiv::R) reader structure"]
impl crate::Readable for CTIV {}
#[doc = "`write(|w| ..)` method takes [ctiv::W](ctiv::W) writer structure"]
impl crate::Writable for CTIV {}
#[doc = "Calibration timer interval"]
pub mod ctiv;
#[doc = "Clocking options for the Trace Port debug interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceconfig](traceconfig) module"]
pub type TRACECONFIG = crate::Reg<u32, _TRACECONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACECONFIG;
#[doc = "`read()` method returns [traceconfig::R](traceconfig::R) reader structure"]
impl crate::Readable for TRACECONFIG {}
#[doc = "`write(|w| ..)` method takes [traceconfig::W](traceconfig::W) writer structure"]
impl crate::Writable for TRACECONFIG {}
#[doc = "Clocking options for the Trace Port debug interface"]
pub mod traceconfig;
