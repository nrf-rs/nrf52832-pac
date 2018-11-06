#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS {
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
#[doc = "Possible values of the field `WRITE_SUSPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_SUSPENDR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl WRITE_SUSPENDR {
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
            WRITE_SUSPENDR::DISABLED => false,
            WRITE_SUSPENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITE_SUSPENDR {
        match value {
            false => WRITE_SUSPENDR::DISABLED,
            true => WRITE_SUSPENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WRITE_SUSPENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WRITE_SUSPENDR::ENABLED
    }
}
#[doc = "Possible values of the field `READ_SUSPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_SUSPENDR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl READ_SUSPENDR {
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
            READ_SUSPENDR::DISABLED => false,
            READ_SUSPENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READ_SUSPENDR {
        match value {
            false => READ_SUSPENDR::DISABLED,
            true => READ_SUSPENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == READ_SUSPENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == READ_SUSPENDR::ENABLED
    }
}
#[doc = "Values that can be written to the field `WRITE_SUSPEND`"]
pub enum WRITE_SUSPENDW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl WRITE_SUSPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRITE_SUSPENDW::DISABLED => false,
            WRITE_SUSPENDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRITE_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_SUSPENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITE_SUSPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITE_SUSPENDW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WRITE_SUSPENDW::ENABLED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `READ_SUSPEND`"]
pub enum READ_SUSPENDW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl READ_SUSPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READ_SUSPENDW::DISABLED => false,
            READ_SUSPENDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READ_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_SUSPENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READ_SUSPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READ_SUSPENDW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READ_SUSPENDW::ENABLED)
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 13 - Shortcut between WRITE event and SUSPEND task"]
    #[inline]
    pub fn write_suspend(&self) -> WRITE_SUSPENDR {
        WRITE_SUSPENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Shortcut between READ event and SUSPEND task"]
    #[inline]
    pub fn read_suspend(&self) -> READ_SUSPENDR {
        READ_SUSPENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
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
    #[doc = "Bit 13 - Shortcut between WRITE event and SUSPEND task"]
    #[inline]
    pub fn write_suspend(&mut self) -> _WRITE_SUSPENDW {
        _WRITE_SUSPENDW { w: self }
    }
    #[doc = "Bit 14 - Shortcut between READ event and SUSPEND task"]
    #[inline]
    pub fn read_suspend(&mut self) -> _READ_SUSPENDW {
        _READ_SUSPENDW { w: self }
    }
}
