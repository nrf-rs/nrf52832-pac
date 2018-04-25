#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `POFWARN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFWARNR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl POFWARNR {
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
            POFWARNR::DISABLED => false,
            POFWARNR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POFWARNR {
        match value {
            false => POFWARNR::DISABLED,
            true => POFWARNR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == POFWARNR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == POFWARNR::ENABLED
    }
}
#[doc = "Possible values of the field `SLEEPENTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPENTERR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SLEEPENTERR {
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
            SLEEPENTERR::DISABLED => false,
            SLEEPENTERR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPENTERR {
        match value {
            false => SLEEPENTERR::DISABLED,
            true => SLEEPENTERR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPENTERR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEPENTERR::ENABLED
    }
}
#[doc = "Possible values of the field `SLEEPEXIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPEXITR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SLEEPEXITR {
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
            SLEEPEXITR::DISABLED => false,
            SLEEPEXITR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPEXITR {
        match value {
            false => SLEEPEXITR::DISABLED,
            true => SLEEPEXITR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPEXITR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEPEXITR::ENABLED
    }
}
#[doc = "Values that can be written to the field `POFWARN`"]
pub enum POFWARNW {
    #[doc = "Enable"]
    SET,
}
impl POFWARNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POFWARNW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POFWARNW<'a> {
    w: &'a mut W,
}
impl<'a> _POFWARNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POFWARNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(POFWARNW::SET)
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
#[doc = "Values that can be written to the field `SLEEPENTER`"]
pub enum SLEEPENTERW {
    #[doc = "Enable"]
    SET,
}
impl SLEEPENTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPENTERW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPENTERW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPENTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPENTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SLEEPENTERW::SET)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLEEPEXIT`"]
pub enum SLEEPEXITW {
    #[doc = "Enable"]
    SET,
}
impl SLEEPEXITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPEXITW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPEXITW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPEXITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPEXITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SLEEPEXITW::SET)
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 2 - Write '1' to Enable interrupt for POFWARN event"]
    #[inline]
    pub fn pofwarn(&self) -> POFWARNR {
        POFWARNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for SLEEPENTER event"]
    #[inline]
    pub fn sleepenter(&self) -> SLEEPENTERR {
        SLEEPENTERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for SLEEPEXIT event"]
    #[inline]
    pub fn sleepexit(&self) -> SLEEPEXITR {
        SLEEPEXITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 2 - Write '1' to Enable interrupt for POFWARN event"]
    #[inline]
    pub fn pofwarn(&mut self) -> _POFWARNW {
        _POFWARNW { w: self }
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for SLEEPENTER event"]
    #[inline]
    pub fn sleepenter(&mut self) -> _SLEEPENTERW {
        _SLEEPENTERW { w: self }
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for SLEEPEXIT event"]
    #[inline]
    pub fn sleepexit(&mut self) -> _SLEEPEXITW {
        _SLEEPEXITW { w: self }
    }
}
