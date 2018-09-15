#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XTALFREQ {
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
#[doc = "Possible values of the field `XTALFREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALFREQR {
    #[doc = "16MHz xtal is used as source for the HFCLK oscillator."]
    _16MHZ,
    #[doc = "32MHz xtal is used as source for the HFCLK oscillator."]
    _32MHZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl XTALFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XTALFREQR::_16MHZ => 255,
            XTALFREQR::_32MHZ => 0,
            XTALFREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XTALFREQR {
        match value {
            255 => XTALFREQR::_16MHZ,
            0 => XTALFREQR::_32MHZ,
            i => XTALFREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline]
    pub fn is_16mhz(&self) -> bool {
        *self == XTALFREQR::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline]
    pub fn is_32mhz(&self) -> bool {
        *self == XTALFREQR::_32MHZ
    }
}
#[doc = "Values that can be written to the field `XTALFREQ`"]
pub enum XTALFREQW {
    #[doc = "16MHz xtal is used as source for the HFCLK oscillator."]
    _16MHZ,
    #[doc = "32MHz xtal is used as source for the HFCLK oscillator."]
    _32MHZ,
}
impl XTALFREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XTALFREQW::_16MHZ => 255,
            XTALFREQW::_32MHZ => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALFREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALFREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16MHz xtal is used as source for the HFCLK oscillator."]
    #[inline]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(XTALFREQW::_16MHZ)
    }
    #[doc = "32MHz xtal is used as source for the HFCLK oscillator."]
    #[inline]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(XTALFREQW::_32MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:7 - External Xtal frequency selection."]
    #[inline]
    pub fn xtalfreq(&self) -> XTALFREQR {
        XTALFREQR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - External Xtal frequency selection."]
    #[inline]
    pub fn xtalfreq(&mut self) -> _XTALFREQW {
        _XTALFREQW { w: self }
    }
}
