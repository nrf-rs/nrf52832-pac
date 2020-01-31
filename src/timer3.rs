#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start Timer"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop Timer"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Increment Timer (Counter mode only)"]
    pub tasks_count: TASKS_COUNT,
    #[doc = "0x0c - Clear time"]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x10 - Deprecated register - Shut down timer"]
    pub tasks_shutdown: TASKS_SHUTDOWN,
    _reserved5: [u8; 44usize],
    #[doc = "0x40 - Description collection\\[0\\]: Capture Timer value to CC\\[0\\]
register"]
    pub tasks_capture: [TASKS_CAPTURE; 6],
    _reserved6: [u8; 232usize],
    #[doc = "0x140 - Description collection\\[0\\]: Compare event on CC\\[0\\]
match"]
    pub events_compare: [EVENTS_COMPARE; 6],
    _reserved7: [u8; 168usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved8: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 504usize],
    #[doc = "0x504 - Timer mode selection"]
    pub mode: MODE,
    #[doc = "0x508 - Configure the number of bits used by the TIMER"]
    pub bitmode: BITMODE,
    _reserved12: [u8; 4usize],
    #[doc = "0x510 - Timer prescaler register"]
    pub prescaler: PRESCALER,
    _reserved13: [u8; 44usize],
    #[doc = "0x540 - Description collection\\[0\\]: Capture/Compare register 0"]
    pub cc: [CC; 6],
}
#[doc = "Start Timer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start Timer"]
pub mod tasks_start;
#[doc = "Stop Timer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop Timer"]
pub mod tasks_stop;
#[doc = "Increment Timer (Counter mode only)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_count](tasks_count) module"]
pub type TASKS_COUNT = crate::Reg<u32, _TASKS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_COUNT;
#[doc = "`write(|w| ..)` method takes [tasks_count::W](tasks_count::W) writer structure"]
impl crate::Writable for TASKS_COUNT {}
#[doc = "Increment Timer (Counter mode only)"]
pub mod tasks_count;
#[doc = "Clear time\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_clear](tasks_clear) module"]
pub type TASKS_CLEAR = crate::Reg<u32, _TASKS_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CLEAR;
#[doc = "`write(|w| ..)` method takes [tasks_clear::W](tasks_clear::W) writer structure"]
impl crate::Writable for TASKS_CLEAR {}
#[doc = "Clear time"]
pub mod tasks_clear;
#[doc = "Deprecated register - Shut down timer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_shutdown](tasks_shutdown) module"]
pub type TASKS_SHUTDOWN = crate::Reg<u32, _TASKS_SHUTDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SHUTDOWN;
#[doc = "`write(|w| ..)` method takes [tasks_shutdown::W](tasks_shutdown::W) writer structure"]
impl crate::Writable for TASKS_SHUTDOWN {}
#[doc = "Deprecated register - Shut down timer"]
pub mod tasks_shutdown;
#[doc = "Description collection\\[0\\]: Capture Timer value to CC\\[0\\]
register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_capture](tasks_capture) module"]
pub type TASKS_CAPTURE = crate::Reg<u32, _TASKS_CAPTURE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CAPTURE;
#[doc = "`write(|w| ..)` method takes [tasks_capture::W](tasks_capture::W) writer structure"]
impl crate::Writable for TASKS_CAPTURE {}
#[doc = "Description collection\\[0\\]: Capture Timer value to CC\\[0\\]
register"]
pub mod tasks_capture;
#[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\]
match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_compare](events_compare) module"]
pub type EVENTS_COMPARE = crate::Reg<u32, _EVENTS_COMPARE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_COMPARE;
#[doc = "`read()` method returns [events_compare::R](events_compare::R) reader structure"]
impl crate::Readable for EVENTS_COMPARE {}
#[doc = "`write(|w| ..)` method takes [events_compare::W](events_compare::W) writer structure"]
impl crate::Writable for EVENTS_COMPARE {}
#[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\]
match"]
pub mod events_compare;
#[doc = "Shortcut register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcut register"]
pub mod shorts;
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
#[doc = "Timer mode selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Timer mode selection"]
pub mod mode;
#[doc = "Configure the number of bits used by the TIMER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bitmode](bitmode) module"]
pub type BITMODE = crate::Reg<u32, _BITMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITMODE;
#[doc = "`read()` method returns [bitmode::R](bitmode::R) reader structure"]
impl crate::Readable for BITMODE {}
#[doc = "`write(|w| ..)` method takes [bitmode::W](bitmode::W) writer structure"]
impl crate::Writable for BITMODE {}
#[doc = "Configure the number of bits used by the TIMER"]
pub mod bitmode;
#[doc = "Timer prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescaler](prescaler) module"]
pub type PRESCALER = crate::Reg<u32, _PRESCALER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESCALER;
#[doc = "`read()` method returns [prescaler::R](prescaler::R) reader structure"]
impl crate::Readable for PRESCALER {}
#[doc = "`write(|w| ..)` method takes [prescaler::W](prescaler::W) writer structure"]
impl crate::Writable for PRESCALER {}
#[doc = "Timer prescaler register"]
pub mod prescaler;
#[doc = "Description collection\\[0\\]: Capture/Compare register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Description collection\\[0\\]: Capture/Compare register 0"]
pub mod cc;
