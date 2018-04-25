#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
#[doc = "Possible values of the field `ORDER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORDERR {
    #[doc = "Most significant bit shifted out first"]
    MSBFIRST,
    #[doc = "Least significant bit shifted out first"]
    LSBFIRST,
}
impl ORDERR {
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
            ORDERR::MSBFIRST => false,
            ORDERR::LSBFIRST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORDERR {
        match value {
            false => ORDERR::MSBFIRST,
            true => ORDERR::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline]
    pub fn is_msb_first(&self) -> bool {
        *self == ORDERR::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline]
    pub fn is_lsb_first(&self) -> bool {
        *self == ORDERR::LSBFIRST
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    LEADING,
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    TRAILING,
}
impl CPHAR {
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
            CPHAR::LEADING => false,
            CPHAR::TRAILING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::LEADING,
            true => CPHAR::TRAILING,
        }
    }
    #[doc = "Checks if the value of the field is `LEADING`"]
    #[inline]
    pub fn is_leading(&self) -> bool {
        *self == CPHAR::LEADING
    }
    #[doc = "Checks if the value of the field is `TRAILING`"]
    #[inline]
    pub fn is_trailing(&self) -> bool {
        *self == CPHAR::TRAILING
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "Active high"]
    ACTIVEHIGH,
    #[doc = "Active low"]
    ACTIVELOW,
}
impl CPOLR {
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
            CPOLR::ACTIVEHIGH => false,
            CPOLR::ACTIVELOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::ACTIVEHIGH,
            true => CPOLR::ACTIVELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline]
    pub fn is_active_high(&self) -> bool {
        *self == CPOLR::ACTIVEHIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline]
    pub fn is_active_low(&self) -> bool {
        *self == CPOLR::ACTIVELOW
    }
}
#[doc = "Values that can be written to the field `ORDER`"]
pub enum ORDERW {
    #[doc = "Most significant bit shifted out first"]
    MSBFIRST,
    #[doc = "Least significant bit shifted out first"]
    LSBFIRST,
}
impl ORDERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ORDERW::MSBFIRST => false,
            ORDERW::LSBFIRST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ORDERW<'a> {
    w: &'a mut W,
}
impl<'a> _ORDERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ORDERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Most significant bit shifted out first"]
    #[inline]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(ORDERW::MSBFIRST)
    }
    #[doc = "Least significant bit shifted out first"]
    #[inline]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(ORDERW::LSBFIRST)
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
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    LEADING,
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    TRAILING,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::LEADING => false,
            CPHAW::TRAILING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    #[inline]
    pub fn leading(self) -> &'a mut W {
        self.variant(CPHAW::LEADING)
    }
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    #[inline]
    pub fn trailing(self) -> &'a mut W {
        self.variant(CPHAW::TRAILING)
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
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "Active high"]
    ACTIVEHIGH,
    #[doc = "Active low"]
    ACTIVELOW,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::ACTIVEHIGH => false,
            CPOLW::ACTIVELOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CPOLW::ACTIVEHIGH)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CPOLW::ACTIVELOW)
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
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Bit order"]
    #[inline]
    pub fn order(&self) -> ORDERR {
        ORDERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Bit order"]
    #[inline]
    pub fn order(&mut self) -> _ORDERW {
        _ORDERW { w: self }
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
}
