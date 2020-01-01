#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 36usize],
    #[doc = "0x24 - Acquire SPI semaphore"]
    pub tasks_acquire: TASKS_ACQUIRE,
    #[doc = "0x28 - Release SPI semaphore, enabling the SPI slave to acquire it"]
    pub tasks_release: TASKS_RELEASE,
    _reserved2: [u8; 216usize],
    #[doc = "0x104 - Granted transaction completed"]
    pub events_end: EVENTS_END,
    _reserved3: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved4: [u8; 20usize],
    #[doc = "0x128 - Semaphore acquired"]
    pub events_acquired: EVENTS_ACQUIRED,
    _reserved5: [u8; 212usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved6: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 244usize],
    #[doc = "0x400 - Semaphore status register"]
    pub semstat: SEMSTAT,
    _reserved9: [u8; 60usize],
    #[doc = "0x440 - Status from last transaction"]
    pub status: STATUS,
    _reserved10: [u8; 188usize],
    #[doc = "0x500 - Enable SPI slave"]
    pub enable: ENABLE,
    _reserved11: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved12: [u8; 28usize],
    #[doc = "0x534 - Unspecified"]
    pub rxd: RXD,
    _reserved13: [u8; 4usize],
    #[doc = "0x544 - Unspecified"]
    pub txd: TXD,
    _reserved14: [u8; 4usize],
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved15: [u8; 4usize],
    #[doc = "0x55c - Default character. Character clocked out in case of an ignored transaction."]
    pub def: DEF,
    _reserved16: [u8; 96usize],
    #[doc = "0x5c0 - Over-read character"]
    pub orc: ORC,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MISO signal"]
    pub miso: self::psel::MISO,
    #[doc = "0x08 - Pin select for MOSI signal"]
    pub mosi: self::psel::MOSI,
    #[doc = "0x0c - Pin select for CSN signal"]
    pub csn: self::psel::CSN,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - RXD data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes received in last granted transaction"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - TXD data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transmitted in last granted transaction"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "Acquire SPI semaphore\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_acquire](tasks_acquire) module"]
pub type TASKS_ACQUIRE = crate::Reg<u32, _TASKS_ACQUIRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_ACQUIRE;
#[doc = "`write(|w| ..)` method takes [tasks_acquire::W](tasks_acquire::W) writer structure"]
impl crate::Writable for TASKS_ACQUIRE {}
#[doc = "Acquire SPI semaphore"]
pub mod tasks_acquire;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_release](tasks_release) module"]
pub type TASKS_RELEASE = crate::Reg<u32, _TASKS_RELEASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RELEASE;
#[doc = "`write(|w| ..)` method takes [tasks_release::W](tasks_release::W) writer structure"]
impl crate::Writable for TASKS_RELEASE {}
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub mod tasks_release;
#[doc = "Granted transaction completed\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_end](events_end) module"]
pub type EVENTS_END = crate::Reg<u32, _EVENTS_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_END;
#[doc = "`read()` method returns [events_end::R](events_end::R) reader structure"]
impl crate::Readable for EVENTS_END {}
#[doc = "`write(|w| ..)` method takes [events_end::W](events_end::W) writer structure"]
impl crate::Writable for EVENTS_END {}
#[doc = "Granted transaction completed"]
pub mod events_end;
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
#[doc = "Semaphore acquired\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_acquired](events_acquired) module"]
pub type EVENTS_ACQUIRED = crate::Reg<u32, _EVENTS_ACQUIRED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ACQUIRED;
#[doc = "`read()` method returns [events_acquired::R](events_acquired::R) reader structure"]
impl crate::Readable for EVENTS_ACQUIRED {}
#[doc = "`write(|w| ..)` method takes [events_acquired::W](events_acquired::W) writer structure"]
impl crate::Writable for EVENTS_ACQUIRED {}
#[doc = "Semaphore acquired"]
pub mod events_acquired;
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
#[doc = "Semaphore status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semstat](semstat) module"]
pub type SEMSTAT = crate::Reg<u32, _SEMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMSTAT;
#[doc = "`read()` method returns [semstat::R](semstat::R) reader structure"]
impl crate::Readable for SEMSTAT {}
#[doc = "Semaphore status register"]
pub mod semstat;
#[doc = "Status from last transaction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status from last transaction"]
pub mod status;
#[doc = "Enable SPI slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable SPI slave"]
pub mod enable;
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
#[doc = "Default character. Character clocked out in case of an ignored transaction.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [def](def) module"]
pub type DEF = crate::Reg<u32, _DEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEF;
#[doc = "`read()` method returns [def::R](def::R) reader structure"]
impl crate::Readable for DEF {}
#[doc = "`write(|w| ..)` method takes [def::W](def::W) writer structure"]
impl crate::Writable for DEF {}
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub mod def;
#[doc = "Over-read character\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orc](orc) module"]
pub type ORC = crate::Reg<u32, _ORC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORC;
#[doc = "`read()` method returns [orc::R](orc::R) reader structure"]
impl crate::Readable for ORC {}
#[doc = "`write(|w| ..)` method takes [orc::W](orc::W) writer structure"]
impl crate::Writable for ORC {}
#[doc = "Over-read character"]
pub mod orc;
