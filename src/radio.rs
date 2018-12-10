#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable RADIO in TX mode"]
    pub tasks_txen: TASKS_TXEN,
    #[doc = "0x04 - Enable RADIO in RX mode"]
    pub tasks_rxen: TASKS_RXEN,
    #[doc = "0x08 - Start RADIO"]
    pub tasks_start: TASKS_START,
    #[doc = "0x0c - Stop RADIO"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x10 - Disable RADIO"]
    pub tasks_disable: TASKS_DISABLE,
    #[doc = "0x14 - Start the RSSI and take one single sample of the receive signal strength."]
    pub tasks_rssistart: TASKS_RSSISTART,
    #[doc = "0x18 - Stop the RSSI measurement"]
    pub tasks_rssistop: TASKS_RSSISTOP,
    #[doc = "0x1c - Start the bit counter"]
    pub tasks_bcstart: TASKS_BCSTART,
    #[doc = "0x20 - Stop the bit counter"]
    pub tasks_bcstop: TASKS_BCSTOP,
    _reserved0: [u8; 220usize],
    #[doc = "0x100 - RADIO has ramped up and is ready to be started"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Address sent or received"]
    pub events_address: EVENTS_ADDRESS,
    #[doc = "0x108 - Packet payload sent or received"]
    pub events_payload: EVENTS_PAYLOAD,
    #[doc = "0x10c - Packet sent or received"]
    pub events_end: EVENTS_END,
    #[doc = "0x110 - RADIO has been disabled"]
    pub events_disabled: EVENTS_DISABLED,
    #[doc = "0x114 - A device address match occurred on the last received packet"]
    pub events_devmatch: EVENTS_DEVMATCH,
    #[doc = "0x118 - No device address match occurred on the last received packet"]
    pub events_devmiss: EVENTS_DEVMISS,
    #[doc = "0x11c - Sampling of receive signal strength complete."]
    pub events_rssiend: EVENTS_RSSIEND,
    _reserved1: [u8; 8usize],
    #[doc = "0x128 - Bit counter reached bit count value."]
    pub events_bcmatch: EVENTS_BCMATCH,
    _reserved2: [u8; 4usize],
    #[doc = "0x130 - Packet received with CRC ok"]
    pub events_crcok: EVENTS_CRCOK,
    #[doc = "0x134 - Packet received with CRC error"]
    pub events_crcerror: EVENTS_CRCERROR,
    _reserved3: [u8; 200usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved4: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 244usize],
    #[doc = "0x400 - CRC status"]
    pub crcstatus: CRCSTATUS,
    _reserved6: [u8; 4usize],
    #[doc = "0x408 - Received address"]
    pub rxmatch: RXMATCH,
    #[doc = "0x40c - CRC field of previously received packet"]
    pub rxcrc: RXCRC,
    #[doc = "0x410 - Device address match index"]
    pub dai: DAI,
    _reserved7: [u8; 240usize],
    #[doc = "0x504 - Packet pointer"]
    pub packetptr: PACKETPTR,
    #[doc = "0x508 - Frequency"]
    pub frequency: FREQUENCY,
    #[doc = "0x50c - Output power"]
    pub txpower: TXPOWER,
    #[doc = "0x510 - Data rate and modulation"]
    pub mode: MODE,
    #[doc = "0x514 - Packet configuration register 0"]
    pub pcnf0: PCNF0,
    #[doc = "0x518 - Packet configuration register 1"]
    pub pcnf1: PCNF1,
    #[doc = "0x51c - Base address 0"]
    pub base0: BASE0,
    #[doc = "0x520 - Base address 1"]
    pub base1: BASE1,
    #[doc = "0x524 - Prefixes bytes for logical addresses 0-3"]
    pub prefix0: PREFIX0,
    #[doc = "0x528 - Prefixes bytes for logical addresses 4-7"]
    pub prefix1: PREFIX1,
    #[doc = "0x52c - Transmit address select"]
    pub txaddress: TXADDRESS,
    #[doc = "0x530 - Receive address select"]
    pub rxaddresses: RXADDRESSES,
    #[doc = "0x534 - CRC configuration"]
    pub crccnf: CRCCNF,
    #[doc = "0x538 - CRC polynomial"]
    pub crcpoly: CRCPOLY,
    #[doc = "0x53c - CRC initial value"]
    pub crcinit: CRCINIT,
    #[doc = "0x540 - Unspecified"]
    pub unused0: UNUSED0,
    #[doc = "0x544 - Inter Frame Spacing in us"]
    pub tifs: TIFS,
    #[doc = "0x548 - RSSI sample"]
    pub rssisample: RSSISAMPLE,
    _reserved8: [u8; 4usize],
    #[doc = "0x550 - Current radio state"]
    pub state: STATE,
    #[doc = "0x554 - Data whitening initial value"]
    pub datawhiteiv: DATAWHITEIV,
    _reserved9: [u8; 8usize],
    #[doc = "0x560 - Bit counter compare"]
    pub bcc: BCC,
    _reserved10: [u8; 156usize],
    #[doc = "0x600 - Description collection\\[0\\]: Device address base segment 0"]
    pub dab: [DAB; 8],
    #[doc = "0x620 - Description collection\\[0\\]: Device address prefix 0"]
    pub dap: [DAP; 8],
    #[doc = "0x640 - Device address match configuration"]
    pub dacnf: DACNF,
    _reserved11: [u8; 12usize],
    #[doc = "0x650 - Radio mode configuration register 0"]
    pub modecnf0: MODECNF0,
    _reserved12: [u8; 2472usize],
    #[doc = "0xffc - Peripheral power control"]
    pub power: POWER,
}
#[doc = "Enable RADIO in TX mode"]
pub struct TASKS_TXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable RADIO in TX mode"]
pub mod tasks_txen;
#[doc = "Enable RADIO in RX mode"]
pub struct TASKS_RXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable RADIO in RX mode"]
pub mod tasks_rxen;
#[doc = "Start RADIO"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start RADIO"]
pub mod tasks_start;
#[doc = "Stop RADIO"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop RADIO"]
pub mod tasks_stop;
#[doc = "Disable RADIO"]
pub struct TASKS_DISABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable RADIO"]
pub mod tasks_disable;
#[doc = "Start the RSSI and take one single sample of the receive signal strength."]
pub struct TASKS_RSSISTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the RSSI and take one single sample of the receive signal strength."]
pub mod tasks_rssistart;
#[doc = "Stop the RSSI measurement"]
pub struct TASKS_RSSISTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop the RSSI measurement"]
pub mod tasks_rssistop;
#[doc = "Start the bit counter"]
pub struct TASKS_BCSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the bit counter"]
pub mod tasks_bcstart;
#[doc = "Stop the bit counter"]
pub struct TASKS_BCSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop the bit counter"]
pub mod tasks_bcstop;
#[doc = "RADIO has ramped up and is ready to be started"]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RADIO has ramped up and is ready to be started"]
pub mod events_ready;
#[doc = "Address sent or received"]
pub struct EVENTS_ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address sent or received"]
pub mod events_address;
#[doc = "Packet payload sent or received"]
pub struct EVENTS_PAYLOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet payload sent or received"]
pub mod events_payload;
#[doc = "Packet sent or received"]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet sent or received"]
pub mod events_end;
#[doc = "RADIO has been disabled"]
pub struct EVENTS_DISABLED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RADIO has been disabled"]
pub mod events_disabled;
#[doc = "A device address match occurred on the last received packet"]
pub struct EVENTS_DEVMATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A device address match occurred on the last received packet"]
pub mod events_devmatch;
#[doc = "No device address match occurred on the last received packet"]
pub struct EVENTS_DEVMISS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "No device address match occurred on the last received packet"]
pub mod events_devmiss;
#[doc = "Sampling of receive signal strength complete."]
pub struct EVENTS_RSSIEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sampling of receive signal strength complete."]
pub mod events_rssiend;
#[doc = "Bit counter reached bit count value."]
pub struct EVENTS_BCMATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit counter reached bit count value."]
pub mod events_bcmatch;
#[doc = "Packet received with CRC ok"]
pub struct EVENTS_CRCOK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet received with CRC ok"]
pub mod events_crcok;
#[doc = "Packet received with CRC error"]
pub struct EVENTS_CRCERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet received with CRC error"]
pub mod events_crcerror;
#[doc = "Shortcut register"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcut register"]
pub mod shorts;
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
#[doc = "CRC status"]
pub struct CRCSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC status"]
pub mod crcstatus;
#[doc = "Received address"]
pub struct RXMATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received address"]
pub mod rxmatch;
#[doc = "CRC field of previously received packet"]
pub struct RXCRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC field of previously received packet"]
pub mod rxcrc;
#[doc = "Device address match index"]
pub struct DAI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address match index"]
pub mod dai;
#[doc = "Packet pointer"]
pub struct PACKETPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet pointer"]
pub mod packetptr;
#[doc = "Frequency"]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency"]
pub mod frequency;
#[doc = "Output power"]
pub struct TXPOWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output power"]
pub mod txpower;
#[doc = "Data rate and modulation"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data rate and modulation"]
pub mod mode;
#[doc = "Packet configuration register 0"]
pub struct PCNF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet configuration register 0"]
pub mod pcnf0;
#[doc = "Packet configuration register 1"]
pub struct PCNF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet configuration register 1"]
pub mod pcnf1;
#[doc = "Base address 0"]
pub struct BASE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base address 0"]
pub mod base0;
#[doc = "Base address 1"]
pub struct BASE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base address 1"]
pub mod base1;
#[doc = "Prefixes bytes for logical addresses 0-3"]
pub struct PREFIX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prefixes bytes for logical addresses 0-3"]
pub mod prefix0;
#[doc = "Prefixes bytes for logical addresses 4-7"]
pub struct PREFIX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prefixes bytes for logical addresses 4-7"]
pub mod prefix1;
#[doc = "Transmit address select"]
pub struct TXADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit address select"]
pub mod txaddress;
#[doc = "Receive address select"]
pub struct RXADDRESSES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive address select"]
pub mod rxaddresses;
#[doc = "CRC configuration"]
pub struct CRCCNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC configuration"]
pub mod crccnf;
#[doc = "CRC polynomial"]
pub struct CRCPOLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC polynomial"]
pub mod crcpoly;
#[doc = "CRC initial value"]
pub struct CRCINIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC initial value"]
pub mod crcinit;
#[doc = "Unspecified"]
pub struct UNUSED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "Inter Frame Spacing in us"]
pub struct TIFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter Frame Spacing in us"]
pub mod tifs;
#[doc = "RSSI sample"]
pub struct RSSISAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSSI sample"]
pub mod rssisample;
#[doc = "Current radio state"]
pub struct STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current radio state"]
pub mod state;
#[doc = "Data whitening initial value"]
pub struct DATAWHITEIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data whitening initial value"]
pub mod datawhiteiv;
#[doc = "Bit counter compare"]
pub struct BCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit counter compare"]
pub mod bcc;
#[doc = "Description collection\\[0\\]: Device address base segment 0"]
pub struct DAB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection\\[0\\]: Device address base segment 0"]
pub mod dab;
#[doc = "Description collection\\[0\\]: Device address prefix 0"]
pub struct DAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection\\[0\\]: Device address prefix 0"]
pub mod dap;
#[doc = "Device address match configuration"]
pub struct DACNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address match configuration"]
pub mod dacnf;
#[doc = "Radio mode configuration register 0"]
pub struct MODECNF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio mode configuration register 0"]
pub mod modecnf0;
#[doc = "Peripheral power control"]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control"]
pub mod power;
