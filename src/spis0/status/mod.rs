#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
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
#[doc = "Possible values of the field `OVERREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREADR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl OVERREADR {
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
            OVERREADR::NOTPRESENT => false,
            OVERREADR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERREADR {
        match value {
            false => OVERREADR::NOTPRESENT,
            true => OVERREADR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == OVERREADR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == OVERREADR::PRESENT
    }
}
#[doc = "Possible values of the field `OVERFLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOWR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl OVERFLOWR {
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
            OVERFLOWR::NOTPRESENT => false,
            OVERFLOWR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERFLOWR {
        match value {
            false => OVERFLOWR::NOTPRESENT,
            true => OVERFLOWR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == OVERFLOWR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == OVERFLOWR::PRESENT
    }
}
#[doc = "Values that can be written to the field `OVERREAD`"]
pub enum OVERREADW {
    #[doc = "Write: clear error on writing '1'"]
    CLEAR,
}
impl OVERREADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERREADW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERREADW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERREADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERREADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERREADW::CLEAR)
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
#[doc = "Values that can be written to the field `OVERFLOW`"]
pub enum OVERFLOWW {
    #[doc = "Write: clear error on writing '1'"]
    CLEAR,
}
impl OVERFLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERFLOWW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERFLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOWW::CLEAR)
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
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline]
    pub fn overread(&self) -> OVERREADR {
        OVERREADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline]
    pub fn overflow(&self) -> OVERFLOWR {
        OVERFLOWR::_from({
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
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline]
    pub fn overread(&mut self) -> _OVERREADW {
        _OVERREADW { w: self }
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline]
    pub fn overflow(&mut self) -> _OVERFLOWW {
        _OVERFLOWW { w: self }
    }
}
