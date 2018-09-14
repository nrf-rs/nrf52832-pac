#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDMCLKCTRL {
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
#[doc = "Possible values of the field `FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQR {
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    _1000K,
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz. Nominal clock for RATIO=Ratio64."]
    DEFAULT,
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    _1067K,
    #[doc = "PDM_CLK = 32 MHz / 26 = 1.231 MHz"]
    _1231K,
    #[doc = "PDM_CLK = 32 MHz / 25 = 1.280 MHz. Nominal clock for RATIO=Ratio80."]
    _1280K,
    #[doc = "PDM_CLK = 32 MHz / 24 = 1.333 MHz"]
    _1333K,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            FREQR::_1000K => 134217728,
            FREQR::DEFAULT => 138412032,
            FREQR::_1067K => 142606336,
            FREQR::_1231K => 159383552,
            FREQR::_1280K => 167772160,
            FREQR::_1333K => 176160768,
            FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> FREQR {
        match value {
            134217728 => FREQR::_1000K,
            138412032 => FREQR::DEFAULT,
            142606336 => FREQR::_1067K,
            159383552 => FREQR::_1231K,
            167772160 => FREQR::_1280K,
            176160768 => FREQR::_1333K,
            i => FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1000K`"]
    #[inline]
    pub fn is_1000k(&self) -> bool {
        *self == FREQR::_1000K
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == FREQR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_1067K`"]
    #[inline]
    pub fn is_1067k(&self) -> bool {
        *self == FREQR::_1067K
    }
    #[doc = "Checks if the value of the field is `_1231K`"]
    #[inline]
    pub fn is_1231k(&self) -> bool {
        *self == FREQR::_1231K
    }
    #[doc = "Checks if the value of the field is `_1280K`"]
    #[inline]
    pub fn is_1280k(&self) -> bool {
        *self == FREQR::_1280K
    }
    #[doc = "Checks if the value of the field is `_1333K`"]
    #[inline]
    pub fn is_1333k(&self) -> bool {
        *self == FREQR::_1333K
    }
}
#[doc = "Values that can be written to the field `FREQ`"]
pub enum FREQW {
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    _1000K,
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz. Nominal clock for RATIO=Ratio64."]
    DEFAULT,
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    _1067K,
    #[doc = "PDM_CLK = 32 MHz / 26 = 1.231 MHz"]
    _1231K,
    #[doc = "PDM_CLK = 32 MHz / 25 = 1.280 MHz. Nominal clock for RATIO=Ratio80."]
    _1280K,
    #[doc = "PDM_CLK = 32 MHz / 24 = 1.333 MHz"]
    _1333K,
}
impl FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            FREQW::_1000K => 134217728,
            FREQW::DEFAULT => 138412032,
            FREQW::_1067K => 142606336,
            FREQW::_1231K => 159383552,
            FREQW::_1280K => 167772160,
            FREQW::_1333K => 176160768,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    #[inline]
    pub fn _1000k(self) -> &'a mut W {
        self.variant(FREQW::_1000K)
    }
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz. Nominal clock for RATIO=Ratio64."]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(FREQW::DEFAULT)
    }
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    #[inline]
    pub fn _1067k(self) -> &'a mut W {
        self.variant(FREQW::_1067K)
    }
    #[doc = "PDM_CLK = 32 MHz / 26 = 1.231 MHz"]
    #[inline]
    pub fn _1231k(self) -> &'a mut W {
        self.variant(FREQW::_1231K)
    }
    #[doc = "PDM_CLK = 32 MHz / 25 = 1.280 MHz. Nominal clock for RATIO=Ratio80."]
    #[inline]
    pub fn _1280k(self) -> &'a mut W {
        self.variant(FREQW::_1280K)
    }
    #[doc = "PDM_CLK = 32 MHz / 24 = 1.333 MHz"]
    #[inline]
    pub fn _1333k(self) -> &'a mut W {
        self.variant(FREQW::_1333K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - PDM_CLK frequency"]
    #[inline]
    pub fn freq(&self) -> FREQR {
        FREQR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 138412032 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - PDM_CLK frequency"]
    #[inline]
    pub fn freq(&mut self) -> _FREQW {
        _FREQW { w: self }
    }
}
