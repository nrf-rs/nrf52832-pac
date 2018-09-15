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
#[doc = "Possible values of the field `IN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0R {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl IN0R {
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
            IN0R::DISABLED => false,
            IN0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN0R {
        match value {
            false => IN0R::DISABLED,
            true => IN0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IN0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IN0R::ENABLED
    }
}
#[doc = "Possible values of the field `IN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1R {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl IN1R {
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
            IN1R::DISABLED => false,
            IN1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN1R {
        match value {
            false => IN1R::DISABLED,
            true => IN1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IN1R::ENABLED
    }
}
#[doc = "Possible values of the field `IN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2R {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl IN2R {
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
            IN2R::DISABLED => false,
            IN2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN2R {
        match value {
            false => IN2R::DISABLED,
            true => IN2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IN2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IN2R::ENABLED
    }
}
#[doc = "Possible values of the field `IN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3R {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl IN3R {
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
            IN3R::DISABLED => false,
            IN3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN3R {
        match value {
            false => IN3R::DISABLED,
            true => IN3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IN3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IN3R::ENABLED
    }
}
#[doc = "Possible values of the field `PORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl PORTR {
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
            PORTR::DISABLED => false,
            PORTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PORTR {
        match value {
            false => PORTR::DISABLED,
            true => PORTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PORTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PORTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `IN0`"]
pub enum IN0W {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl IN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN0W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN0W<'a> {
    w: &'a mut W,
}
impl<'a> _IN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(IN0W::SET)
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
#[doc = "Values that can be written to the field `IN1`"]
pub enum IN1W {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl IN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN1W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN1W<'a> {
    w: &'a mut W,
}
impl<'a> _IN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(IN1W::SET)
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
#[doc = "Values that can be written to the field `IN2`"]
pub enum IN2W {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl IN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN2W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN2W<'a> {
    w: &'a mut W,
}
impl<'a> _IN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(IN2W::SET)
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
#[doc = "Values that can be written to the field `IN3`"]
pub enum IN3W {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl IN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN3W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN3W<'a> {
    w: &'a mut W,
}
impl<'a> _IN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(IN3W::SET)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PORT`"]
pub enum PORTW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl PORTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PORTW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTW<'a> {
    w: &'a mut W,
}
impl<'a> _PORTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PORTW::SET)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Enable interrupt on IN[0] event."]
    #[inline]
    pub fn in0(&self) -> IN0R {
        IN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable interrupt on IN[1] event."]
    #[inline]
    pub fn in1(&self) -> IN1R {
        IN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable interrupt on IN[2] event."]
    #[inline]
    pub fn in2(&self) -> IN2R {
        IN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable interrupt on IN[3] event."]
    #[inline]
    pub fn in3(&self) -> IN3R {
        IN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable interrupt on PORT event."]
    #[inline]
    pub fn port(&self) -> PORTR {
        PORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Enable interrupt on IN[0] event."]
    #[inline]
    pub fn in0(&mut self) -> _IN0W {
        _IN0W { w: self }
    }
    #[doc = "Bit 1 - Enable interrupt on IN[1] event."]
    #[inline]
    pub fn in1(&mut self) -> _IN1W {
        _IN1W { w: self }
    }
    #[doc = "Bit 2 - Enable interrupt on IN[2] event."]
    #[inline]
    pub fn in2(&mut self) -> _IN2W {
        _IN2W { w: self }
    }
    #[doc = "Bit 3 - Enable interrupt on IN[3] event."]
    #[inline]
    pub fn in3(&mut self) -> _IN3W {
        _IN3W { w: self }
    }
    #[doc = "Bit 31 - Enable interrupt on PORT event."]
    #[inline]
    pub fn port(&mut self) -> _PORTW {
        _PORTW { w: self }
    }
}
