#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE {
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
#[doc = "Possible values of the field `SP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPR {
    #[doc = "Low-power mode"]
    LOW,
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "High-speed mode"]
    HIGH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPR::LOW => 0,
            SPR::NORMAL => 1,
            SPR::HIGH => 2,
            SPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPR {
        match value {
            0 => SPR::LOW,
            1 => SPR::NORMAL,
            2 => SPR::HIGH,
            i => SPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SPR::LOW
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == SPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SPR::HIGH
    }
}
#[doc = "Possible values of the field `MAIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAINR {
    #[doc = "Single-ended mode"]
    SE,
    #[doc = "Differential mode"]
    DIFF,
}
impl MAINR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MAINR::SE => false,
            MAINR::DIFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAINR {
        match value {
            false => MAINR::SE,
            true => MAINR::DIFF,
        }
    }
    #[doc = "Checks if the value of the field is `SE`"]
    #[inline]
    pub fn is_se(&self) -> bool {
        *self == MAINR::SE
    }
    #[doc = "Checks if the value of the field is `DIFF`"]
    #[inline]
    pub fn is_diff(&self) -> bool {
        *self == MAINR::DIFF
    }
}
#[doc = "Values that can be written to the field `SP`"]
pub enum SPW {
    #[doc = "Low-power mode"]
    LOW,
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "High-speed mode"]
    HIGH,
}
impl SPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPW::LOW => 0,
            SPW::NORMAL => 1,
            SPW::HIGH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPW<'a> {
    w: &'a mut W,
}
impl<'a> _SPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low-power mode"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SPW::LOW)
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(SPW::NORMAL)
    }
    #[doc = "High-speed mode"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SPW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAIN`"]
pub enum MAINW {
    #[doc = "Single-ended mode"]
    SE,
    #[doc = "Differential mode"]
    DIFF,
}
impl MAINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAINW::SE => false,
            MAINW::DIFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAINW<'a> {
    w: &'a mut W,
}
impl<'a> _MAINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single-ended mode"]
    #[inline]
    pub fn se(self) -> &'a mut W {
        self.variant(MAINW::SE)
    }
    #[doc = "Differential mode"]
    #[inline]
    pub fn diff(self) -> &'a mut W {
        self.variant(MAINW::DIFF)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline]
    pub fn sp(&self) -> SPR {
        SPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline]
    pub fn main(&self) -> MAINR {
        MAINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline]
    pub fn sp(&mut self) -> _SPW {
        _SPW { w: self }
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline]
    pub fn main(&mut self) -> _MAINW {
        _MAINW { w: self }
    }
}
