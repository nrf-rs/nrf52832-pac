#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 252usize],
    #[doc = "0x104 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
    pub events_rxptrupd: EVENTS_RXPTRUPD,
    #[doc = "0x108 - I2S transfer stopped."]
    pub events_stopped: EVENTS_STOPPED,
    _reserved4: [u8; 8usize],
    #[doc = "0x114 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    pub events_txptrupd: EVENTS_TXPTRUPD,
    _reserved5: [u8; 488usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 500usize],
    #[doc = "0x500 - Enable I2S module."]
    pub enable: ENABLE,
    #[doc = "0x504 - Unspecified"]
    pub config: CONFIG,
    _reserved10: [u8; 12usize],
    #[doc = "0x538 - Unspecified"]
    pub rxd: RXD,
    _reserved11: [u8; 4usize],
    #[doc = "0x540 - Unspecified"]
    pub txd: TXD,
    _reserved12: [u8; 12usize],
    #[doc = "0x550 - Unspecified"]
    pub rxtxd: RXTXD,
    _reserved13: [u8; 12usize],
    #[doc = "0x560 - Unspecified"]
    pub psel: PSEL,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CONFIG {
    #[doc = "0x00 - I2S mode."]
    pub mode: self::config::MODE,
    #[doc = "0x04 - Reception (RX) enable."]
    pub rxen: self::config::RXEN,
    #[doc = "0x08 - Transmission (TX) enable."]
    pub txen: self::config::TXEN,
    #[doc = "0x0c - Master clock generator enable."]
    pub mcken: self::config::MCKEN,
    #[doc = "0x10 - Master clock generator frequency."]
    pub mckfreq: self::config::MCKFREQ,
    #[doc = "0x14 - MCK / LRCK ratio."]
    pub ratio: self::config::RATIO,
    #[doc = "0x18 - Sample width."]
    pub swidth: self::config::SWIDTH,
    #[doc = "0x1c - Alignment of sample within a frame."]
    pub align: self::config::ALIGN,
    #[doc = "0x20 - Frame format."]
    pub format: self::config::FORMAT,
    #[doc = "0x24 - Enable channels."]
    pub channels: self::config::CHANNELS,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod config;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Receive buffer RAM start address."]
    pub ptr: self::rxd::PTR,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Transmit buffer RAM start address."]
    pub ptr: self::txd::PTR,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXTXD {
    #[doc = "0x00 - Size of RXD and TXD buffers."]
    pub maxcnt: self::rxtxd::MAXCNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod rxtxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for MCK signal."]
    pub mck: self::psel::MCK,
    #[doc = "0x04 - Pin select for SCK signal."]
    pub sck: self::psel::SCK,
    #[doc = "0x08 - Pin select for LRCK signal."]
    pub lrck: self::psel::LRCK,
    #[doc = "0x0c - Pin select for SDIN signal."]
    pub sdin: self::psel::SDIN,
    #[doc = "0x10 - Pin select for SDOUT signal."]
    pub sdout: self::psel::SDOUT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
pub mod tasks_start;
#[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated."]
pub mod tasks_stop;
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
pub struct EVENTS_RXPTRUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
pub mod events_rxptrupd;
#[doc = "I2S transfer stopped."]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S transfer stopped."]
pub mod events_stopped;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub struct EVENTS_TXPTRUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub mod events_txptrupd;
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
#[doc = "Enable I2S module."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable I2S module."]
pub mod enable;
