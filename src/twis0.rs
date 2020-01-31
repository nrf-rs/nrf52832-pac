#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 20usize],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved3: [u8; 12usize],
    #[doc = "0x30 - Prepare the TWI slave to respond to a write command"]
    pub tasks_preparerx: TASKS_PREPARERX,
    #[doc = "0x34 - Prepare the TWI slave to respond to a read command"]
    pub tasks_preparetx: TASKS_PREPARETX,
    _reserved5: [u8; 204usize],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved6: [u8; 28usize],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved7: [u8; 36usize],
    #[doc = "0x14c - Receive sequence started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - Transmit sequence started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved9: [u8; 16usize],
    #[doc = "0x164 - Write command received"]
    pub events_write: EVENTS_WRITE,
    #[doc = "0x168 - Read command received"]
    pub events_read: EVENTS_READ,
    _reserved11: [u8; 148usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved12: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved15: [u8; 452usize],
    #[doc = "0x4d0 - Error source"]
    pub errorsrc: ERRORSRC,
    #[doc = "0x4d4 - Status register indicating which address had a match"]
    pub match_: MATCH,
    _reserved17: [u8; 40usize],
    #[doc = "0x500 - Enable TWIS"]
    pub enable: ENABLE,
    _reserved18: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved19: [u8; 36usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    _reserved20: [u8; 4usize],
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved21: [u8; 56usize],
    #[doc = "0x588 - Description collection\\[0\\]: TWI slave address 0"]
    pub address: [ADDRESS; 2],
    _reserved22: [u8; 4usize],
    #[doc = "0x594 - Configuration register for the address match mechanism"]
    pub config: CONFIG,
    _reserved23: [u8; 40usize],
    #[doc = "0x5c0 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    pub orc: ORC,
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
    #[doc = "0x00 - RXD Data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in RXD buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last RXD transaction"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - TXD Data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in TXD buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last TXD transaction"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "Stop TWI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop TWI transaction"]
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
#[doc = "Prepare the TWI slave to respond to a write command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_preparerx](tasks_preparerx) module"]
pub type TASKS_PREPARERX = crate::Reg<u32, _TASKS_PREPARERX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_PREPARERX;
#[doc = "`write(|w| ..)` method takes [tasks_preparerx::W](tasks_preparerx::W) writer structure"]
impl crate::Writable for TASKS_PREPARERX {}
#[doc = "Prepare the TWI slave to respond to a write command"]
pub mod tasks_preparerx;
#[doc = "Prepare the TWI slave to respond to a read command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_preparetx](tasks_preparetx) module"]
pub type TASKS_PREPARETX = crate::Reg<u32, _TASKS_PREPARETX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_PREPARETX;
#[doc = "`write(|w| ..)` method takes [tasks_preparetx::W](tasks_preparetx::W) writer structure"]
impl crate::Writable for TASKS_PREPARETX {}
#[doc = "Prepare the TWI slave to respond to a read command"]
pub mod tasks_preparetx;
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
#[doc = "Write command received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_write](events_write) module"]
pub type EVENTS_WRITE = crate::Reg<u32, _EVENTS_WRITE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_WRITE;
#[doc = "`read()` method returns [events_write::R](events_write::R) reader structure"]
impl crate::Readable for EVENTS_WRITE {}
#[doc = "`write(|w| ..)` method takes [events_write::W](events_write::W) writer structure"]
impl crate::Writable for EVENTS_WRITE {}
#[doc = "Write command received"]
pub mod events_write;
#[doc = "Read command received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_read](events_read) module"]
pub type EVENTS_READ = crate::Reg<u32, _EVENTS_READ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READ;
#[doc = "`read()` method returns [events_read::R](events_read::R) reader structure"]
impl crate::Readable for EVENTS_READ {}
#[doc = "`write(|w| ..)` method takes [events_read::W](events_read::W) writer structure"]
impl crate::Writable for EVENTS_READ {}
#[doc = "Read command received"]
pub mod events_read;
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
#[doc = "Status register indicating which address had a match\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_](match_) module"]
pub type MATCH = crate::Reg<u32, _MATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH;
#[doc = "`read()` method returns [match_::R](match_::R) reader structure"]
impl crate::Readable for MATCH {}
#[doc = "Status register indicating which address had a match"]
pub mod match_;
#[doc = "Enable TWIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable TWIS"]
pub mod enable;
#[doc = "Description collection\\[0\\]: TWI slave address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address](address) module"]
pub type ADDRESS = crate::Reg<u32, _ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRESS;
#[doc = "`read()` method returns [address::R](address::R) reader structure"]
impl crate::Readable for ADDRESS {}
#[doc = "`write(|w| ..)` method takes [address::W](address::W) writer structure"]
impl crate::Writable for ADDRESS {}
#[doc = "Description collection\\[0\\]: TWI slave address 0"]
pub mod address;
#[doc = "Configuration register for the address match mechanism\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration register for the address match mechanism"]
pub mod config;
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orc](orc) module"]
pub type ORC = crate::Reg<u32, _ORC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORC;
#[doc = "`read()` method returns [orc::R](orc::R) reader structure"]
impl crate::Readable for ORC {}
#[doc = "`write(|w| ..)` method takes [orc::W](orc::W) writer structure"]
impl crate::Writable for ORC {}
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub mod orc;
