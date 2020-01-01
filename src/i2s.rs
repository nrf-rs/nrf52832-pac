#[doc = r"Register block"]
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
#[doc = r"Register block"]
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
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod config;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Receive buffer RAM start address."]
    pub ptr: self::rxd::PTR,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Transmit buffer RAM start address."]
    pub ptr: self::txd::PTR,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXTXD {
    #[doc = "0x00 - Size of RXD and TXD buffers."]
    pub maxcnt: self::rxtxd::MAXCNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod rxtxd;
#[doc = r"Register block"]
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
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
pub mod tasks_start;
#[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated."]
pub mod tasks_stop;
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxptrupd](events_rxptrupd) module"]
pub type EVENTS_RXPTRUPD = crate::Reg<u32, _EVENTS_RXPTRUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXPTRUPD;
#[doc = "`read()` method returns [events_rxptrupd::R](events_rxptrupd::R) reader structure"]
impl crate::Readable for EVENTS_RXPTRUPD {}
#[doc = "`write(|w| ..)` method takes [events_rxptrupd::W](events_rxptrupd::W) writer structure"]
impl crate::Writable for EVENTS_RXPTRUPD {}
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
pub mod events_rxptrupd;
#[doc = "I2S transfer stopped.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "I2S transfer stopped."]
pub mod events_stopped;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txptrupd](events_txptrupd) module"]
pub type EVENTS_TXPTRUPD = crate::Reg<u32, _EVENTS_TXPTRUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXPTRUPD;
#[doc = "`read()` method returns [events_txptrupd::R](events_txptrupd::R) reader structure"]
impl crate::Readable for EVENTS_TXPTRUPD {}
#[doc = "`write(|w| ..)` method takes [events_txptrupd::W](events_txptrupd::W) writer structure"]
impl crate::Writable for EVENTS_TXPTRUPD {}
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub mod events_txptrupd;
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
#[doc = "Enable I2S module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable I2S module."]
pub mod enable;
