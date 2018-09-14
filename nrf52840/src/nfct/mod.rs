#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    pub tasks_activate: TASKS_ACTIVATE,
    #[doc = "0x04 - Disable NFCT peripheral"]
    pub tasks_disable: TASKS_DISABLE,
    #[doc = "0x08 - Enable NFC sense field mode, change state to sense mode"]
    pub tasks_sense: TASKS_SENSE,
    #[doc = "0x0c - Start transmission of an outgoing frame, change state to transmit"]
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
    #[doc = "0x100 - The NFCT peripheral is ready to receive and send frames"]
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
    #[doc = "0x118 - Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
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
    #[doc = "0x148 - NFC auto collision resolution error reported."]
    pub events_collision: EVENTS_COLLISION,
    #[doc = "0x14c - NFC auto collision resolution successfully completed"]
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
    #[doc = "0x410 - NfcTag state register"]
    pub nfctagstate: NFCTAGSTATE,
    _reserved29: [u8; 12usize],
    #[doc = "0x420 - Sleep state during automatic collision resolution"]
    pub sleepstate: SLEEPSTATE,
    _reserved30: [u8; 24usize],
    #[doc = "0x43c - Indicates the presence or not of a valid field"]
    pub fieldpresent: FIELDPRESENT,
    _reserved31: [u8; 196usize],
    #[doc = "0x504 - Minimum frame delay"]
    pub framedelaymin: FRAMEDELAYMIN,
    #[doc = "0x508 - Maximum frame delay"]
    pub framedelaymax: FRAMEDELAYMAX,
    #[doc = "0x50c - Configuration register for the Frame Delay Timer"]
    pub framedelaymode: FRAMEDELAYMODE,
    #[doc = "0x510 - Packet pointer for TXD and RXD data storage in Data RAM"]
    pub packetptr: PACKETPTR,
    #[doc = "0x514 - Size of the RAM buffer allocated to TXD and RXD data storage each"]
    pub maxlen: MAXLEN,
    #[doc = "0x518 - Unspecified"]
    pub txd: TXD,
    #[doc = "0x520 - Unspecified"]
    pub rxd: RXD,
    _reserved38: [u8; 104usize],
    #[doc = "0x590 - Last NFCID1 part (4, 7 or 10 bytes ID)"]
    pub nfcid1_last: NFCID1_LAST,
    #[doc = "0x594 - Second last NFCID1 part (7 or 10 bytes ID)"]
    pub nfcid1_2nd_last: NFCID1_2ND_LAST,
    #[doc = "0x598 - Third last NFCID1 part (10 bytes ID)"]
    pub nfcid1_3rd_last: NFCID1_3RD_LAST,
    #[doc = "0x59c - Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is enabled."]
    pub autocolresconfig: AUTOCOLRESCONFIG,
    #[doc = "0x5a0 - NFC-A SENS_RES auto-response settings"]
    pub sensres: SENSRES,
    #[doc = "0x5a4 - NFC-A SEL_RES auto-response settings"]
    pub selres: SELRES,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct FRAMESTATUS {
    #[doc = "0x00 - Result of last incoming frame"]
    pub rx: self::framestatus::RX,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod framestatus;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Configuration of outgoing frames"]
    pub frameconfig: self::txd::FRAMECONFIG,
    #[doc = "0x04 - Size of outgoing frame"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Configuration of incoming frames"]
    pub frameconfig: self::rxd::FRAMECONFIG,
    #[doc = "0x04 - Size of last incoming frame"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
pub struct TASKS_ACTIVATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
pub mod tasks_activate;
#[doc = "Disable NFCT peripheral"]
pub struct TASKS_DISABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable NFCT peripheral"]
pub mod tasks_disable;
#[doc = "Enable NFC sense field mode, change state to sense mode"]
pub struct TASKS_SENSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable NFC sense field mode, change state to sense mode"]
pub mod tasks_sense;
#[doc = "Start transmission of an outgoing frame, change state to transmit"]
pub struct TASKS_STARTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start transmission of an outgoing frame, change state to transmit"]
pub mod tasks_starttx;
#[doc = "Initializes the EasyDMA for receive."]
pub struct TASKS_ENABLERXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initializes the EasyDMA for receive."]
pub mod tasks_enablerxdata;
#[doc = "Force state machine to IDLE state"]
pub struct TASKS_GOIDLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Force state machine to IDLE state"]
pub mod tasks_goidle;
#[doc = "Force state machine to SLEEP_A state"]
pub struct TASKS_GOSLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Force state machine to SLEEP_A state"]
pub mod tasks_gosleep;
#[doc = "The NFCT peripheral is ready to receive and send frames"]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The NFCT peripheral is ready to receive and send frames"]
pub mod events_ready;
#[doc = "Remote NFC field detected"]
pub struct EVENTS_FIELDDETECTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Remote NFC field detected"]
pub mod events_fielddetected;
#[doc = "Remote NFC field lost"]
pub struct EVENTS_FIELDLOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Remote NFC field lost"]
pub mod events_fieldlost;
#[doc = "Marks the start of the first symbol of a transmitted frame"]
pub struct EVENTS_TXFRAMESTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Marks the start of the first symbol of a transmitted frame"]
pub mod events_txframestart;
#[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
pub struct EVENTS_TXFRAMEEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
pub mod events_txframeend;
#[doc = "Marks the end of the first symbol of a received frame"]
pub struct EVENTS_RXFRAMESTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Marks the end of the first symbol of a received frame"]
pub mod events_rxframestart;
#[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
pub struct EVENTS_RXFRAMEEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
pub mod events_rxframeend;
#[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
pub mod events_error;
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
pub struct EVENTS_RXERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
pub mod events_rxerror;
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
pub struct EVENTS_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
pub mod events_endrx;
#[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
pub struct EVENTS_ENDTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
pub mod events_endtx;
#[doc = "Auto collision resolution process has started"]
pub struct EVENTS_AUTOCOLRESSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auto collision resolution process has started"]
pub mod events_autocolresstarted;
#[doc = "NFC auto collision resolution error reported."]
pub struct EVENTS_COLLISION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NFC auto collision resolution error reported."]
pub mod events_collision;
#[doc = "NFC auto collision resolution successfully completed"]
pub struct EVENTS_SELECTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NFC auto collision resolution successfully completed"]
pub mod events_selected;
#[doc = "EasyDMA is ready to receive or send frames."]
pub struct EVENTS_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EasyDMA is ready to receive or send frames."]
pub mod events_started;
#[doc = "Shortcut register"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "Enable or disable interrupt"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "NFC Error Status register"]
pub struct ERRORSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NFC Error Status register"]
pub mod errorstatus;
#[doc = "NfcTag state register"]
pub struct NFCTAGSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NfcTag state register"]
pub mod nfctagstate;
#[doc = "Sleep state during automatic collision resolution"]
pub struct SLEEPSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep state during automatic collision resolution"]
pub mod sleepstate;
#[doc = "Indicates the presence or not of a valid field"]
pub struct FIELDPRESENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indicates the presence or not of a valid field"]
pub mod fieldpresent;
#[doc = "Minimum frame delay"]
pub struct FRAMEDELAYMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Minimum frame delay"]
pub mod framedelaymin;
#[doc = "Maximum frame delay"]
pub struct FRAMEDELAYMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum frame delay"]
pub mod framedelaymax;
#[doc = "Configuration register for the Frame Delay Timer"]
pub struct FRAMEDELAYMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register for the Frame Delay Timer"]
pub mod framedelaymode;
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
pub struct PACKETPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
pub mod packetptr;
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub struct MAXLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub mod maxlen;
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
pub struct NFCID1_LAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
pub mod nfcid1_last;
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
pub struct NFCID1_2ND_LAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
pub mod nfcid1_2nd_last;
#[doc = "Third last NFCID1 part (10 bytes ID)"]
pub struct NFCID1_3RD_LAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Third last NFCID1 part (10 bytes ID)"]
pub mod nfcid1_3rd_last;
#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is enabled."]
pub struct AUTOCOLRESCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is enabled."]
pub mod autocolresconfig;
#[doc = "NFC-A SENS_RES auto-response settings"]
pub struct SENSRES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NFC-A SENS_RES auto-response settings"]
pub mod sensres;
#[doc = "NFC-A SEL_RES auto-response settings"]
pub struct SELRES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NFC-A SEL_RES auto-response settings"]
pub mod selres;
