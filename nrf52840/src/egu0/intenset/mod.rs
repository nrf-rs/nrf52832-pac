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
#[doc = "Possible values of the field `TRIGGERED0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED0R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED0R {
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
            TRIGGERED0R::DISABLED => false,
            TRIGGERED0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED0R {
        match value {
            false => TRIGGERED0R::DISABLED,
            true => TRIGGERED0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED0R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED1R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED1R {
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
            TRIGGERED1R::DISABLED => false,
            TRIGGERED1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED1R {
        match value {
            false => TRIGGERED1R::DISABLED,
            true => TRIGGERED1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED1R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED2R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED2R {
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
            TRIGGERED2R::DISABLED => false,
            TRIGGERED2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED2R {
        match value {
            false => TRIGGERED2R::DISABLED,
            true => TRIGGERED2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED2R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED3R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED3R {
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
            TRIGGERED3R::DISABLED => false,
            TRIGGERED3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED3R {
        match value {
            false => TRIGGERED3R::DISABLED,
            true => TRIGGERED3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED3R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED4R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED4R {
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
            TRIGGERED4R::DISABLED => false,
            TRIGGERED4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED4R {
        match value {
            false => TRIGGERED4R::DISABLED,
            true => TRIGGERED4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED4R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED5R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED5R {
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
            TRIGGERED5R::DISABLED => false,
            TRIGGERED5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED5R {
        match value {
            false => TRIGGERED5R::DISABLED,
            true => TRIGGERED5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED5R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED6R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED6R {
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
            TRIGGERED6R::DISABLED => false,
            TRIGGERED6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED6R {
        match value {
            false => TRIGGERED6R::DISABLED,
            true => TRIGGERED6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED6R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED7R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED7R {
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
            TRIGGERED7R::DISABLED => false,
            TRIGGERED7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED7R {
        match value {
            false => TRIGGERED7R::DISABLED,
            true => TRIGGERED7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED7R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED8R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED8R {
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
            TRIGGERED8R::DISABLED => false,
            TRIGGERED8R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED8R {
        match value {
            false => TRIGGERED8R::DISABLED,
            true => TRIGGERED8R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED8R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED8R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED9R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED9R {
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
            TRIGGERED9R::DISABLED => false,
            TRIGGERED9R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED9R {
        match value {
            false => TRIGGERED9R::DISABLED,
            true => TRIGGERED9R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED9R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED9R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED10R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED10R {
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
            TRIGGERED10R::DISABLED => false,
            TRIGGERED10R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED10R {
        match value {
            false => TRIGGERED10R::DISABLED,
            true => TRIGGERED10R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED10R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED10R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED11R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED11R {
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
            TRIGGERED11R::DISABLED => false,
            TRIGGERED11R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED11R {
        match value {
            false => TRIGGERED11R::DISABLED,
            true => TRIGGERED11R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED11R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED11R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED12R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED12R {
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
            TRIGGERED12R::DISABLED => false,
            TRIGGERED12R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED12R {
        match value {
            false => TRIGGERED12R::DISABLED,
            true => TRIGGERED12R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED12R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED12R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED13R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED13R {
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
            TRIGGERED13R::DISABLED => false,
            TRIGGERED13R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED13R {
        match value {
            false => TRIGGERED13R::DISABLED,
            true => TRIGGERED13R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED13R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED13R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED14R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED14R {
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
            TRIGGERED14R::DISABLED => false,
            TRIGGERED14R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED14R {
        match value {
            false => TRIGGERED14R::DISABLED,
            true => TRIGGERED14R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED14R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED14R::ENABLED
    }
}
#[doc = "Possible values of the field `TRIGGERED15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED15R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TRIGGERED15R {
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
            TRIGGERED15R::DISABLED => false,
            TRIGGERED15R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERED15R {
        match value {
            false => TRIGGERED15R::DISABLED,
            true => TRIGGERED15R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED15R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED15R::ENABLED
    }
}
#[doc = "Values that can be written to the field `TRIGGERED0`"]
pub enum TRIGGERED0W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED0W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED0W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED0W::SET)
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
#[doc = "Values that can be written to the field `TRIGGERED1`"]
pub enum TRIGGERED1W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED1W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED1W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED1W::SET)
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
#[doc = "Values that can be written to the field `TRIGGERED2`"]
pub enum TRIGGERED2W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED2W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED2W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED2W::SET)
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
#[doc = "Values that can be written to the field `TRIGGERED3`"]
pub enum TRIGGERED3W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED3W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED3W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED3W::SET)
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
#[doc = "Values that can be written to the field `TRIGGERED4`"]
pub enum TRIGGERED4W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED4W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED4W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED4W::SET)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGERED5`"]
pub enum TRIGGERED5W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED5W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED5W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED5W::SET)
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
#[doc = "Values that can be written to the field `TRIGGERED6`"]
pub enum TRIGGERED6W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED6W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED6W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED6W::SET)
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
#[doc = "Values that can be written to the field `TRIGGERED7`"]
pub enum TRIGGERED7W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED7W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED7W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED7W::SET)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGERED8`"]
pub enum TRIGGERED8W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED8W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED8W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED8W::SET)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGERED9`"]
pub enum TRIGGERED9W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED9W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED9W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED9W::SET)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGERED10`"]
pub enum TRIGGERED10W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED10W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED10W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED10W::SET)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGERED11`"]
pub enum TRIGGERED11W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED11W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED11W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED11W::SET)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGERED12`"]
pub enum TRIGGERED12W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED12W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED12W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED12W::SET)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGERED13`"]
pub enum TRIGGERED13W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED13W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED13W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED13W::SET)
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
#[doc = "Values that can be written to the field `TRIGGERED14`"]
pub enum TRIGGERED14W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED14W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED14W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED14W::SET)
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
#[doc = "Values that can be written to the field `TRIGGERED15`"]
pub enum TRIGGERED15W {
    #[doc = "Enable"]
    SET,
}
impl TRIGGERED15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERED15W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERED15W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERED15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERED15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED15W::SET)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Write '1' to enable interrupt for TRIGGERED[0] event"]
    #[inline]
    pub fn triggered0(&self) -> TRIGGERED0R {
        TRIGGERED0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for TRIGGERED[1] event"]
    #[inline]
    pub fn triggered1(&self) -> TRIGGERED1R {
        TRIGGERED1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for TRIGGERED[2] event"]
    #[inline]
    pub fn triggered2(&self) -> TRIGGERED2R {
        TRIGGERED2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for TRIGGERED[3] event"]
    #[inline]
    pub fn triggered3(&self) -> TRIGGERED3R {
        TRIGGERED3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for TRIGGERED[4] event"]
    #[inline]
    pub fn triggered4(&self) -> TRIGGERED4R {
        TRIGGERED4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for TRIGGERED[5] event"]
    #[inline]
    pub fn triggered5(&self) -> TRIGGERED5R {
        TRIGGERED5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for TRIGGERED[6] event"]
    #[inline]
    pub fn triggered6(&self) -> TRIGGERED6R {
        TRIGGERED6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for TRIGGERED[7] event"]
    #[inline]
    pub fn triggered7(&self) -> TRIGGERED7R {
        TRIGGERED7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for TRIGGERED[8] event"]
    #[inline]
    pub fn triggered8(&self) -> TRIGGERED8R {
        TRIGGERED8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for TRIGGERED[9] event"]
    #[inline]
    pub fn triggered9(&self) -> TRIGGERED9R {
        TRIGGERED9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for TRIGGERED[10] event"]
    #[inline]
    pub fn triggered10(&self) -> TRIGGERED10R {
        TRIGGERED10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for TRIGGERED[11] event"]
    #[inline]
    pub fn triggered11(&self) -> TRIGGERED11R {
        TRIGGERED11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for TRIGGERED[12] event"]
    #[inline]
    pub fn triggered12(&self) -> TRIGGERED12R {
        TRIGGERED12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for TRIGGERED[13] event"]
    #[inline]
    pub fn triggered13(&self) -> TRIGGERED13R {
        TRIGGERED13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for TRIGGERED[14] event"]
    #[inline]
    pub fn triggered14(&self) -> TRIGGERED14R {
        TRIGGERED14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for TRIGGERED[15] event"]
    #[inline]
    pub fn triggered15(&self) -> TRIGGERED15R {
        TRIGGERED15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Write '1' to enable interrupt for TRIGGERED[0] event"]
    #[inline]
    pub fn triggered0(&mut self) -> _TRIGGERED0W {
        _TRIGGERED0W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for TRIGGERED[1] event"]
    #[inline]
    pub fn triggered1(&mut self) -> _TRIGGERED1W {
        _TRIGGERED1W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for TRIGGERED[2] event"]
    #[inline]
    pub fn triggered2(&mut self) -> _TRIGGERED2W {
        _TRIGGERED2W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for TRIGGERED[3] event"]
    #[inline]
    pub fn triggered3(&mut self) -> _TRIGGERED3W {
        _TRIGGERED3W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for TRIGGERED[4] event"]
    #[inline]
    pub fn triggered4(&mut self) -> _TRIGGERED4W {
        _TRIGGERED4W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for TRIGGERED[5] event"]
    #[inline]
    pub fn triggered5(&mut self) -> _TRIGGERED5W {
        _TRIGGERED5W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for TRIGGERED[6] event"]
    #[inline]
    pub fn triggered6(&mut self) -> _TRIGGERED6W {
        _TRIGGERED6W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for TRIGGERED[7] event"]
    #[inline]
    pub fn triggered7(&mut self) -> _TRIGGERED7W {
        _TRIGGERED7W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for TRIGGERED[8] event"]
    #[inline]
    pub fn triggered8(&mut self) -> _TRIGGERED8W {
        _TRIGGERED8W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for TRIGGERED[9] event"]
    #[inline]
    pub fn triggered9(&mut self) -> _TRIGGERED9W {
        _TRIGGERED9W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for TRIGGERED[10] event"]
    #[inline]
    pub fn triggered10(&mut self) -> _TRIGGERED10W {
        _TRIGGERED10W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for TRIGGERED[11] event"]
    #[inline]
    pub fn triggered11(&mut self) -> _TRIGGERED11W {
        _TRIGGERED11W { w: self }
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for TRIGGERED[12] event"]
    #[inline]
    pub fn triggered12(&mut self) -> _TRIGGERED12W {
        _TRIGGERED12W { w: self }
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for TRIGGERED[13] event"]
    #[inline]
    pub fn triggered13(&mut self) -> _TRIGGERED13W {
        _TRIGGERED13W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for TRIGGERED[14] event"]
    #[inline]
    pub fn triggered14(&mut self) -> _TRIGGERED14W {
        _TRIGGERED14W { w: self }
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for TRIGGERED[15] event"]
    #[inline]
    pub fn triggered15(&mut self) -> _TRIGGERED15W {
        _TRIGGERED15W { w: self }
    }
}
