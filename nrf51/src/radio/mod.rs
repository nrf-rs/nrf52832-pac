#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable radio in TX mode."]
    pub tasks_txen: TASKS_TXEN,
    #[doc = "0x04 - Enable radio in RX mode."]
    pub tasks_rxen: TASKS_RXEN,
    #[doc = "0x08 - Start radio."]
    pub tasks_start: TASKS_START,
    #[doc = "0x0c - Stop radio."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x10 - Disable radio."]
    pub tasks_disable: TASKS_DISABLE,
    #[doc = "0x14 - Start the RSSI and take one sample of the receive signal strength."]
    pub tasks_rssistart: TASKS_RSSISTART,
    #[doc = "0x18 - Stop the RSSI measurement."]
    pub tasks_rssistop: TASKS_RSSISTOP,
    #[doc = "0x1c - Start the bit counter."]
    pub tasks_bcstart: TASKS_BCSTART,
    #[doc = "0x20 - Stop the bit counter."]
    pub tasks_bcstop: TASKS_BCSTOP,
    _reserved9: [u8; 220usize],
    #[doc = "0x100 - Ready event."]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Address event."]
    pub events_address: EVENTS_ADDRESS,
    #[doc = "0x108 - Payload event."]
    pub events_payload: EVENTS_PAYLOAD,
    #[doc = "0x10c - End event."]
    pub events_end: EVENTS_END,
    #[doc = "0x110 - Disable event."]
    pub events_disabled: EVENTS_DISABLED,
    #[doc = "0x114 - A device address match occurred on the last received packet."]
    pub events_devmatch: EVENTS_DEVMATCH,
    #[doc = "0x118 - No device address match occurred on the last received packet."]
    pub events_devmiss: EVENTS_DEVMISS,
    #[doc = "0x11c - Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
    pub events_rssiend: EVENTS_RSSIEND,
    _reserved17: [u8; 8usize],
    #[doc = "0x128 - Bit counter reached bit count value specified in BCC register."]
    pub events_bcmatch: EVENTS_BCMATCH,
    _reserved18: [u8; 212usize],
    #[doc = "0x200 - Shortcuts for the radio."]
    pub shorts: SHORTS,
    _reserved19: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved21: [u8; 244usize],
    #[doc = "0x400 - CRC status of received packet."]
    pub crcstatus: CRCSTATUS,
    _reserved22: [u8; 4usize],
    #[doc = "0x408 - Received address."]
    pub rxmatch: RXMATCH,
    #[doc = "0x40c - Received CRC."]
    pub rxcrc: RXCRC,
    #[doc = "0x410 - Device address match index."]
    pub dai: DAI,
    _reserved25: [u8; 240usize],
    #[doc = "0x504 - Packet pointer. Decision point: START task."]
    pub packetptr: PACKETPTR,
    #[doc = "0x508 - Frequency."]
    pub frequency: FREQUENCY,
    #[doc = "0x50c - Output power."]
    pub txpower: TXPOWER,
    #[doc = "0x510 - Data rate and modulation."]
    pub mode: MODE,
    #[doc = "0x514 - Packet configuration 0."]
    pub pcnf0: PCNF0,
    #[doc = "0x518 - Packet configuration 1."]
    pub pcnf1: PCNF1,
    #[doc = "0x51c - Radio base address 0. Decision point: START task."]
    pub base0: BASE0,
    #[doc = "0x520 - Radio base address 1. Decision point: START task."]
    pub base1: BASE1,
    #[doc = "0x524 - Prefixes bytes for logical addresses 0 to 3."]
    pub prefix0: PREFIX0,
    #[doc = "0x528 - Prefixes bytes for logical addresses 4 to 7."]
    pub prefix1: PREFIX1,
    #[doc = "0x52c - Transmit address select."]
    pub txaddress: TXADDRESS,
    #[doc = "0x530 - Receive address select."]
    pub rxaddresses: RXADDRESSES,
    #[doc = "0x534 - CRC configuration."]
    pub crccnf: CRCCNF,
    #[doc = "0x538 - CRC polynomial."]
    pub crcpoly: CRCPOLY,
    #[doc = "0x53c - CRC initial value."]
    pub crcinit: CRCINIT,
    #[doc = "0x540 - Test features enable register."]
    pub test: TEST,
    #[doc = "0x544 - Inter Frame Spacing in microseconds."]
    pub tifs: TIFS,
    #[doc = "0x548 - RSSI sample."]
    pub rssisample: RSSISAMPLE,
    _reserved43: [u8; 4usize],
    #[doc = "0x550 - Current radio state."]
    pub state: STATE,
    #[doc = "0x554 - Data whitening initial value."]
    pub datawhiteiv: DATAWHITEIV,
    _reserved45: [u8; 8usize],
    #[doc = "0x560 - Bit counter compare."]
    pub bcc: BCC,
    _reserved46: [u8; 156usize],
    #[doc = "0x600 - Device address base segment."]
    pub dab: [DAB; 8],
    #[doc = "0x620 - Device address prefix."]
    pub dap: [DAP; 8],
    #[doc = "0x640 - Device address match configuration."]
    pub dacnf: DACNF,
    _reserved49: [u8; 224usize],
    #[doc = "0x724 - Trim value override register 0."]
    pub override0: OVERRIDE0,
    #[doc = "0x728 - Trim value override register 1."]
    pub override1: OVERRIDE1,
    #[doc = "0x72c - Trim value override register 2."]
    pub override2: OVERRIDE2,
    #[doc = "0x730 - Trim value override register 3."]
    pub override3: OVERRIDE3,
    #[doc = "0x734 - Trim value override register 4."]
    pub override4: OVERRIDE4,
    _reserved54: [u8; 2244usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Enable radio in TX mode."]
pub struct TASKS_TXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable radio in TX mode."]
pub mod tasks_txen;
#[doc = "Enable radio in RX mode."]
pub struct TASKS_RXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable radio in RX mode."]
pub mod tasks_rxen;
#[doc = "Start radio."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start radio."]
pub mod tasks_start;
#[doc = "Stop radio."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop radio."]
pub mod tasks_stop;
#[doc = "Disable radio."]
pub struct TASKS_DISABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable radio."]
pub mod tasks_disable;
#[doc = "Start the RSSI and take one sample of the receive signal strength."]
pub struct TASKS_RSSISTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the RSSI and take one sample of the receive signal strength."]
pub mod tasks_rssistart;
#[doc = "Stop the RSSI measurement."]
pub struct TASKS_RSSISTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop the RSSI measurement."]
pub mod tasks_rssistop;
#[doc = "Start the bit counter."]
pub struct TASKS_BCSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the bit counter."]
pub mod tasks_bcstart;
#[doc = "Stop the bit counter."]
pub struct TASKS_BCSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop the bit counter."]
pub mod tasks_bcstop;
#[doc = "Ready event."]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready event."]
pub mod events_ready;
#[doc = "Address event."]
pub struct EVENTS_ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address event."]
pub mod events_address;
#[doc = "Payload event."]
pub struct EVENTS_PAYLOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Payload event."]
pub mod events_payload;
#[doc = "End event."]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End event."]
pub mod events_end;
#[doc = "Disable event."]
pub struct EVENTS_DISABLED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable event."]
pub mod events_disabled;
#[doc = "A device address match occurred on the last received packet."]
pub struct EVENTS_DEVMATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A device address match occurred on the last received packet."]
pub mod events_devmatch;
#[doc = "No device address match occurred on the last received packet."]
pub struct EVENTS_DEVMISS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "No device address match occurred on the last received packet."]
pub mod events_devmiss;
#[doc = "Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
pub struct EVENTS_RSSIEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
pub mod events_rssiend;
#[doc = "Bit counter reached bit count value specified in BCC register."]
pub struct EVENTS_BCMATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit counter reached bit count value specified in BCC register."]
pub mod events_bcmatch;
#[doc = "Shortcuts for the radio."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts for the radio."]
pub mod shorts;
#[doc = "Interrupt enable set register."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "Interrupt enable clear register."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "CRC status of received packet."]
pub struct CRCSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC status of received packet."]
pub mod crcstatus;
#[doc = "Received address."]
pub struct RXMATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received address."]
pub mod rxmatch;
#[doc = "Received CRC."]
pub struct RXCRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received CRC."]
pub mod rxcrc;
#[doc = "Device address match index."]
pub struct DAI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address match index."]
pub mod dai;
#[doc = "Packet pointer. Decision point: START task."]
pub struct PACKETPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet pointer. Decision point: START task."]
pub mod packetptr;
#[doc = "Frequency."]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency."]
pub mod frequency;
#[doc = "Output power."]
pub struct TXPOWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output power."]
pub mod txpower;
#[doc = "Data rate and modulation."]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data rate and modulation."]
pub mod mode;
#[doc = "Packet configuration 0."]
pub struct PCNF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet configuration 0."]
pub mod pcnf0;
#[doc = "Packet configuration 1."]
pub struct PCNF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Packet configuration 1."]
pub mod pcnf1;
#[doc = "Radio base address 0. Decision point: START task."]
pub struct BASE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio base address 0. Decision point: START task."]
pub mod base0;
#[doc = "Radio base address 1. Decision point: START task."]
pub struct BASE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio base address 1. Decision point: START task."]
pub mod base1;
#[doc = "Prefixes bytes for logical addresses 0 to 3."]
pub struct PREFIX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prefixes bytes for logical addresses 0 to 3."]
pub mod prefix0;
#[doc = "Prefixes bytes for logical addresses 4 to 7."]
pub struct PREFIX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prefixes bytes for logical addresses 4 to 7."]
pub mod prefix1;
#[doc = "Transmit address select."]
pub struct TXADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit address select."]
pub mod txaddress;
#[doc = "Receive address select."]
pub struct RXADDRESSES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive address select."]
pub mod rxaddresses;
#[doc = "CRC configuration."]
pub struct CRCCNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC configuration."]
pub mod crccnf;
#[doc = "CRC polynomial."]
pub struct CRCPOLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC polynomial."]
pub mod crcpoly;
#[doc = "CRC initial value."]
pub struct CRCINIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC initial value."]
pub mod crcinit;
#[doc = "Test features enable register."]
pub struct TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test features enable register."]
pub mod test;
#[doc = "Inter Frame Spacing in microseconds."]
pub struct TIFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter Frame Spacing in microseconds."]
pub mod tifs;
#[doc = "RSSI sample."]
pub struct RSSISAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSSI sample."]
pub mod rssisample;
#[doc = "Current radio state."]
pub struct STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current radio state."]
pub mod state;
#[doc = "Data whitening initial value."]
pub struct DATAWHITEIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data whitening initial value."]
pub mod datawhiteiv;
#[doc = "Bit counter compare."]
pub struct BCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit counter compare."]
pub mod bcc;
#[doc = "Device address base segment."]
pub struct DAB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address base segment."]
pub mod dab;
#[doc = "Device address prefix."]
pub struct DAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address prefix."]
pub mod dap;
#[doc = "Device address match configuration."]
pub struct DACNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address match configuration."]
pub mod dacnf;
#[doc = "Trim value override register 0."]
pub struct OVERRIDE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trim value override register 0."]
pub mod override0;
#[doc = "Trim value override register 1."]
pub struct OVERRIDE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trim value override register 1."]
pub mod override1;
#[doc = "Trim value override register 2."]
pub struct OVERRIDE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trim value override register 2."]
pub mod override2;
#[doc = "Trim value override register 3."]
pub struct OVERRIDE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trim value override register 3."]
pub mod override3;
#[doc = "Trim value override register 4."]
pub struct OVERRIDE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trim value override register 4."]
pub mod override4;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
