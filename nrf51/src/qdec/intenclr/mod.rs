#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENCLR {
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
#[doc = "Possible values of the field `SAMPLERDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDYR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl SAMPLERDYR {
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
            SAMPLERDYR::DISABLED => false,
            SAMPLERDYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAMPLERDYR {
        match value {
            false => SAMPLERDYR::DISABLED,
            true => SAMPLERDYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SAMPLERDYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SAMPLERDYR::ENABLED
    }
}
#[doc = "Possible values of the field `REPORTRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDYR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl REPORTRDYR {
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
            REPORTRDYR::DISABLED => false,
            REPORTRDYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REPORTRDYR {
        match value {
            false => REPORTRDYR::DISABLED,
            true => REPORTRDYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REPORTRDYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDYR::ENABLED
    }
}
#[doc = "Possible values of the field `ACCOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCOFR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl ACCOFR {
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
            ACCOFR::DISABLED => false,
            ACCOFR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACCOFR {
        match value {
            false => ACCOFR::DISABLED,
            true => ACCOFR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ACCOFR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ACCOFR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SAMPLERDY`"]
pub enum SAMPLERDYW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl SAMPLERDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAMPLERDYW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLERDYW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLERDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLERDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SAMPLERDYW::CLEAR)
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
#[doc = "Values that can be written to the field `REPORTRDY`"]
pub enum REPORTRDYW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl REPORTRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REPORTRDYW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REPORTRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _REPORTRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REPORTRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(REPORTRDYW::CLEAR)
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
#[doc = "Values that can be written to the field `ACCOF`"]
pub enum ACCOFW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl ACCOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACCOFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACCOFW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACCOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACCOFW::CLEAR)
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
    #[doc = "Bit 0 - Disable interrupt on SAMPLERDY event."]
    #[inline]
    pub fn samplerdy(&self) -> SAMPLERDYR {
        SAMPLERDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Disable interrupt on REPORTRDY event."]
    #[inline]
    pub fn reportrdy(&self) -> REPORTRDYR {
        REPORTRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Disable interrupt on ACCOF event."]
    #[inline]
    pub fn accof(&self) -> ACCOFR {
        ACCOFR::_from({
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
    #[doc = "Bit 0 - Disable interrupt on SAMPLERDY event."]
    #[inline]
    pub fn samplerdy(&mut self) -> _SAMPLERDYW {
        _SAMPLERDYW { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on REPORTRDY event."]
    #[inline]
    pub fn reportrdy(&mut self) -> _REPORTRDYW {
        _REPORTRDYW { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on ACCOF event."]
    #[inline]
    pub fn accof(&mut self) -> _ACCOFW {
        _ACCOFW { w: self }
    }
}
