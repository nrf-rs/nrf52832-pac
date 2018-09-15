#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAMPLEPER {
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
#[doc = "Possible values of the field `SAMPLEPER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLEPERR {
    #[doc = "128us sample period."]
    _128US,
    #[doc = "256us sample period."]
    _256US,
    #[doc = "512us sample period."]
    _512US,
    #[doc = "1024us sample period."]
    _1024US,
    #[doc = "2048us sample period."]
    _2048US,
    #[doc = "4096us sample period."]
    _4096US,
    #[doc = "8192us sample period."]
    _8192US,
    #[doc = "16384us sample period."]
    _16384US,
}
impl SAMPLEPERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAMPLEPERR::_128US => 0,
            SAMPLEPERR::_256US => 1,
            SAMPLEPERR::_512US => 2,
            SAMPLEPERR::_1024US => 3,
            SAMPLEPERR::_2048US => 4,
            SAMPLEPERR::_4096US => 5,
            SAMPLEPERR::_8192US => 6,
            SAMPLEPERR::_16384US => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAMPLEPERR {
        match value {
            0 => SAMPLEPERR::_128US,
            1 => SAMPLEPERR::_256US,
            2 => SAMPLEPERR::_512US,
            3 => SAMPLEPERR::_1024US,
            4 => SAMPLEPERR::_2048US,
            5 => SAMPLEPERR::_4096US,
            6 => SAMPLEPERR::_8192US,
            7 => SAMPLEPERR::_16384US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_128US`"]
    #[inline]
    pub fn is_128us(&self) -> bool {
        *self == SAMPLEPERR::_128US
    }
    #[doc = "Checks if the value of the field is `_256US`"]
    #[inline]
    pub fn is_256us(&self) -> bool {
        *self == SAMPLEPERR::_256US
    }
    #[doc = "Checks if the value of the field is `_512US`"]
    #[inline]
    pub fn is_512us(&self) -> bool {
        *self == SAMPLEPERR::_512US
    }
    #[doc = "Checks if the value of the field is `_1024US`"]
    #[inline]
    pub fn is_1024us(&self) -> bool {
        *self == SAMPLEPERR::_1024US
    }
    #[doc = "Checks if the value of the field is `_2048US`"]
    #[inline]
    pub fn is_2048us(&self) -> bool {
        *self == SAMPLEPERR::_2048US
    }
    #[doc = "Checks if the value of the field is `_4096US`"]
    #[inline]
    pub fn is_4096us(&self) -> bool {
        *self == SAMPLEPERR::_4096US
    }
    #[doc = "Checks if the value of the field is `_8192US`"]
    #[inline]
    pub fn is_8192us(&self) -> bool {
        *self == SAMPLEPERR::_8192US
    }
    #[doc = "Checks if the value of the field is `_16384US`"]
    #[inline]
    pub fn is_16384us(&self) -> bool {
        *self == SAMPLEPERR::_16384US
    }
}
#[doc = "Values that can be written to the field `SAMPLEPER`"]
pub enum SAMPLEPERW {
    #[doc = "128us sample period."]
    _128US,
    #[doc = "256us sample period."]
    _256US,
    #[doc = "512us sample period."]
    _512US,
    #[doc = "1024us sample period."]
    _1024US,
    #[doc = "2048us sample period."]
    _2048US,
    #[doc = "4096us sample period."]
    _4096US,
    #[doc = "8192us sample period."]
    _8192US,
    #[doc = "16384us sample period."]
    _16384US,
}
impl SAMPLEPERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMPLEPERW::_128US => 0,
            SAMPLEPERW::_256US => 1,
            SAMPLEPERW::_512US => 2,
            SAMPLEPERW::_1024US => 3,
            SAMPLEPERW::_2048US => 4,
            SAMPLEPERW::_4096US => 5,
            SAMPLEPERW::_8192US => 6,
            SAMPLEPERW::_16384US => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLEPERW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLEPERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLEPERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "128us sample period."]
    #[inline]
    pub fn _128us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_128US)
    }
    #[doc = "256us sample period."]
    #[inline]
    pub fn _256us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_256US)
    }
    #[doc = "512us sample period."]
    #[inline]
    pub fn _512us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_512US)
    }
    #[doc = "1024us sample period."]
    #[inline]
    pub fn _1024us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_1024US)
    }
    #[doc = "2048us sample period."]
    #[inline]
    pub fn _2048us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_2048US)
    }
    #[doc = "4096us sample period."]
    #[inline]
    pub fn _4096us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_4096US)
    }
    #[doc = "8192us sample period."]
    #[inline]
    pub fn _8192us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_8192US)
    }
    #[doc = "16384us sample period."]
    #[inline]
    pub fn _16384us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_16384US)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Sample period."]
    #[inline]
    pub fn sampleper(&self) -> SAMPLEPERR {
        SAMPLEPERR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Sample period."]
    #[inline]
    pub fn sampleper(&mut self) -> _SAMPLEPERW {
        _SAMPLEPERW { w: self }
    }
}
