#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY."]
    pub tasks_out: [TASKS_OUT; 8],
    _reserved1: [u8; 16usize],
    #[doc = "0x30 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high."]
    pub tasks_set: [TASKS_SET; 8],
    _reserved2: [u8; 16usize],
    #[doc = "0x60 - Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low."]
    pub tasks_clr: [TASKS_CLR; 8],
    _reserved3: [u8; 128usize],
    #[doc = "0x100 - Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL"]
    pub events_in: [EVENTS_IN; 8],
    _reserved4: [u8; 92usize],
    #[doc = "0x17c - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    pub events_port: EVENTS_PORT,
    _reserved5: [u8; 388usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 516usize],
    #[doc = "0x510 - Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\]
and CLR\\[n\\]
tasks and IN\\[n\\]
event"]
    pub config: [CONFIG; 8],
}
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_out](tasks_out) module"]
pub type TASKS_OUT = crate::Reg<u32, _TASKS_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_OUT;
#[doc = "`write(|w| ..)` method takes [tasks_out::W](tasks_out::W) writer structure"]
impl crate::Writable for TASKS_OUT {}
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY."]
pub mod tasks_out;
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_set](tasks_set) module"]
pub type TASKS_SET = crate::Reg<u32, _TASKS_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SET;
#[doc = "`write(|w| ..)` method takes [tasks_set::W](tasks_set::W) writer structure"]
impl crate::Writable for TASKS_SET {}
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high."]
pub mod tasks_set;
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_clr](tasks_clr) module"]
pub type TASKS_CLR = crate::Reg<u32, _TASKS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CLR;
#[doc = "`write(|w| ..)` method takes [tasks_clr::W](tasks_clr::W) writer structure"]
impl crate::Writable for TASKS_CLR {}
#[doc = "Description collection\\[0\\]: Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low."]
pub mod tasks_clr;
#[doc = "Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_in](events_in) module"]
pub type EVENTS_IN = crate::Reg<u32, _EVENTS_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_IN;
#[doc = "`read()` method returns [events_in::R](events_in::R) reader structure"]
impl crate::Readable for EVENTS_IN {}
#[doc = "`write(|w| ..)` method takes [events_in::W](events_in::W) writer structure"]
impl crate::Writable for EVENTS_IN {}
#[doc = "Description collection\\[0\\]: Event generated from pin specified in CONFIG\\[0\\].PSEL"]
pub mod events_in;
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_port](events_port) module"]
pub type EVENTS_PORT = crate::Reg<u32, _EVENTS_PORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_PORT;
#[doc = "`read()` method returns [events_port::R](events_port::R) reader structure"]
impl crate::Readable for EVENTS_PORT {}
#[doc = "`write(|w| ..)` method takes [events_port::W](events_port::W) writer structure"]
impl crate::Writable for EVENTS_PORT {}
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub mod events_port;
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
#[doc = "Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\]
and CLR\\[n\\]
tasks and IN\\[n\\]
event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Description collection\\[0\\]: Configuration for OUT\\[n\\], SET\\[n\\]
and CLR\\[n\\]
tasks and IN\\[n\\]
event"]
pub mod config;
