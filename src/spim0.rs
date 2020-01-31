#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Start SPI transaction"]
    pub tasks_start: TASKS_START,
    #[doc = "0x14 - Stop SPI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 4usize],
    #[doc = "0x1c - Suspend SPI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume SPI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved4: [u8; 224usize],
    #[doc = "0x104 - SPI transaction has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved5: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - End of RXD buffer and TXD buffer reached"]
    pub events_end: EVENTS_END,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - End of TXD buffer reached"]
    pub events_endtx: EVENTS_ENDTX,
    _reserved8: [u8; 40usize],
    #[doc = "0x14c - Transaction started"]
    pub events_started: EVENTS_STARTED,
    _reserved9: [u8; 176usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved10: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved12: [u8; 500usize],
    #[doc = "0x500 - Enable SPIM"]
    pub enable: ENABLE,
    _reserved13: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved14: [u8; 16usize],
    #[doc = "0x524 - SPI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: FREQUENCY,
    _reserved15: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved18: [u8; 104usize],
    #[doc = "0x5c0 - Over-read character. Character clocked out in case and over-read of the TXD buffer."]
    pub orc: ORC,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MOSI signal"]
    pub mosi: self::psel::MOSI,
    #[doc = "0x08 - Pin select for MISO signal"]
    pub miso: self::psel::MISO,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::rxd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::rxd::LIST,
}
#[doc = r"Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::txd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::txd::LIST,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "Start SPI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "Stop SPI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "Suspend SPI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_suspend](tasks_suspend) module"]
pub type TASKS_SUSPEND = crate::Reg<u32, _TASKS_SUSPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SUSPEND;
#[doc = "`write(|w| ..)` method takes [tasks_suspend::W](tasks_suspend::W) writer structure"]
impl crate::Writable for TASKS_SUSPEND {}
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "Resume SPI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_resume](tasks_resume) module"]
pub type TASKS_RESUME = crate::Reg<u32, _TASKS_RESUME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RESUME;
#[doc = "`write(|w| ..)` method takes [tasks_resume::W](tasks_resume::W) writer structure"]
impl crate::Writable for TASKS_RESUME {}
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "SPI transaction has stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "End of RXD buffer reached\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endrx](events_endrx) module"]
pub type EVENTS_ENDRX = crate::Reg<u32, _EVENTS_ENDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDRX;
#[doc = "`read()` method returns [events_endrx::R](events_endrx::R) reader structure"]
impl crate::Readable for EVENTS_ENDRX {}
#[doc = "`write(|w| ..)` method takes [events_endrx::W](events_endrx::W) writer structure"]
impl crate::Writable for EVENTS_ENDRX {}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "End of RXD buffer and TXD buffer reached\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_end](events_end) module"]
pub type EVENTS_END = crate::Reg<u32, _EVENTS_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_END;
#[doc = "`read()` method returns [events_end::R](events_end::R) reader structure"]
impl crate::Readable for EVENTS_END {}
#[doc = "`write(|w| ..)` method takes [events_end::W](events_end::W) writer structure"]
impl crate::Writable for EVENTS_END {}
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "End of TXD buffer reached\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endtx](events_endtx) module"]
pub type EVENTS_ENDTX = crate::Reg<u32, _EVENTS_ENDTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDTX;
#[doc = "`read()` method returns [events_endtx::R](events_endtx::R) reader structure"]
impl crate::Readable for EVENTS_ENDTX {}
#[doc = "`write(|w| ..)` method takes [events_endtx::W](events_endtx::W) writer structure"]
impl crate::Writable for EVENTS_ENDTX {}
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "Transaction started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_started](events_started) module"]
pub type EVENTS_STARTED = crate::Reg<u32, _EVENTS_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STARTED;
#[doc = "`read()` method returns [events_started::R](events_started::R) reader structure"]
impl crate::Readable for EVENTS_STARTED {}
#[doc = "`write(|w| ..)` method takes [events_started::W](events_started::W) writer structure"]
impl crate::Writable for EVENTS_STARTED {}
#[doc = "Transaction started"]
pub mod events_started;
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
#[doc = "Enable SPIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](frequency) module"]
pub type FREQUENCY = crate::Reg<u32, _FREQUENCY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQUENCY;
#[doc = "`read()` method returns [frequency::R](frequency::R) reader structure"]
impl crate::Readable for FREQUENCY {}
#[doc = "`write(|w| ..)` method takes [frequency::W](frequency::W) writer structure"]
impl crate::Writable for FREQUENCY {}
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
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
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orc](orc) module"]
pub type ORC = crate::Reg<u32, _ORC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORC;
#[doc = "`read()` method returns [orc::R](orc::R) reader structure"]
impl crate::Readable for ORC {}
#[doc = "`write(|w| ..)` method takes [orc::W](orc::W) writer structure"]
impl crate::Writable for ORC {}
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub mod orc;
