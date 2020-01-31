#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start TWI receive sequence"]
    pub tasks_startrx: TASKS_STARTRX,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Start TWI transmit sequence"]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved5: [u8; 224usize],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - TWI RXD byte received"]
    pub events_rxdready: EVENTS_RXDREADY,
    _reserved7: [u8; 16usize],
    #[doc = "0x11c - TWI TXD byte sent"]
    pub events_txdsent: EVENTS_TXDSENT,
    _reserved8: [u8; 4usize],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved9: [u8; 16usize],
    #[doc = "0x138 - TWI byte boundary, generated before each byte that is sent or received"]
    pub events_bb: EVENTS_BB,
    _reserved10: [u8; 12usize],
    #[doc = "0x148 - TWI entered the suspended state"]
    pub events_suspended: EVENTS_SUSPENDED,
    _reserved11: [u8; 180usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved12: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 440usize],
    #[doc = "0x4c4 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 56usize],
    #[doc = "0x500 - Enable TWI"]
    pub enable: ENABLE,
    _reserved16: [u8; 4usize],
    #[doc = "0x508 - Pin select for SCL"]
    pub pselscl: PSELSCL,
    #[doc = "0x50c - Pin select for SDA"]
    pub pselsda: PSELSDA,
    _reserved18: [u8; 8usize],
    #[doc = "0x518 - RXD register"]
    pub rxd: RXD,
    #[doc = "0x51c - TXD register"]
    pub txd: TXD,
    _reserved20: [u8; 4usize],
    #[doc = "0x524 - TWI frequency"]
    pub frequency: FREQUENCY,
    _reserved21: [u8; 96usize],
    #[doc = "0x588 - Address used in the TWI transfer"]
    pub address: ADDRESS,
}
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
#[doc = "TWI RXD byte received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxdready](events_rxdready) module"]
pub type EVENTS_RXDREADY = crate::Reg<u32, _EVENTS_RXDREADY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXDREADY;
#[doc = "`read()` method returns [events_rxdready::R](events_rxdready::R) reader structure"]
impl crate::Readable for EVENTS_RXDREADY {}
#[doc = "`write(|w| ..)` method takes [events_rxdready::W](events_rxdready::W) writer structure"]
impl crate::Writable for EVENTS_RXDREADY {}
#[doc = "TWI RXD byte received"]
pub mod events_rxdready;
#[doc = "TWI TXD byte sent\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txdsent](events_txdsent) module"]
pub type EVENTS_TXDSENT = crate::Reg<u32, _EVENTS_TXDSENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXDSENT;
#[doc = "`read()` method returns [events_txdsent::R](events_txdsent::R) reader structure"]
impl crate::Readable for EVENTS_TXDSENT {}
#[doc = "`write(|w| ..)` method takes [events_txdsent::W](events_txdsent::W) writer structure"]
impl crate::Writable for EVENTS_TXDSENT {}
#[doc = "TWI TXD byte sent"]
pub mod events_txdsent;
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
#[doc = "TWI byte boundary, generated before each byte that is sent or received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_bb](events_bb) module"]
pub type EVENTS_BB = crate::Reg<u32, _EVENTS_BB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_BB;
#[doc = "`read()` method returns [events_bb::R](events_bb::R) reader structure"]
impl crate::Readable for EVENTS_BB {}
#[doc = "`write(|w| ..)` method takes [events_bb::W](events_bb::W) writer structure"]
impl crate::Writable for EVENTS_BB {}
#[doc = "TWI byte boundary, generated before each byte that is sent or received"]
pub mod events_bb;
#[doc = "TWI entered the suspended state\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_suspended](events_suspended) module"]
pub type EVENTS_SUSPENDED = crate::Reg<u32, _EVENTS_SUSPENDED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SUSPENDED;
#[doc = "`read()` method returns [events_suspended::R](events_suspended::R) reader structure"]
impl crate::Readable for EVENTS_SUSPENDED {}
#[doc = "`write(|w| ..)` method takes [events_suspended::W](events_suspended::W) writer structure"]
impl crate::Writable for EVENTS_SUSPENDED {}
#[doc = "TWI entered the suspended state"]
pub mod events_suspended;
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
#[doc = "Enable TWI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable TWI"]
pub mod enable;
#[doc = "Pin select for SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselscl](pselscl) module"]
pub type PSELSCL = crate::Reg<u32, _PSELSCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELSCL;
#[doc = "`read()` method returns [pselscl::R](pselscl::R) reader structure"]
impl crate::Readable for PSELSCL {}
#[doc = "`write(|w| ..)` method takes [pselscl::W](pselscl::W) writer structure"]
impl crate::Writable for PSELSCL {}
#[doc = "Pin select for SCL"]
pub mod pselscl;
#[doc = "Pin select for SDA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselsda](pselsda) module"]
pub type PSELSDA = crate::Reg<u32, _PSELSDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELSDA;
#[doc = "`read()` method returns [pselsda::R](pselsda::R) reader structure"]
impl crate::Readable for PSELSDA {}
#[doc = "`write(|w| ..)` method takes [pselsda::W](pselsda::W) writer structure"]
impl crate::Writable for PSELSDA {}
#[doc = "Pin select for SDA"]
pub mod pselsda;
#[doc = "RXD register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd](rxd) module"]
pub type RXD = crate::Reg<u32, _RXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXD;
#[doc = "`read()` method returns [rxd::R](rxd::R) reader structure"]
impl crate::Readable for RXD {}
#[doc = "RXD register"]
pub mod rxd;
#[doc = "TXD register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txd](txd) module"]
pub type TXD = crate::Reg<u32, _TXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXD;
#[doc = "`read()` method returns [txd::R](txd::R) reader structure"]
impl crate::Readable for TXD {}
#[doc = "`write(|w| ..)` method takes [txd::W](txd::W) writer structure"]
impl crate::Writable for TXD {}
#[doc = "TXD register"]
pub mod txd;
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
