#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BITMODE {
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
#[doc = "Possible values of the field `BITMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITMODER {
    #[doc = "16 bit timer bit width"]
    _16BIT,
    #[doc = "8 bit timer bit width"]
    _08BIT,
    #[doc = "24 bit timer bit width"]
    _24BIT,
    #[doc = "32 bit timer bit width"]
    _32BIT,
}
impl BITMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITMODER::_16BIT => 0,
            BITMODER::_08BIT => 1,
            BITMODER::_24BIT => 2,
            BITMODER::_32BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITMODER {
        match value {
            0 => BITMODER::_16BIT,
            1 => BITMODER::_08BIT,
            2 => BITMODER::_24BIT,
            3 => BITMODER::_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline]
    pub fn is_16bit(&self) -> bool {
        *self == BITMODER::_16BIT
    }
    #[doc = "Checks if the value of the field is `_08BIT`"]
    #[inline]
    pub fn is_08bit(&self) -> bool {
        *self == BITMODER::_08BIT
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline]
    pub fn is_24bit(&self) -> bool {
        *self == BITMODER::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline]
    pub fn is_32bit(&self) -> bool {
        *self == BITMODER::_32BIT
    }
}
#[doc = "Values that can be written to the field `BITMODE`"]
pub enum BITMODEW {
    #[doc = "16 bit timer bit width"]
    _16BIT,
    #[doc = "8 bit timer bit width"]
    _08BIT,
    #[doc = "24 bit timer bit width"]
    _24BIT,
    #[doc = "32 bit timer bit width"]
    _32BIT,
}
impl BITMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BITMODEW::_16BIT => 0,
            BITMODEW::_08BIT => 1,
            BITMODEW::_24BIT => 2,
            BITMODEW::_32BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BITMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "16 bit timer bit width"]
    #[inline]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(BITMODEW::_16BIT)
    }
    #[doc = "8 bit timer bit width"]
    #[inline]
    pub fn _08bit(self) -> &'a mut W {
        self.variant(BITMODEW::_08BIT)
    }
    #[doc = "24 bit timer bit width"]
    #[inline]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(BITMODEW::_24BIT)
    }
    #[doc = "32 bit timer bit width"]
    #[inline]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(BITMODEW::_32BIT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Timer bit width"]
    #[inline]
    pub fn bitmode(&self) -> BITMODER {
        BITMODER::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Timer bit width"]
    #[inline]
    pub fn bitmode(&mut self) -> _BITMODEW {
        _BITMODEW { w: self }
    }
}
