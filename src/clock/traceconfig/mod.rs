#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRACECONFIG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TRACEPORTSPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACEPORTSPEEDR {
    #[doc = "32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    _32MHZ,
    #[doc = "16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    _16MHZ,
    #[doc = "8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    _8MHZ,
    #[doc = "4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    _4MHZ,
}
impl TRACEPORTSPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRACEPORTSPEEDR::_32MHZ => 0,
            TRACEPORTSPEEDR::_16MHZ => 1,
            TRACEPORTSPEEDR::_8MHZ => 2,
            TRACEPORTSPEEDR::_4MHZ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRACEPORTSPEEDR {
        match value {
            0 => TRACEPORTSPEEDR::_32MHZ,
            1 => TRACEPORTSPEEDR::_16MHZ,
            2 => TRACEPORTSPEEDR::_8MHZ,
            3 => TRACEPORTSPEEDR::_4MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline]
    pub fn is_32mhz(&self) -> bool {
        *self == TRACEPORTSPEEDR::_32MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline]
    pub fn is_16mhz(&self) -> bool {
        *self == TRACEPORTSPEEDR::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline]
    pub fn is_8mhz(&self) -> bool {
        *self == TRACEPORTSPEEDR::_8MHZ
    }
    #[doc = "Checks if the value of the field is `_4MHZ`"]
    #[inline]
    pub fn is_4mhz(&self) -> bool {
        *self == TRACEPORTSPEEDR::_4MHZ
    }
}
#[doc = "Possible values of the field `TRACEMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACEMUXR {
    #[doc = "GPIOs multiplexed onto all trace-pins"]
    GPIO,
    #[doc = "SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins"]
    SERIAL,
    #[doc = "TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14."]
    PARALLEL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRACEMUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRACEMUXR::GPIO => 0,
            TRACEMUXR::SERIAL => 1,
            TRACEMUXR::PARALLEL => 2,
            TRACEMUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRACEMUXR {
        match value {
            0 => TRACEMUXR::GPIO,
            1 => TRACEMUXR::SERIAL,
            2 => TRACEMUXR::PARALLEL,
            i => TRACEMUXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline]
    pub fn is_gpio(&self) -> bool {
        *self == TRACEMUXR::GPIO
    }
    #[doc = "Checks if the value of the field is `SERIAL`"]
    #[inline]
    pub fn is_serial(&self) -> bool {
        *self == TRACEMUXR::SERIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline]
    pub fn is_parallel(&self) -> bool {
        *self == TRACEMUXR::PARALLEL
    }
}
#[doc = "Values that can be written to the field `TRACEPORTSPEED`"]
pub enum TRACEPORTSPEEDW {
    #[doc = "32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    _32MHZ,
    #[doc = "16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    _16MHZ,
    #[doc = "8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    _8MHZ,
    #[doc = "4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    _4MHZ,
}
impl TRACEPORTSPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRACEPORTSPEEDW::_32MHZ => 0,
            TRACEPORTSPEEDW::_16MHZ => 1,
            TRACEPORTSPEEDW::_8MHZ => 2,
            TRACEPORTSPEEDW::_4MHZ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACEPORTSPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACEPORTSPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACEPORTSPEEDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    #[inline]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEEDW::_32MHZ)
    }
    #[doc = "16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    #[inline]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEEDW::_16MHZ)
    }
    #[doc = "8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    #[inline]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEEDW::_8MHZ)
    }
    #[doc = "4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    #[inline]
    pub fn _4mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEEDW::_4MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRACEMUX`"]
pub enum TRACEMUXW {
    #[doc = "GPIOs multiplexed onto all trace-pins"]
    GPIO,
    #[doc = "SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins"]
    SERIAL,
    #[doc = "TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14."]
    PARALLEL,
}
impl TRACEMUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRACEMUXW::GPIO => 0,
            TRACEMUXW::SERIAL => 1,
            TRACEMUXW::PARALLEL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACEMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACEMUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACEMUXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIOs multiplexed onto all trace-pins"]
    #[inline]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TRACEMUXW::GPIO)
    }
    #[doc = "SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins"]
    #[inline]
    pub fn serial(self) -> &'a mut W {
        self.variant(TRACEMUXW::SERIAL)
    }
    #[doc = "TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14."]
    #[inline]
    pub fn parallel(self) -> &'a mut W {
        self.variant(TRACEMUXW::PARALLEL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline]
    pub fn traceportspeed(&self) -> TRACEPORTSPEEDR {
        TRACEPORTSPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals."]
    #[inline]
    pub fn tracemux(&self) -> TRACEMUXR {
        TRACEMUXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline]
    pub fn traceportspeed(&mut self) -> _TRACEPORTSPEEDW {
        _TRACEPORTSPEEDW { w: self }
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals."]
    #[inline]
    pub fn tracemux(&mut self) -> _TRACEMUXW {
        _TRACEMUXW { w: self }
    }
}
