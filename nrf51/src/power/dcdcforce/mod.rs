#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCDCFORCE {
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
#[doc = "Possible values of the field `FORCEOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFFR {
    #[doc = "No force."]
    NOFORCE,
    #[doc = "Force."]
    FORCE,
}
impl FORCEOFFR {
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
            FORCEOFFR::NOFORCE => false,
            FORCEOFFR::FORCE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCEOFFR {
        match value {
            false => FORCEOFFR::NOFORCE,
            true => FORCEOFFR::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFORCE`"]
    #[inline]
    pub fn is_no_force(&self) -> bool {
        *self == FORCEOFFR::NOFORCE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline]
    pub fn is_force(&self) -> bool {
        *self == FORCEOFFR::FORCE
    }
}
#[doc = "Possible values of the field `FORCEON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEONR {
    #[doc = "No force."]
    NOFORCE,
    #[doc = "Force."]
    FORCE,
}
impl FORCEONR {
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
            FORCEONR::NOFORCE => false,
            FORCEONR::FORCE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCEONR {
        match value {
            false => FORCEONR::NOFORCE,
            true => FORCEONR::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFORCE`"]
    #[inline]
    pub fn is_no_force(&self) -> bool {
        *self == FORCEONR::NOFORCE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline]
    pub fn is_force(&self) -> bool {
        *self == FORCEONR::FORCE
    }
}
#[doc = "Values that can be written to the field `FORCEOFF`"]
pub enum FORCEOFFW {
    #[doc = "No force."]
    NOFORCE,
    #[doc = "Force."]
    FORCE,
}
impl FORCEOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEOFFW::NOFORCE => false,
            FORCEOFFW::FORCE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEOFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No force."]
    #[inline]
    pub fn no_force(self) -> &'a mut W {
        self.variant(FORCEOFFW::NOFORCE)
    }
    #[doc = "Force."]
    #[inline]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEOFFW::FORCE)
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
#[doc = "Values that can be written to the field `FORCEON`"]
pub enum FORCEONW {
    #[doc = "No force."]
    NOFORCE,
    #[doc = "Force."]
    FORCE,
}
impl FORCEONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEONW::NOFORCE => false,
            FORCEONW::FORCE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEONW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No force."]
    #[inline]
    pub fn no_force(self) -> &'a mut W {
        self.variant(FORCEONW::NOFORCE)
    }
    #[doc = "Force."]
    #[inline]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEONW::FORCE)
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
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline]
    pub fn forceoff(&self) -> FORCEOFFR {
        FORCEOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline]
    pub fn forceon(&self) -> FORCEONR {
        FORCEONR::_from({
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
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline]
    pub fn forceoff(&mut self) -> _FORCEOFFW {
        _FORCEOFFW { w: self }
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline]
    pub fn forceon(&mut self) -> _FORCEONW {
        _FORCEONW { w: self }
    }
}
