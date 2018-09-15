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
#[doc = "Possible values of the field `END`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl ENDR {
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
            ENDR::DISABLED => false,
            ENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDR {
        match value {
            false => ENDR::DISABLED,
            true => ENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDR::ENABLED
    }
}
#[doc = "Possible values of the field `RESOLVED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLVEDR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl RESOLVEDR {
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
            RESOLVEDR::DISABLED => false,
            RESOLVEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESOLVEDR {
        match value {
            false => RESOLVEDR::DISABLED,
            true => RESOLVEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RESOLVEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RESOLVEDR::ENABLED
    }
}
#[doc = "Possible values of the field `NOTRESOLVED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRESOLVEDR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl NOTRESOLVEDR {
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
            NOTRESOLVEDR::DISABLED => false,
            NOTRESOLVEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOTRESOLVEDR {
        match value {
            false => NOTRESOLVEDR::DISABLED,
            true => NOTRESOLVEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NOTRESOLVEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NOTRESOLVEDR::ENABLED
    }
}
#[doc = "Values that can be written to the field `END`"]
pub enum ENDW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl ENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDW::SET)
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
#[doc = "Values that can be written to the field `RESOLVED`"]
pub enum RESOLVEDW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl RESOLVEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESOLVEDW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESOLVEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RESOLVEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESOLVEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RESOLVEDW::SET)
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
#[doc = "Values that can be written to the field `NOTRESOLVED`"]
pub enum NOTRESOLVEDW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl NOTRESOLVEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOTRESOLVEDW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOTRESOLVEDW<'a> {
    w: &'a mut W,
}
impl<'a> _NOTRESOLVEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOTRESOLVEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(NOTRESOLVEDW::SET)
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
    #[doc = "Bit 0 - Enable interrupt on END event."]
    #[inline]
    pub fn end(&self) -> ENDR {
        ENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable interrupt on RESOLVED event."]
    #[inline]
    pub fn resolved(&self) -> RESOLVEDR {
        RESOLVEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable interrupt on NOTRESOLVED event."]
    #[inline]
    pub fn notresolved(&self) -> NOTRESOLVEDR {
        NOTRESOLVEDR::_from({
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
    #[doc = "Bit 0 - Enable interrupt on END event."]
    #[inline]
    pub fn end(&mut self) -> _ENDW {
        _ENDW { w: self }
    }
    #[doc = "Bit 1 - Enable interrupt on RESOLVED event."]
    #[inline]
    pub fn resolved(&mut self) -> _RESOLVEDW {
        _RESOLVEDW { w: self }
    }
    #[doc = "Bit 2 - Enable interrupt on NOTRESOLVED event."]
    #[inline]
    pub fn notresolved(&mut self) -> _NOTRESOLVEDW {
        _NOTRESOLVEDW { w: self }
    }
}
