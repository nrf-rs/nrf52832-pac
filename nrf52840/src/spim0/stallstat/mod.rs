#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STALLSTAT {
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
#[doc = "Possible values of the field `TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXR {
    #[doc = "No stall"]
    NOSTALL,
    #[doc = "A stall has occurred"]
    STALL,
}
impl TXR {
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
            TXR::NOSTALL => false,
            TXR::STALL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXR {
        match value {
            false => TXR::NOSTALL,
            true => TXR::STALL,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL`"]
    #[inline]
    pub fn is_nostall(&self) -> bool {
        *self == TXR::NOSTALL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline]
    pub fn is_stall(&self) -> bool {
        *self == TXR::STALL
    }
}
#[doc = "Possible values of the field `RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXR {
    #[doc = "No stall"]
    NOSTALL,
    #[doc = "A stall has occurred"]
    STALL,
}
impl RXR {
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
            RXR::NOSTALL => false,
            RXR::STALL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXR {
        match value {
            false => RXR::NOSTALL,
            true => RXR::STALL,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL`"]
    #[inline]
    pub fn is_nostall(&self) -> bool {
        *self == RXR::NOSTALL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline]
    pub fn is_stall(&self) -> bool {
        *self == RXR::STALL
    }
}
#[doc = "Values that can be written to the field `TX`"]
pub enum TXW {
    #[doc = "No stall"]
    NOSTALL,
    #[doc = "A stall has occurred"]
    STALL,
}
impl TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXW::NOSTALL => false,
            TXW::STALL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXW<'a> {
    w: &'a mut W,
}
impl<'a> _TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No stall"]
    #[inline]
    pub fn nostall(self) -> &'a mut W {
        self.variant(TXW::NOSTALL)
    }
    #[doc = "A stall has occurred"]
    #[inline]
    pub fn stall(self) -> &'a mut W {
        self.variant(TXW::STALL)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX`"]
pub enum RXW {
    #[doc = "No stall"]
    NOSTALL,
    #[doc = "A stall has occurred"]
    STALL,
}
impl RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXW::NOSTALL => false,
            RXW::STALL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXW<'a> {
    w: &'a mut W,
}
impl<'a> _RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No stall"]
    #[inline]
    pub fn nostall(self) -> &'a mut W {
        self.variant(RXW::NOSTALL)
    }
    #[doc = "A stall has occurred"]
    #[inline]
    pub fn stall(self) -> &'a mut W {
        self.variant(RXW::STALL)
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
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Stall status for EasyDMA RAM reads"]
    #[inline]
    pub fn tx(&self) -> TXR {
        TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Stall status for EasyDMA RAM writes"]
    #[inline]
    pub fn rx(&self) -> RXR {
        RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Stall status for EasyDMA RAM reads"]
    #[inline]
    pub fn tx(&mut self) -> _TXW {
        _TXW { w: self }
    }
    #[doc = "Bit 1 - Stall status for EasyDMA RAM writes"]
    #[inline]
    pub fn rx(&mut self) -> _RXW {
        _RXW { w: self }
    }
}
