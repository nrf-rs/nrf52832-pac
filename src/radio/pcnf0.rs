#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCNF0 {
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
#[doc = r" Value of the field"]
pub struct LFLENR {
    bits: u8,
}
impl LFLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S0LENR {
    bits: bool,
}
impl S0LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct S1LENR {
    bits: u8,
}
impl S1LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `S1INCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1INCLR {
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    AUTOMATIC,
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    INCLUDE,
}
impl S1INCLR {
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
            S1INCLR::AUTOMATIC => false,
            S1INCLR::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1INCLR {
        match value {
            false => S1INCLR::AUTOMATIC,
            true => S1INCLR::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline]
    pub fn is_automatic(&self) -> bool {
        *self == S1INCLR::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == S1INCLR::INCLUDE
    }
}
#[doc = "Possible values of the field `PLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLENR {
    #[doc = "8-bit preamble"]
    _8BIT,
    #[doc = "16-bit preamble"]
    _16BIT,
}
impl PLENR {
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
            PLENR::_8BIT => false,
            PLENR::_16BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLENR {
        match value {
            false => PLENR::_8BIT,
            true => PLENR::_16BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline]
    pub fn is_8bit(&self) -> bool {
        *self == PLENR::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline]
    pub fn is_16bit(&self) -> bool {
        *self == PLENR::_16BIT
    }
}
#[doc = r" Proxy"]
pub struct _LFLENW<'a> {
    w: &'a mut W,
}
impl<'a> _LFLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S0LENW<'a> {
    w: &'a mut W,
}
impl<'a> _S0LENW<'a> {
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
#[doc = r" Proxy"]
pub struct _S1LENW<'a> {
    w: &'a mut W,
}
impl<'a> _S1LENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S1INCL`"]
pub enum S1INCLW {
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    AUTOMATIC,
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    INCLUDE,
}
impl S1INCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1INCLW::AUTOMATIC => false,
            S1INCLW::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1INCLW<'a> {
    w: &'a mut W,
}
impl<'a> _S1INCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1INCLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    #[inline]
    pub fn automatic(self) -> &'a mut W {
        self.variant(S1INCLW::AUTOMATIC)
    }
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(S1INCLW::INCLUDE)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLEN`"]
pub enum PLENW {
    #[doc = "8-bit preamble"]
    _8BIT,
    #[doc = "16-bit preamble"]
    _16BIT,
}
impl PLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLENW::_8BIT => false,
            PLENW::_16BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLENW<'a> {
    w: &'a mut W,
}
impl<'a> _PLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit preamble"]
    #[inline]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(PLENW::_8BIT)
    }
    #[doc = "16-bit preamble"]
    #[inline]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(PLENW::_16BIT)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline]
    pub fn lflen(&self) -> LFLENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LFLENR { bits }
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline]
    pub fn s0len(&self) -> S0LENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S0LENR { bits }
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline]
    pub fn s1len(&self) -> S1LENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S1LENR { bits }
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline]
    pub fn s1incl(&self) -> S1INCLR {
        S1INCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline]
    pub fn plen(&self) -> PLENR {
        PLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline]
    pub fn lflen(&mut self) -> _LFLENW {
        _LFLENW { w: self }
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline]
    pub fn s0len(&mut self) -> _S0LENW {
        _S0LENW { w: self }
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline]
    pub fn s1len(&mut self) -> _S1LENW {
        _S1LENW { w: self }
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline]
    pub fn s1incl(&mut self) -> _S1INCLW {
        _S1INCLW { w: self }
    }
    #[doc = "Bit 24 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline]
    pub fn plen(&mut self) -> _PLENW {
        _PLENW { w: self }
    }
}
