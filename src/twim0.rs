#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start TWI receive sequence"]
    pub tasks_startrx: TASKS_STARTRX,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Start TWI transmit sequence"]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - Stop TWI transaction. Must be issued while the TWI master is not suspended."]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved5: [u8; 224usize],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved6: [u8; 28usize],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved7: [u8; 32usize],
    #[doc = "0x148 - Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
    pub events_suspended: EVENTS_SUSPENDED,
    #[doc = "0x14c - Receive sequence started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - Transmit sequence started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved10: [u8; 8usize],
    #[doc = "0x15c - Byte boundary, starting to receive the last byte"]
    pub events_lastrx: EVENTS_LASTRX,
    #[doc = "0x160 - Byte boundary, starting to transmit the last byte"]
    pub events_lasttx: EVENTS_LASTTX,
    _reserved12: [u8; 156usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved13: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved16: [u8; 440usize],
    #[doc = "0x4c4 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved17: [u8; 56usize],
    #[doc = "0x500 - Enable TWIM"]
    pub enable: ENABLE,
    _reserved18: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved19: [u8; 20usize],
    #[doc = "0x524 - TWI frequency"]
    pub frequency: FREQUENCY,
    _reserved20: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved22: [u8; 52usize],
    #[doc = "0x588 - Address used in the TWI transfer"]
    pub address: ADDRESS,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCL signal"]
    pub scl: self::psel::SCL,
    #[doc = "0x04 - Pin select for SDA signal"]
    pub sda: self::psel::SDA,
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
#[doc = "Start TWI receive sequence\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startrx](tasks_startrx) module"]
pub type TASKS_STARTRX = crate::Reg<u32, _TASKS_STARTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTRX;
#[doc = "`write(|w| ..)` method takes [tasks_startrx::W](tasks_startrx::W) writer structure"]
impl crate::Writable for TASKS_STARTRX {}
#[doc = "Start TWI receive sequence"]
pub mod tasks_startrx;
#[doc = "Start TWI transmit sequence\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_starttx](tasks_starttx) module"]
pub type TASKS_STARTTX = crate::Reg<u32, _TASKS_STARTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTTX;
#[doc = "`write(|w| ..)` method takes [tasks_starttx::W](tasks_starttx::W) writer structure"]
impl crate::Writable for TASKS_STARTTX {}
#[doc = "Start TWI transmit sequence"]
pub mod tasks_starttx;
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
pub mod tasks_stop;
#[doc = "Suspend TWI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_suspend](tasks_suspend) module"]
pub type TASKS_SUSPEND = crate::Reg<u32, _TASKS_SUSPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SUSPEND;
#[doc = "`write(|w| ..)` method takes [tasks_suspend::W](tasks_suspend::W) writer structure"]
impl crate::Writable for TASKS_SUSPEND {}
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "Resume TWI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_resume](tasks_resume) module"]
pub type TASKS_RESUME = crate::Reg<u32, _TASKS_RESUME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RESUME;
#[doc = "`write(|w| ..)` method takes [tasks_resume::W](tasks_resume::W) writer structure"]
impl crate::Writable for TASKS_RESUME {}
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "TWI stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "TWI error\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "TWI error"]
pub mod events_error;
#[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_suspended](events_suspended) module"]
pub type EVENTS_SUSPENDED = crate::Reg<u32, _EVENTS_SUSPENDED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SUSPENDED;
#[doc = "`read()` method returns [events_suspended::R](events_suspended::R) reader structure"]
impl crate::Readable for EVENTS_SUSPENDED {}
#[doc = "`write(|w| ..)` method takes [events_suspended::W](events_suspended::W) writer structure"]
impl crate::Writable for EVENTS_SUSPENDED {}
#[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
pub mod events_suspended;
#[doc = "Receive sequence started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxstarted](events_rxstarted) module"]
pub type EVENTS_RXSTARTED = crate::Reg<u32, _EVENTS_RXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXSTARTED;
#[doc = "`read()` method returns [events_rxstarted::R](events_rxstarted::R) reader structure"]
impl crate::Readable for EVENTS_RXSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_rxstarted::W](events_rxstarted::W) writer structure"]
impl crate::Writable for EVENTS_RXSTARTED {}
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "Transmit sequence started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txstarted](events_txstarted) module"]
pub type EVENTS_TXSTARTED = crate::Reg<u32, _EVENTS_TXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXSTARTED;
#[doc = "`read()` method returns [events_txstarted::R](events_txstarted::R) reader structure"]
impl crate::Readable for EVENTS_TXSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_txstarted::W](events_txstarted::W) writer structure"]
impl crate::Writable for EVENTS_TXSTARTED {}
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "Byte boundary, starting to receive the last byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_lastrx](events_lastrx) module"]
pub type EVENTS_LASTRX = crate::Reg<u32, _EVENTS_LASTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_LASTRX;
#[doc = "`read()` method returns [events_lastrx::R](events_lastrx::R) reader structure"]
impl crate::Readable for EVENTS_LASTRX {}
#[doc = "`write(|w| ..)` method takes [events_lastrx::W](events_lastrx::W) writer structure"]
impl crate::Writable for EVENTS_LASTRX {}
#[doc = "Byte boundary, starting to receive the last byte"]
pub mod events_lastrx;
#[doc = "Byte boundary, starting to transmit the last byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_lasttx](events_lasttx) module"]
pub type EVENTS_LASTTX = crate::Reg<u32, _EVENTS_LASTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_LASTTX;
#[doc = "`read()` method returns [events_lasttx::R](events_lasttx::R) reader structure"]
impl crate::Readable for EVENTS_LASTTX {}
#[doc = "`write(|w| ..)` method takes [events_lasttx::W](events_lasttx::W) writer structure"]
impl crate::Writable for EVENTS_LASTTX {}
#[doc = "Byte boundary, starting to transmit the last byte"]
pub mod events_lasttx;
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
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
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
#[doc = "Error source\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorsrc](errorsrc) module"]
pub type ERRORSRC = crate::Reg<u32, _ERRORSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRORSRC;
#[doc = "`read()` method returns [errorsrc::R](errorsrc::R) reader structure"]
impl crate::Readable for ERRORSRC {}
#[doc = "`write(|w| ..)` method takes [errorsrc::W](errorsrc::W) writer structure"]
impl crate::Writable for ERRORSRC {}
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "Enable TWIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable TWIM"]
pub mod enable;
#[doc = "TWI frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](frequency) module"]
pub type FREQUENCY = crate::Reg<u32, _FREQUENCY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQUENCY;
#[doc = "`read()` method returns [frequency::R](frequency::R) reader structure"]
impl crate::Readable for FREQUENCY {}
#[doc = "`write(|w| ..)` method takes [frequency::W](frequency::W) writer structure"]
impl crate::Writable for FREQUENCY {}
#[doc = "TWI frequency"]
pub mod frequency;
#[doc = "Address used in the TWI transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address](address) module"]
pub type ADDRESS = crate::Reg<u32, _ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRESS;
#[doc = "`read()` method returns [address::R](address::R) reader structure"]
impl crate::Readable for ADDRESS {}
#[doc = "`write(|w| ..)` method takes [address::W](address::W) writer structure"]
impl crate::Writable for ADDRESS {}
#[doc = "Address used in the TWI transfer"]
pub mod address;
