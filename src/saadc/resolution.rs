#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESOLUTION {
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
#[doc = "Possible values of the field `VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALR {
    #[doc = "8 bit"]
    _8BIT,
    #[doc = "10 bit"]
    _10BIT,
    #[doc = "12 bit"]
    _12BIT,
    #[doc = "14 bit"]
    _14BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VALR::_8BIT => 0,
            VALR::_10BIT => 1,
            VALR::_12BIT => 2,
            VALR::_14BIT => 3,
            VALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VALR {
        match value {
            0 => VALR::_8BIT,
            1 => VALR::_10BIT,
            2 => VALR::_12BIT,
            3 => VALR::_14BIT,
            i => VALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline]
    pub fn is_8bit(&self) -> bool {
        *self == VALR::_8BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline]
    pub fn is_10bit(&self) -> bool {
        *self == VALR::_10BIT
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline]
    pub fn is_12bit(&self) -> bool {
        *self == VALR::_12BIT
    }
    #[doc = "Checks if the value of the field is `_14BIT`"]
    #[inline]
    pub fn is_14bit(&self) -> bool {
        *self == VALR::_14BIT
    }
}
#[doc = "Values that can be written to the field `VAL`"]
pub enum VALW {
    #[doc = "8 bit"]
    _8BIT,
    #[doc = "10 bit"]
    _10BIT,
    #[doc = "12 bit"]
    _12BIT,
    #[doc = "14 bit"]
    _14BIT,
}
impl VALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VALW::_8BIT => 0,
            VALW::_10BIT => 1,
            VALW::_12BIT => 2,
            VALW::_14BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALW<'a> {
    w: &'a mut W,
}
impl<'a> _VALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bit"]
    #[inline]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(VALW::_8BIT)
    }
    #[doc = "10 bit"]
    #[inline]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(VALW::_10BIT)
    }
    #[doc = "12 bit"]
    #[inline]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(VALW::_12BIT)
    }
    #[doc = "14 bit"]
    #[inline]
    pub fn _14bit(self) -> &'a mut W {
        self.variant(VALW::_14BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:2 - Set the resolution"]
    #[inline]
    pub fn val(&self) -> VALR {
        VALR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Set the resolution"]
    #[inline]
    pub fn val(&mut self) -> _VALW {
        _VALW { w: self }
    }
}
