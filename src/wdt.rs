#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the watchdog"]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - Watchdog timeout"]
    pub events_timeout: EVENTS_TIMEOUT,
    _reserved2: [u8; 512usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 244usize],
    #[doc = "0x400 - Run status"]
    pub runstatus: RUNSTATUS,
    #[doc = "0x404 - Request status"]
    pub reqstatus: REQSTATUS,
    _reserved6: [u8; 252usize],
    #[doc = "0x504 - Counter reload value"]
    pub crv: CRV,
    #[doc = "0x508 - Enable register for reload request registers"]
    pub rren: RREN,
    #[doc = "0x50c - Configuration register"]
    pub config: CONFIG,
    _reserved9: [u8; 240usize],
    #[doc = "0x600 - Description collection\\[0\\]: Reload request 0"]
    pub rr: [RR; 8],
}
#[doc = "Start the watchdog\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start the watchdog"]
pub mod tasks_start;
#[doc = "Watchdog timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_timeout](events_timeout) module"]
pub type EVENTS_TIMEOUT = crate::Reg<u32, _EVENTS_TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TIMEOUT;
#[doc = "`read()` method returns [events_timeout::R](events_timeout::R) reader structure"]
impl crate::Readable for EVENTS_TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [events_timeout::W](events_timeout::W) writer structure"]
impl crate::Writable for EVENTS_TIMEOUT {}
#[doc = "Watchdog timeout"]
pub mod events_timeout;
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
#[doc = "Run status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [runstatus](runstatus) module"]
pub type RUNSTATUS = crate::Reg<u32, _RUNSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RUNSTATUS;
#[doc = "`read()` method returns [runstatus::R](runstatus::R) reader structure"]
impl crate::Readable for RUNSTATUS {}
#[doc = "Run status"]
pub mod runstatus;
#[doc = "Request status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqstatus](reqstatus) module"]
pub type REQSTATUS = crate::Reg<u32, _REQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQSTATUS;
#[doc = "`read()` method returns [reqstatus::R](reqstatus::R) reader structure"]
impl crate::Readable for REQSTATUS {}
#[doc = "Request status"]
pub mod reqstatus;
#[doc = "Counter reload value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crv](crv) module"]
pub type CRV = crate::Reg<u32, _CRV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRV;
#[doc = "`read()` method returns [crv::R](crv::R) reader structure"]
impl crate::Readable for CRV {}
#[doc = "`write(|w| ..)` method takes [crv::W](crv::W) writer structure"]
impl crate::Writable for CRV {}
#[doc = "Counter reload value"]
pub mod crv;
#[doc = "Enable register for reload request registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rren](rren) module"]
pub type RREN = crate::Reg<u32, _RREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RREN;
#[doc = "`read()` method returns [rren::R](rren::R) reader structure"]
impl crate::Readable for RREN {}
#[doc = "`write(|w| ..)` method takes [rren::W](rren::W) writer structure"]
impl crate::Writable for RREN {}
#[doc = "Enable register for reload request registers"]
pub mod rren;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Description collection\\[0\\]: Reload request 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rr](rr) module"]
pub type RR = crate::Reg<u32, _RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RR;
#[doc = "`write(|w| ..)` method takes [rr::W](rr::W) writer structure"]
impl crate::Writable for RR {}
#[doc = "Description collection\\[0\\]: Reload request 0"]
pub mod rr;
