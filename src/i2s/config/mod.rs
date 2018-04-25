#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "I2S mode."]
pub struct MODE {
    register: VolatileCell<u32>,
}
#[doc = "I2S mode."]
pub mod mode;
#[doc = "Reception (RX) enable."]
pub struct RXEN {
    register: VolatileCell<u32>,
}
#[doc = "Reception (RX) enable."]
pub mod rxen;
#[doc = "Transmission (TX) enable."]
pub struct TXEN {
    register: VolatileCell<u32>,
}
#[doc = "Transmission (TX) enable."]
pub mod txen;
#[doc = "Master clock generator enable."]
pub struct MCKEN {
    register: VolatileCell<u32>,
}
#[doc = "Master clock generator enable."]
pub mod mcken;
#[doc = "Master clock generator frequency."]
pub struct MCKFREQ {
    register: VolatileCell<u32>,
}
#[doc = "Master clock generator frequency."]
pub mod mckfreq;
#[doc = "MCK / LRCK ratio."]
pub struct RATIO {
    register: VolatileCell<u32>,
}
#[doc = "MCK / LRCK ratio."]
pub mod ratio;
#[doc = "Sample width."]
pub struct SWIDTH {
    register: VolatileCell<u32>,
}
#[doc = "Sample width."]
pub mod swidth;
#[doc = "Alignment of sample within a frame."]
pub struct ALIGN {
    register: VolatileCell<u32>,
}
#[doc = "Alignment of sample within a frame."]
pub mod align;
#[doc = "Frame format."]
pub struct FORMAT {
    register: VolatileCell<u32>,
}
#[doc = "Frame format."]
pub mod format;
#[doc = "Enable channels."]
pub struct CHANNELS {
    register: VolatileCell<u32>,
}
#[doc = "Enable channels."]
pub mod channels;
