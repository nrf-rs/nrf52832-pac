#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Activate NFC peripheral for incoming and outgoing frames, change state to activated"]
    pub tasks_activate: TASKS_ACTIVATE,
    #[doc = "0x04 - Disable NFC peripheral"]
    pub tasks_disable: TASKS_DISABLE,
    #[doc = "0x08 - Enable NFC sense field mode, change state to sense mode"]
    pub tasks_sense: TASKS_SENSE,
    #[doc = "0x0c - Start transmission of a outgoing frame, change state to transmit"]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - Initializes the EasyDMA for receive."]
    pub tasks_enablerxdata: TASKS_ENABLERXDATA,
    _reserved5: [u8; 4usize],
    #[doc = "0x24 - Force state machine to IDLE state"]
    pub tasks_goidle: TASKS_GOIDLE,
    #[doc = "0x28 - Force state machine to SLEEP_A state"]
    pub tasks_gosleep: TASKS_GOSLEEP,
    _reserved7: [u8; 212usize],
    #[doc = "0x100 - The NFC peripheral is ready to receive and send frames"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Remote NFC field detected"]
    pub events_fielddetected: EVENTS_FIELDDETECTED,
    #[doc = "0x108 - Remote NFC field lost"]
    pub events_fieldlost: EVENTS_FIELDLOST,
    #[doc = "0x10c - Marks the start of the first symbol of a transmitted frame"]
    pub events_txframestart: EVENTS_TXFRAMESTART,
    #[doc = "0x110 - Marks the end of the last transmitted on-air symbol of a frame"]
    pub events_txframeend: EVENTS_TXFRAMEEND,
    #[doc = "0x114 - Marks the end of the first symbol of a received frame"]
    pub events_rxframestart: EVENTS_RXFRAMESTART,
    #[doc = "0x118 - Received data have been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
    pub events_rxframeend: EVENTS_RXFRAMEEND,
    #[doc = "0x11c - NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
    pub events_error: EVENTS_ERROR,
    _reserved15: [u8; 8usize],
    #[doc = "0x128 - NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
    pub events_rxerror: EVENTS_RXERROR,
    #[doc = "0x12c - RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
    pub events_endrx: EVENTS_ENDRX,
    #[doc = "0x130 - Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
    pub events_endtx: EVENTS_ENDTX,
    _reserved18: [u8; 4usize],
    #[doc = "0x138 - Auto collision resolution process has started"]
    pub events_autocolresstarted: EVENTS_AUTOCOLRESSTARTED,
    _reserved19: [u8; 12usize],
    #[doc = "0x148 - NFC Auto collision resolution error reported."]
    pub events_collision: EVENTS_COLLISION,
    #[doc = "0x14c - NFC Auto collision resolution successfully completed"]
    pub events_selected: EVENTS_SELECTED,
    #[doc = "0x150 - EasyDMA is ready to receive or send frames."]
    pub events_started: EVENTS_STARTED,
    _reserved22: [u8; 172usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved23: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved26: [u8; 248usize],
    #[doc = "0x404 - NFC Error Status register"]
    pub errorstatus: ERRORSTATUS,
    _reserved27: [u8; 4usize],
    #[doc = "0x40c - Unspecified"]
    pub framestatus: FRAMESTATUS,
    _reserved28: [u8; 32usize],
    #[doc = "0x430 - Current value driven to the NFC Load Control"]
    pub currentloadctrl: CURRENTLOADCTRL,
    _reserved29: [u8; 8usize],
    #[doc = "0x43c - Indicates the presence or not of a valid field"]
    pub fieldpresent: FIELDPRESENT,
    _reserved30: [u8; 196usize],
    #[doc = "0x504 - Minimum frame delay"]
    pub framedelaymin: FRAMEDELAYMIN,
    #[doc = "0x508 - Maximum frame delay"]
    pub framedelaymax: FRAMEDELAYMAX,
    #[doc = "0x50c - Configuration register for the Frame Delay Timer"]
    pub framedelaymode: FRAMEDELAYMODE,
    #[doc = "0x510 - Packet pointer for TXD and RXD data storage in Data RAM"]
    pub packetptr: PACKETPTR,
    #[doc = "0x514 - Size of allocated for TXD and RXD data storage buffer in Data RAM"]
    pub maxlen: MAXLEN,
    #[doc = "0x518 - Unspecified"]
    pub txd: TXD,
    #[doc = "0x520 - Unspecified"]
    pub rxd: RXD,
    _reserved37: [u8; 104usize],
    #[doc = "0x590 - Last NFCID1 part (4, 7 or 10 bytes ID)"]
    pub nfcid1_last: NFCID1_LAST,
    #[doc = "0x594 - Second last NFCID1 part (7 or 10 bytes ID)"]
    pub nfcid1_2nd_last: NFCID1_2ND_LAST,
    #[doc = "0x598 - Third last NFCID1 part (10 bytes ID)"]
    pub nfcid1_3rd_last: NFCID1_3RD_LAST,
    _reserved40: [u8; 4usize],
    #[doc = "0x5a0 - NFC-A SENS_RES auto-response settings"]
    pub sensres: SENSRES,
    #[doc = "0x5a4 - NFC-A SEL_RES auto-response settings"]
    pub selres: SELRES,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct FRAMESTATUS {
    #[doc = "0x00 - Result of last incoming frames"]
    pub rx: self::framestatus::RX,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod framestatus;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Configuration of outgoing frames"]
    pub frameconfig: self::txd::FRAMECONFIG,
    #[doc = "0x04 - Size of outgoing frame"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Configuration of incoming frames"]
    pub frameconfig: self::rxd::FRAMECONFIG,
    #[doc = "0x04 - Size of last incoming frame"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = "Activate NFC peripheral for incoming and outgoing frames, change state to activated\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_activate](tasks_activate) module"]
pub type TASKS_ACTIVATE = crate::Reg<u32, _TASKS_ACTIVATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_ACTIVATE;
#[doc = "`write(|w| ..)` method takes [tasks_activate::W](tasks_activate::W) writer structure"]
impl crate::Writable for TASKS_ACTIVATE {}
#[doc = "Activate NFC peripheral for incoming and outgoing frames, change state to activated"]
pub mod tasks_activate;
#[doc = "Disable NFC peripheral\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_disable](tasks_disable) module"]
pub type TASKS_DISABLE = crate::Reg<u32, _TASKS_DISABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_DISABLE;
#[doc = "`write(|w| ..)` method takes [tasks_disable::W](tasks_disable::W) writer structure"]
impl crate::Writable for TASKS_DISABLE {}
#[doc = "Disable NFC peripheral"]
pub mod tasks_disable;
#[doc = "Enable NFC sense field mode, change state to sense mode\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_sense](tasks_sense) module"]
pub type TASKS_SENSE = crate::Reg<u32, _TASKS_SENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SENSE;
#[doc = "`write(|w| ..)` method takes [tasks_sense::W](tasks_sense::W) writer structure"]
impl crate::Writable for TASKS_SENSE {}
#[doc = "Enable NFC sense field mode, change state to sense mode"]
pub mod tasks_sense;
#[doc = "Start transmission of a outgoing frame, change state to transmit\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_starttx](tasks_starttx) module"]
pub type TASKS_STARTTX = crate::Reg<u32, _TASKS_STARTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTTX;
#[doc = "`write(|w| ..)` method takes [tasks_starttx::W](tasks_starttx::W) writer structure"]
impl crate::Writable for TASKS_STARTTX {}
#[doc = "Start transmission of a outgoing frame, change state to transmit"]
pub mod tasks_starttx;
#[doc = "Initializes the EasyDMA for receive.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_enablerxdata](tasks_enablerxdata) module"]
pub type TASKS_ENABLERXDATA = crate::Reg<u32, _TASKS_ENABLERXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_ENABLERXDATA;
#[doc = "`write(|w| ..)` method takes [tasks_enablerxdata::W](tasks_enablerxdata::W) writer structure"]
impl crate::Writable for TASKS_ENABLERXDATA {}
#[doc = "Initializes the EasyDMA for receive."]
pub mod tasks_enablerxdata;
#[doc = "Force state machine to IDLE state\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_goidle](tasks_goidle) module"]
pub type TASKS_GOIDLE = crate::Reg<u32, _TASKS_GOIDLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_GOIDLE;
#[doc = "`write(|w| ..)` method takes [tasks_goidle::W](tasks_goidle::W) writer structure"]
impl crate::Writable for TASKS_GOIDLE {}
#[doc = "Force state machine to IDLE state"]
pub mod tasks_goidle;
#[doc = "Force state machine to SLEEP_A state\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_gosleep](tasks_gosleep) module"]
pub type TASKS_GOSLEEP = crate::Reg<u32, _TASKS_GOSLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_GOSLEEP;
#[doc = "`write(|w| ..)` method takes [tasks_gosleep::W](tasks_gosleep::W) writer structure"]
impl crate::Writable for TASKS_GOSLEEP {}
#[doc = "Force state machine to SLEEP_A state"]
pub mod tasks_gosleep;
#[doc = "The NFC peripheral is ready to receive and send frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ready](events_ready) module"]
pub type EVENTS_READY = crate::Reg<u32, _EVENTS_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READY;
#[doc = "`read()` method returns [events_ready::R](events_ready::R) reader structure"]
impl crate::Readable for EVENTS_READY {}
#[doc = "`write(|w| ..)` method takes [events_ready::W](events_ready::W) writer structure"]
impl crate::Writable for EVENTS_READY {}
#[doc = "The NFC peripheral is ready to receive and send frames"]
pub mod events_ready;
#[doc = "Remote NFC field detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_fielddetected](events_fielddetected) module"]
pub type EVENTS_FIELDDETECTED = crate::Reg<u32, _EVENTS_FIELDDETECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_FIELDDETECTED;
#[doc = "`read()` method returns [events_fielddetected::R](events_fielddetected::R) reader structure"]
impl crate::Readable for EVENTS_FIELDDETECTED {}
#[doc = "`write(|w| ..)` method takes [events_fielddetected::W](events_fielddetected::W) writer structure"]
impl crate::Writable for EVENTS_FIELDDETECTED {}
#[doc = "Remote NFC field detected"]
pub mod events_fielddetected;
#[doc = "Remote NFC field lost\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_fieldlost](events_fieldlost) module"]
pub type EVENTS_FIELDLOST = crate::Reg<u32, _EVENTS_FIELDLOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_FIELDLOST;
#[doc = "`read()` method returns [events_fieldlost::R](events_fieldlost::R) reader structure"]
impl crate::Readable for EVENTS_FIELDLOST {}
#[doc = "`write(|w| ..)` method takes [events_fieldlost::W](events_fieldlost::W) writer structure"]
impl crate::Writable for EVENTS_FIELDLOST {}
#[doc = "Remote NFC field lost"]
pub mod events_fieldlost;
#[doc = "Marks the start of the first symbol of a transmitted frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txframestart](events_txframestart) module"]
pub type EVENTS_TXFRAMESTART = crate::Reg<u32, _EVENTS_TXFRAMESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXFRAMESTART;
#[doc = "`read()` method returns [events_txframestart::R](events_txframestart::R) reader structure"]
impl crate::Readable for EVENTS_TXFRAMESTART {}
#[doc = "`write(|w| ..)` method takes [events_txframestart::W](events_txframestart::W) writer structure"]
impl crate::Writable for EVENTS_TXFRAMESTART {}
#[doc = "Marks the start of the first symbol of a transmitted frame"]
pub mod events_txframestart;
#[doc = "Marks the end of the last transmitted on-air symbol of a frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txframeend](events_txframeend) module"]
pub type EVENTS_TXFRAMEEND = crate::Reg<u32, _EVENTS_TXFRAMEEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXFRAMEEND;
#[doc = "`read()` method returns [events_txframeend::R](events_txframeend::R) reader structure"]
impl crate::Readable for EVENTS_TXFRAMEEND {}
#[doc = "`write(|w| ..)` method takes [events_txframeend::W](events_txframeend::W) writer structure"]
impl crate::Writable for EVENTS_TXFRAMEEND {}
#[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
pub mod events_txframeend;
#[doc = "Marks the end of the first symbol of a received frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxframestart](events_rxframestart) module"]
pub type EVENTS_RXFRAMESTART = crate::Reg<u32, _EVENTS_RXFRAMESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXFRAMESTART;
#[doc = "`read()` method returns [events_rxframestart::R](events_rxframestart::R) reader structure"]
impl crate::Readable for EVENTS_RXFRAMESTART {}
#[doc = "`write(|w| ..)` method takes [events_rxframestart::W](events_rxframestart::W) writer structure"]
impl crate::Writable for EVENTS_RXFRAMESTART {}
#[doc = "Marks the end of the first symbol of a received frame"]
pub mod events_rxframestart;
#[doc = "Received data have been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxframeend](events_rxframeend) module"]
pub type EVENTS_RXFRAMEEND = crate::Reg<u32, _EVENTS_RXFRAMEEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXFRAMEEND;
#[doc = "`read()` method returns [events_rxframeend::R](events_rxframeend::R) reader structure"]
impl crate::Readable for EVENTS_RXFRAMEEND {}
#[doc = "`write(|w| ..)` method takes [events_rxframeend::W](events_rxframeend::W) writer structure"]
impl crate::Writable for EVENTS_RXFRAMEEND {}
#[doc = "Received data have been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
pub mod events_rxframeend;
#[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
pub mod events_error;
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxerror](events_rxerror) module"]
pub type EVENTS_RXERROR = crate::Reg<u32, _EVENTS_RXERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXERROR;
#[doc = "`read()` method returns [events_rxerror::R](events_rxerror::R) reader structure"]
impl crate::Readable for EVENTS_RXERROR {}
#[doc = "`write(|w| ..)` method takes [events_rxerror::W](events_rxerror::W) writer structure"]
impl crate::Writable for EVENTS_RXERROR {}
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
pub mod events_rxerror;
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endrx](events_endrx) module"]
pub type EVENTS_ENDRX = crate::Reg<u32, _EVENTS_ENDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDRX;
#[doc = "`read()` method returns [events_endrx::R](events_endrx::R) reader structure"]
impl crate::Readable for EVENTS_ENDRX {}
#[doc = "`write(|w| ..)` method takes [events_endrx::W](events_endrx::W) writer structure"]
impl crate::Writable for EVENTS_ENDRX {}
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
pub mod events_endrx;
#[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endtx](events_endtx) module"]
pub type EVENTS_ENDTX = crate::Reg<u32, _EVENTS_ENDTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDTX;
#[doc = "`read()` method returns [events_endtx::R](events_endtx::R) reader structure"]
impl crate::Readable for EVENTS_ENDTX {}
#[doc = "`write(|w| ..)` method takes [events_endtx::W](events_endtx::W) writer structure"]
impl crate::Writable for EVENTS_ENDTX {}
#[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
pub mod events_endtx;
#[doc = "Auto collision resolution process has started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_autocolresstarted](events_autocolresstarted) module"]
pub type EVENTS_AUTOCOLRESSTARTED = crate::Reg<u32, _EVENTS_AUTOCOLRESSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_AUTOCOLRESSTARTED;
#[doc = "`read()` method returns [events_autocolresstarted::R](events_autocolresstarted::R) reader structure"]
impl crate::Readable for EVENTS_AUTOCOLRESSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_autocolresstarted::W](events_autocolresstarted::W) writer structure"]
impl crate::Writable for EVENTS_AUTOCOLRESSTARTED {}
#[doc = "Auto collision resolution process has started"]
pub mod events_autocolresstarted;
#[doc = "NFC Auto collision resolution error reported.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_collision](events_collision) module"]
pub type EVENTS_COLLISION = crate::Reg<u32, _EVENTS_COLLISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_COLLISION;
#[doc = "`read()` method returns [events_collision::R](events_collision::R) reader structure"]
impl crate::Readable for EVENTS_COLLISION {}
#[doc = "`write(|w| ..)` method takes [events_collision::W](events_collision::W) writer structure"]
impl crate::Writable for EVENTS_COLLISION {}
#[doc = "NFC Auto collision resolution error reported."]
pub mod events_collision;
#[doc = "NFC Auto collision resolution successfully completed\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_selected](events_selected) module"]
pub type EVENTS_SELECTED = crate::Reg<u32, _EVENTS_SELECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SELECTED;
#[doc = "`read()` method returns [events_selected::R](events_selected::R) reader structure"]
impl crate::Readable for EVENTS_SELECTED {}
#[doc = "`write(|w| ..)` method takes [events_selected::W](events_selected::W) writer structure"]
impl crate::Writable for EVENTS_SELECTED {}
#[doc = "NFC Auto collision resolution successfully completed"]
pub mod events_selected;
#[doc = "EasyDMA is ready to receive or send frames.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_started](events_started) module"]
pub type EVENTS_STARTED = crate::Reg<u32, _EVENTS_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STARTED;
#[doc = "`read()` method returns [events_started::R](events_started::R) reader structure"]
impl crate::Readable for EVENTS_STARTED {}
#[doc = "`write(|w| ..)` method takes [events_started::W](events_started::W) writer structure"]
impl crate::Writable for EVENTS_STARTED {}
#[doc = "EasyDMA is ready to receive or send frames."]
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
#[doc = "NFC Error Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorstatus](errorstatus) module"]
pub type ERRORSTATUS = crate::Reg<u32, _ERRORSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRORSTATUS;
#[doc = "`read()` method returns [errorstatus::R](errorstatus::R) reader structure"]
impl crate::Readable for ERRORSTATUS {}
#[doc = "`write(|w| ..)` method takes [errorstatus::W](errorstatus::W) writer structure"]
impl crate::Writable for ERRORSTATUS {}
#[doc = "NFC Error Status register"]
pub mod errorstatus;
#[doc = "Current value driven to the NFC Load Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [currentloadctrl](currentloadctrl) module"]
pub type CURRENTLOADCTRL = crate::Reg<u32, _CURRENTLOADCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURRENTLOADCTRL;
#[doc = "`read()` method returns [currentloadctrl::R](currentloadctrl::R) reader structure"]
impl crate::Readable for CURRENTLOADCTRL {}
#[doc = "Current value driven to the NFC Load Control"]
pub mod currentloadctrl;
#[doc = "Indicates the presence or not of a valid field\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fieldpresent](fieldpresent) module"]
pub type FIELDPRESENT = crate::Reg<u32, _FIELDPRESENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIELDPRESENT;
#[doc = "`read()` method returns [fieldpresent::R](fieldpresent::R) reader structure"]
impl crate::Readable for FIELDPRESENT {}
#[doc = "Indicates the presence or not of a valid field"]
pub mod fieldpresent;
#[doc = "Minimum frame delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedelaymin](framedelaymin) module"]
pub type FRAMEDELAYMIN = crate::Reg<u32, _FRAMEDELAYMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMEDELAYMIN;
#[doc = "`read()` method returns [framedelaymin::R](framedelaymin::R) reader structure"]
impl crate::Readable for FRAMEDELAYMIN {}
#[doc = "`write(|w| ..)` method takes [framedelaymin::W](framedelaymin::W) writer structure"]
impl crate::Writable for FRAMEDELAYMIN {}
#[doc = "Minimum frame delay"]
pub mod framedelaymin;
#[doc = "Maximum frame delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedelaymax](framedelaymax) module"]
pub type FRAMEDELAYMAX = crate::Reg<u32, _FRAMEDELAYMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMEDELAYMAX;
#[doc = "`read()` method returns [framedelaymax::R](framedelaymax::R) reader structure"]
impl crate::Readable for FRAMEDELAYMAX {}
#[doc = "`write(|w| ..)` method takes [framedelaymax::W](framedelaymax::W) writer structure"]
impl crate::Writable for FRAMEDELAYMAX {}
#[doc = "Maximum frame delay"]
pub mod framedelaymax;
#[doc = "Configuration register for the Frame Delay Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedelaymode](framedelaymode) module"]
pub type FRAMEDELAYMODE = crate::Reg<u32, _FRAMEDELAYMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMEDELAYMODE;
#[doc = "`read()` method returns [framedelaymode::R](framedelaymode::R) reader structure"]
impl crate::Readable for FRAMEDELAYMODE {}
#[doc = "`write(|w| ..)` method takes [framedelaymode::W](framedelaymode::W) writer structure"]
impl crate::Writable for FRAMEDELAYMODE {}
#[doc = "Configuration register for the Frame Delay Timer"]
pub mod framedelaymode;
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packetptr](packetptr) module"]
pub type PACKETPTR = crate::Reg<u32, _PACKETPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKETPTR;
#[doc = "`read()` method returns [packetptr::R](packetptr::R) reader structure"]
impl crate::Readable for PACKETPTR {}
#[doc = "`write(|w| ..)` method takes [packetptr::W](packetptr::W) writer structure"]
impl crate::Writable for PACKETPTR {}
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
pub mod packetptr;
#[doc = "Size of allocated for TXD and RXD data storage buffer in Data RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxlen](maxlen) module"]
pub type MAXLEN = crate::Reg<u32, _MAXLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXLEN;
#[doc = "`read()` method returns [maxlen::R](maxlen::R) reader structure"]
impl crate::Readable for MAXLEN {}
#[doc = "`write(|w| ..)` method takes [maxlen::W](maxlen::W) writer structure"]
impl crate::Writable for MAXLEN {}
#[doc = "Size of allocated for TXD and RXD data storage buffer in Data RAM"]
pub mod maxlen;
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcid1_last](nfcid1_last) module"]
pub type NFCID1_LAST = crate::Reg<u32, _NFCID1_LAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCID1_LAST;
#[doc = "`read()` method returns [nfcid1_last::R](nfcid1_last::R) reader structure"]
impl crate::Readable for NFCID1_LAST {}
#[doc = "`write(|w| ..)` method takes [nfcid1_last::W](nfcid1_last::W) writer structure"]
impl crate::Writable for NFCID1_LAST {}
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
pub mod nfcid1_last;
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcid1_2nd_last](nfcid1_2nd_last) module"]
pub type NFCID1_2ND_LAST = crate::Reg<u32, _NFCID1_2ND_LAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCID1_2ND_LAST;
#[doc = "`read()` method returns [nfcid1_2nd_last::R](nfcid1_2nd_last::R) reader structure"]
impl crate::Readable for NFCID1_2ND_LAST {}
#[doc = "`write(|w| ..)` method takes [nfcid1_2nd_last::W](nfcid1_2nd_last::W) writer structure"]
impl crate::Writable for NFCID1_2ND_LAST {}
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
pub mod nfcid1_2nd_last;
#[doc = "Third last NFCID1 part (10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcid1_3rd_last](nfcid1_3rd_last) module"]
pub type NFCID1_3RD_LAST = crate::Reg<u32, _NFCID1_3RD_LAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCID1_3RD_LAST;
#[doc = "`read()` method returns [nfcid1_3rd_last::R](nfcid1_3rd_last::R) reader structure"]
impl crate::Readable for NFCID1_3RD_LAST {}
#[doc = "`write(|w| ..)` method takes [nfcid1_3rd_last::W](nfcid1_3rd_last::W) writer structure"]
impl crate::Writable for NFCID1_3RD_LAST {}
#[doc = "Third last NFCID1 part (10 bytes ID)"]
pub mod nfcid1_3rd_last;
#[doc = "NFC-A SENS_RES auto-response settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensres](sensres) module"]
pub type SENSRES = crate::Reg<u32, _SENSRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSRES;
#[doc = "`read()` method returns [sensres::R](sensres::R) reader structure"]
impl crate::Readable for SENSRES {}
#[doc = "`write(|w| ..)` method takes [sensres::W](sensres::W) writer structure"]
impl crate::Writable for SENSRES {}
#[doc = "NFC-A SENS_RES auto-response settings"]
pub mod sensres;
#[doc = "NFC-A SEL_RES auto-response settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selres](selres) module"]
pub type SELRES = crate::Reg<u32, _SELRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SELRES;
#[doc = "`read()` method returns [selres::R](selres::R) reader structure"]
impl crate::Readable for SELRES {}
#[doc = "`write(|w| ..)` method takes [selres::W](selres::W) writer structure"]
impl crate::Writable for SELRES {}
#[doc = "NFC-A SEL_RES auto-response settings"]
pub mod selres;
