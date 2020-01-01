#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start UART receiver"]
    pub tasks_startrx: TASKS_STARTRX,
    #[doc = "0x04 - Stop UART receiver"]
    pub tasks_stoprx: TASKS_STOPRX,
    #[doc = "0x08 - Start UART transmitter"]
    pub tasks_starttx: TASKS_STARTTX,
    #[doc = "0x0c - Stop UART transmitter"]
    pub tasks_stoptx: TASKS_STOPTX,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - Suspend UART"]
    pub tasks_suspend: TASKS_SUSPEND,
    _reserved5: [u8; 224usize],
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    pub events_cts: EVENTS_CTS,
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    pub events_ncts: EVENTS_NCTS,
    #[doc = "0x108 - Data received in RXD"]
    pub events_rxdrdy: EVENTS_RXDRDY,
    _reserved8: [u8; 16usize],
    #[doc = "0x11c - Data sent from TXD"]
    pub events_txdrdy: EVENTS_TXDRDY,
    _reserved9: [u8; 4usize],
    #[doc = "0x124 - Error detected"]
    pub events_error: EVENTS_ERROR,
    _reserved10: [u8; 28usize],
    #[doc = "0x144 - Receiver timeout"]
    pub events_rxto: EVENTS_RXTO,
    _reserved11: [u8; 184usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved12: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 372usize],
    #[doc = "0x480 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 124usize],
    #[doc = "0x500 - Enable UART"]
    pub enable: ENABLE,
    _reserved16: [u8; 4usize],
    #[doc = "0x508 - Pin select for RTS"]
    pub pselrts: PSELRTS,
    #[doc = "0x50c - Pin select for TXD"]
    pub pseltxd: PSELTXD,
    #[doc = "0x510 - Pin select for CTS"]
    pub pselcts: PSELCTS,
    #[doc = "0x514 - Pin select for RXD"]
    pub pselrxd: PSELRXD,
    #[doc = "0x518 - RXD register"]
    pub rxd: RXD,
    #[doc = "0x51c - TXD register"]
    pub txd: TXD,
    _reserved22: [u8; 4usize],
    #[doc = "0x524 - Baud rate"]
    pub baudrate: BAUDRATE,
    _reserved23: [u8; 68usize],
    #[doc = "0x56c - Configuration of parity and hardware flow control"]
    pub config: CONFIG,
}
#[doc = "Start UART receiver\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startrx](tasks_startrx) module"]
pub type TASKS_STARTRX = crate::Reg<u32, _TASKS_STARTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTRX;
#[doc = "`write(|w| ..)` method takes [tasks_startrx::W](tasks_startrx::W) writer structure"]
impl crate::Writable for TASKS_STARTRX {}
#[doc = "Start UART receiver"]
pub mod tasks_startrx;
#[doc = "Stop UART receiver\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stoprx](tasks_stoprx) module"]
pub type TASKS_STOPRX = crate::Reg<u32, _TASKS_STOPRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOPRX;
#[doc = "`write(|w| ..)` method takes [tasks_stoprx::W](tasks_stoprx::W) writer structure"]
impl crate::Writable for TASKS_STOPRX {}
#[doc = "Stop UART receiver"]
pub mod tasks_stoprx;
#[doc = "Start UART transmitter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_starttx](tasks_starttx) module"]
pub type TASKS_STARTTX = crate::Reg<u32, _TASKS_STARTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTTX;
#[doc = "`write(|w| ..)` method takes [tasks_starttx::W](tasks_starttx::W) writer structure"]
impl crate::Writable for TASKS_STARTTX {}
#[doc = "Start UART transmitter"]
pub mod tasks_starttx;
#[doc = "Stop UART transmitter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stoptx](tasks_stoptx) module"]
pub type TASKS_STOPTX = crate::Reg<u32, _TASKS_STOPTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOPTX;
#[doc = "`write(|w| ..)` method takes [tasks_stoptx::W](tasks_stoptx::W) writer structure"]
impl crate::Writable for TASKS_STOPTX {}
#[doc = "Stop UART transmitter"]
pub mod tasks_stoptx;
#[doc = "Suspend UART\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_suspend](tasks_suspend) module"]
pub type TASKS_SUSPEND = crate::Reg<u32, _TASKS_SUSPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SUSPEND;
#[doc = "`write(|w| ..)` method takes [tasks_suspend::W](tasks_suspend::W) writer structure"]
impl crate::Writable for TASKS_SUSPEND {}
#[doc = "Suspend UART"]
pub mod tasks_suspend;
#[doc = "CTS is activated (set low). Clear To Send.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_cts](events_cts) module"]
pub type EVENTS_CTS = crate::Reg<u32, _EVENTS_CTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CTS;
#[doc = "`read()` method returns [events_cts::R](events_cts::R) reader structure"]
impl crate::Readable for EVENTS_CTS {}
#[doc = "`write(|w| ..)` method takes [events_cts::W](events_cts::W) writer structure"]
impl crate::Writable for EVENTS_CTS {}
#[doc = "CTS is activated (set low). Clear To Send."]
pub mod events_cts;
#[doc = "CTS is deactivated (set high). Not Clear To Send.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ncts](events_ncts) module"]
pub type EVENTS_NCTS = crate::Reg<u32, _EVENTS_NCTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_NCTS;
#[doc = "`read()` method returns [events_ncts::R](events_ncts::R) reader structure"]
impl crate::Readable for EVENTS_NCTS {}
#[doc = "`write(|w| ..)` method takes [events_ncts::W](events_ncts::W) writer structure"]
impl crate::Writable for EVENTS_NCTS {}
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub mod events_ncts;
#[doc = "Data received in RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxdrdy](events_rxdrdy) module"]
pub type EVENTS_RXDRDY = crate::Reg<u32, _EVENTS_RXDRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXDRDY;
#[doc = "`read()` method returns [events_rxdrdy::R](events_rxdrdy::R) reader structure"]
impl crate::Readable for EVENTS_RXDRDY {}
#[doc = "`write(|w| ..)` method takes [events_rxdrdy::W](events_rxdrdy::W) writer structure"]
impl crate::Writable for EVENTS_RXDRDY {}
#[doc = "Data received in RXD"]
pub mod events_rxdrdy;
#[doc = "Data sent from TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txdrdy](events_txdrdy) module"]
pub type EVENTS_TXDRDY = crate::Reg<u32, _EVENTS_TXDRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXDRDY;
#[doc = "`read()` method returns [events_txdrdy::R](events_txdrdy::R) reader structure"]
impl crate::Readable for EVENTS_TXDRDY {}
#[doc = "`write(|w| ..)` method takes [events_txdrdy::W](events_txdrdy::W) writer structure"]
impl crate::Writable for EVENTS_TXDRDY {}
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "Error detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "Error detected"]
pub mod events_error;
#[doc = "Receiver timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxto](events_rxto) module"]
pub type EVENTS_RXTO = crate::Reg<u32, _EVENTS_RXTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXTO;
#[doc = "`read()` method returns [events_rxto::R](events_rxto::R) reader structure"]
impl crate::Readable for EVENTS_RXTO {}
#[doc = "`write(|w| ..)` method takes [events_rxto::W](events_rxto::W) writer structure"]
impl crate::Writable for EVENTS_RXTO {}
#[doc = "Receiver timeout"]
pub mod events_rxto;
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
#[doc = "Enable UART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable UART"]
pub mod enable;
#[doc = "Pin select for RTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselrts](pselrts) module"]
pub type PSELRTS = crate::Reg<u32, _PSELRTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELRTS;
#[doc = "`read()` method returns [pselrts::R](pselrts::R) reader structure"]
impl crate::Readable for PSELRTS {}
#[doc = "`write(|w| ..)` method takes [pselrts::W](pselrts::W) writer structure"]
impl crate::Writable for PSELRTS {}
#[doc = "Pin select for RTS"]
pub mod pselrts;
#[doc = "Pin select for TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pseltxd](pseltxd) module"]
pub type PSELTXD = crate::Reg<u32, _PSELTXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELTXD;
#[doc = "`read()` method returns [pseltxd::R](pseltxd::R) reader structure"]
impl crate::Readable for PSELTXD {}
#[doc = "`write(|w| ..)` method takes [pseltxd::W](pseltxd::W) writer structure"]
impl crate::Writable for PSELTXD {}
#[doc = "Pin select for TXD"]
pub mod pseltxd;
#[doc = "Pin select for CTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselcts](pselcts) module"]
pub type PSELCTS = crate::Reg<u32, _PSELCTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELCTS;
#[doc = "`read()` method returns [pselcts::R](pselcts::R) reader structure"]
impl crate::Readable for PSELCTS {}
#[doc = "`write(|w| ..)` method takes [pselcts::W](pselcts::W) writer structure"]
impl crate::Writable for PSELCTS {}
#[doc = "Pin select for CTS"]
pub mod pselcts;
#[doc = "Pin select for RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselrxd](pselrxd) module"]
pub type PSELRXD = crate::Reg<u32, _PSELRXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELRXD;
#[doc = "`read()` method returns [pselrxd::R](pselrxd::R) reader structure"]
impl crate::Readable for PSELRXD {}
#[doc = "`write(|w| ..)` method takes [pselrxd::W](pselrxd::W) writer structure"]
impl crate::Writable for PSELRXD {}
#[doc = "Pin select for RXD"]
pub mod pselrxd;
#[doc = "RXD register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd](rxd) module"]
pub type RXD = crate::Reg<u32, _RXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXD;
#[doc = "`read()` method returns [rxd::R](rxd::R) reader structure"]
impl crate::Readable for RXD {}
#[doc = "RXD register"]
pub mod rxd;
#[doc = "TXD register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txd](txd) module"]
pub type TXD = crate::Reg<u32, _TXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXD;
#[doc = "`write(|w| ..)` method takes [txd::W](txd::W) writer structure"]
impl crate::Writable for TXD {}
#[doc = "TXD register"]
pub mod txd;
#[doc = "Baud rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baudrate](baudrate) module"]
pub type BAUDRATE = crate::Reg<u32, _BAUDRATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUDRATE;
#[doc = "`read()` method returns [baudrate::R](baudrate::R) reader structure"]
impl crate::Readable for BAUDRATE {}
#[doc = "`write(|w| ..)` method takes [baudrate::W](baudrate::W) writer structure"]
impl crate::Writable for BAUDRATE {}
#[doc = "Baud rate"]
pub mod baudrate;
#[doc = "Configuration of parity and hardware flow control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration of parity and hardware flow control"]
pub mod config;
