#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG0 {
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
#[doc = "Possible values of the field `REGION0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION0R {
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
            REGION0R::DISABLED => false,
            REGION0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION0R {
        match value {
            false => REGION0R::DISABLED,
            true => REGION0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION0R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION1R {
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
            REGION1R::DISABLED => false,
            REGION1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION1R {
        match value {
            false => REGION1R::DISABLED,
            true => REGION1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION1R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION2R {
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
            REGION2R::DISABLED => false,
            REGION2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION2R {
        match value {
            false => REGION2R::DISABLED,
            true => REGION2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION2R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION3R {
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
            REGION3R::DISABLED => false,
            REGION3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION3R {
        match value {
            false => REGION3R::DISABLED,
            true => REGION3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION3R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION4R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION4R {
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
            REGION4R::DISABLED => false,
            REGION4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION4R {
        match value {
            false => REGION4R::DISABLED,
            true => REGION4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION4R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION5R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION5R {
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
            REGION5R::DISABLED => false,
            REGION5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION5R {
        match value {
            false => REGION5R::DISABLED,
            true => REGION5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION5R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION6R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION6R {
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
            REGION6R::DISABLED => false,
            REGION6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION6R {
        match value {
            false => REGION6R::DISABLED,
            true => REGION6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION6R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION7R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION7R {
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
            REGION7R::DISABLED => false,
            REGION7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION7R {
        match value {
            false => REGION7R::DISABLED,
            true => REGION7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION7R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION8R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION8R {
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
            REGION8R::DISABLED => false,
            REGION8R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION8R {
        match value {
            false => REGION8R::DISABLED,
            true => REGION8R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION8R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION8R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION9R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION9R {
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
            REGION9R::DISABLED => false,
            REGION9R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION9R {
        match value {
            false => REGION9R::DISABLED,
            true => REGION9R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION9R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION9R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION10R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION10R {
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
            REGION10R::DISABLED => false,
            REGION10R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION10R {
        match value {
            false => REGION10R::DISABLED,
            true => REGION10R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION10R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION10R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION11R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION11R {
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
            REGION11R::DISABLED => false,
            REGION11R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION11R {
        match value {
            false => REGION11R::DISABLED,
            true => REGION11R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION11R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION11R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION12R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION12R {
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
            REGION12R::DISABLED => false,
            REGION12R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION12R {
        match value {
            false => REGION12R::DISABLED,
            true => REGION12R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION12R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION12R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION13R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION13R {
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
            REGION13R::DISABLED => false,
            REGION13R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION13R {
        match value {
            false => REGION13R::DISABLED,
            true => REGION13R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION13R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION13R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION14R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION14R {
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
            REGION14R::DISABLED => false,
            REGION14R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION14R {
        match value {
            false => REGION14R::DISABLED,
            true => REGION14R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION14R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION14R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION15R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION15R {
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
            REGION15R::DISABLED => false,
            REGION15R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION15R {
        match value {
            false => REGION15R::DISABLED,
            true => REGION15R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION15R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION15R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION16R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION16R {
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
            REGION16R::DISABLED => false,
            REGION16R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION16R {
        match value {
            false => REGION16R::DISABLED,
            true => REGION16R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION16R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION16R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION17R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION17R {
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
            REGION17R::DISABLED => false,
            REGION17R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION17R {
        match value {
            false => REGION17R::DISABLED,
            true => REGION17R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION17R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION17R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION18R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION18R {
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
            REGION18R::DISABLED => false,
            REGION18R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION18R {
        match value {
            false => REGION18R::DISABLED,
            true => REGION18R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION18R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION18R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION19R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION19R {
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
            REGION19R::DISABLED => false,
            REGION19R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION19R {
        match value {
            false => REGION19R::DISABLED,
            true => REGION19R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION19R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION19R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION20R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION20R {
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
            REGION20R::DISABLED => false,
            REGION20R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION20R {
        match value {
            false => REGION20R::DISABLED,
            true => REGION20R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION20R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION20R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION21R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION21R {
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
            REGION21R::DISABLED => false,
            REGION21R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION21R {
        match value {
            false => REGION21R::DISABLED,
            true => REGION21R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION21R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION21R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION22R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION22R {
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
            REGION22R::DISABLED => false,
            REGION22R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION22R {
        match value {
            false => REGION22R::DISABLED,
            true => REGION22R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION22R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION22R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION23R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION23R {
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
            REGION23R::DISABLED => false,
            REGION23R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION23R {
        match value {
            false => REGION23R::DISABLED,
            true => REGION23R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION23R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION23R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION24R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION24R {
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
            REGION24R::DISABLED => false,
            REGION24R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION24R {
        match value {
            false => REGION24R::DISABLED,
            true => REGION24R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION24R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION24R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION25R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION25R {
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
            REGION25R::DISABLED => false,
            REGION25R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION25R {
        match value {
            false => REGION25R::DISABLED,
            true => REGION25R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION25R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION25R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION26R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION26R {
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
            REGION26R::DISABLED => false,
            REGION26R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION26R {
        match value {
            false => REGION26R::DISABLED,
            true => REGION26R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION26R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION26R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION27R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION27R {
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
            REGION27R::DISABLED => false,
            REGION27R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION27R {
        match value {
            false => REGION27R::DISABLED,
            true => REGION27R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION27R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION27R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION28R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION28R {
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
            REGION28R::DISABLED => false,
            REGION28R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION28R {
        match value {
            false => REGION28R::DISABLED,
            true => REGION28R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION28R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION28R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION29R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION29R {
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
            REGION29R::DISABLED => false,
            REGION29R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION29R {
        match value {
            false => REGION29R::DISABLED,
            true => REGION29R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION29R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION29R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION30R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION30R {
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
            REGION30R::DISABLED => false,
            REGION30R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION30R {
        match value {
            false => REGION30R::DISABLED,
            true => REGION30R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION30R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION30R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION31R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION31R {
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
            REGION31R::DISABLED => false,
            REGION31R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION31R {
        match value {
            false => REGION31R::DISABLED,
            true => REGION31R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION31R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION31R::ENABLED
    }
}
#[doc = "Values that can be written to the field `REGION0`"]
pub enum REGION0W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION0W::DISABLED => false,
            REGION0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION0W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION0W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION0W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION1`"]
pub enum REGION1W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION1W::DISABLED => false,
            REGION1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION1W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION1W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION1W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION2`"]
pub enum REGION2W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION2W::DISABLED => false,
            REGION2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION2W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION2W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION2W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION3`"]
pub enum REGION3W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION3W::DISABLED => false,
            REGION3W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION3W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION3W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION3W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION4`"]
pub enum REGION4W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION4W::DISABLED => false,
            REGION4W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION4W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION4W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION4W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION5`"]
pub enum REGION5W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION5W::DISABLED => false,
            REGION5W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION5W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION5W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION5W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION6`"]
pub enum REGION6W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION6W::DISABLED => false,
            REGION6W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION6W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION6W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION6W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION7`"]
pub enum REGION7W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION7W::DISABLED => false,
            REGION7W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION7W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION7W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION7W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION8`"]
pub enum REGION8W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION8W::DISABLED => false,
            REGION8W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION8W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION8W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION8W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION9`"]
pub enum REGION9W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION9W::DISABLED => false,
            REGION9W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION9W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION9W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION9W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION10`"]
pub enum REGION10W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION10W::DISABLED => false,
            REGION10W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION10W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION10W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION10W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION11`"]
pub enum REGION11W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION11W::DISABLED => false,
            REGION11W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION11W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION11W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION11W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION12`"]
pub enum REGION12W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION12W::DISABLED => false,
            REGION12W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION12W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION12W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION12W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION13`"]
pub enum REGION13W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION13W::DISABLED => false,
            REGION13W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION13W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION13W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION13W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION14`"]
pub enum REGION14W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION14W::DISABLED => false,
            REGION14W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION14W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION14W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION14W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION15`"]
pub enum REGION15W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION15W::DISABLED => false,
            REGION15W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION15W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION15W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION15W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION16`"]
pub enum REGION16W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION16W::DISABLED => false,
            REGION16W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION16W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION16W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION16W::ENABLED)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION17`"]
pub enum REGION17W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION17W::DISABLED => false,
            REGION17W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION17W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION17W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION17W::ENABLED)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION18`"]
pub enum REGION18W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION18W::DISABLED => false,
            REGION18W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION18W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION18W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION18W::ENABLED)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION19`"]
pub enum REGION19W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION19W::DISABLED => false,
            REGION19W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION19W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION19W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION19W::ENABLED)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION20`"]
pub enum REGION20W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION20W::DISABLED => false,
            REGION20W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION20W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION20W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION20W::ENABLED)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION21`"]
pub enum REGION21W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION21W::DISABLED => false,
            REGION21W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION21W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION21W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION21W::ENABLED)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION22`"]
pub enum REGION22W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION22W::DISABLED => false,
            REGION22W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION22W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION22W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION22W::ENABLED)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION23`"]
pub enum REGION23W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION23W::DISABLED => false,
            REGION23W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION23W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION23W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION23W::ENABLED)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION24`"]
pub enum REGION24W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION24W::DISABLED => false,
            REGION24W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION24W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION24W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION24W::ENABLED)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION25`"]
pub enum REGION25W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION25W::DISABLED => false,
            REGION25W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION25W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION25W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION25W::ENABLED)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION26`"]
pub enum REGION26W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION26W::DISABLED => false,
            REGION26W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION26W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION26W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION26W::ENABLED)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION27`"]
pub enum REGION27W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION27W::DISABLED => false,
            REGION27W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION27W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION27W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION27W::ENABLED)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION28`"]
pub enum REGION28W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION28W::DISABLED => false,
            REGION28W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION28W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION28W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION28W::ENABLED)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION29`"]
pub enum REGION29W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION29W::DISABLED => false,
            REGION29W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION29W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION29W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION29W::ENABLED)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION30`"]
pub enum REGION30W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION30W::DISABLED => false,
            REGION30W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION30W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION30W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION30W::ENABLED)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REGION31`"]
pub enum REGION31W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enable"]
    ENABLED,
}
impl REGION31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION31W::DISABLED => false,
            REGION31W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION31W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION31W::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION31W::ENABLED)
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
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline]
    pub fn region0(&self) -> REGION0R {
        REGION0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline]
    pub fn region1(&self) -> REGION1R {
        REGION1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline]
    pub fn region2(&self) -> REGION2R {
        REGION2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline]
    pub fn region3(&self) -> REGION3R {
        REGION3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline]
    pub fn region4(&self) -> REGION4R {
        REGION4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline]
    pub fn region5(&self) -> REGION5R {
        REGION5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline]
    pub fn region6(&self) -> REGION6R {
        REGION6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline]
    pub fn region7(&self) -> REGION7R {
        REGION7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline]
    pub fn region8(&self) -> REGION8R {
        REGION8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline]
    pub fn region9(&self) -> REGION9R {
        REGION9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline]
    pub fn region10(&self) -> REGION10R {
        REGION10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline]
    pub fn region11(&self) -> REGION11R {
        REGION11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline]
    pub fn region12(&self) -> REGION12R {
        REGION12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline]
    pub fn region13(&self) -> REGION13R {
        REGION13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline]
    pub fn region14(&self) -> REGION14R {
        REGION14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline]
    pub fn region15(&self) -> REGION15R {
        REGION15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline]
    pub fn region16(&self) -> REGION16R {
        REGION16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline]
    pub fn region17(&self) -> REGION17R {
        REGION17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline]
    pub fn region18(&self) -> REGION18R {
        REGION18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline]
    pub fn region19(&self) -> REGION19R {
        REGION19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline]
    pub fn region20(&self) -> REGION20R {
        REGION20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline]
    pub fn region21(&self) -> REGION21R {
        REGION21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline]
    pub fn region22(&self) -> REGION22R {
        REGION22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline]
    pub fn region23(&self) -> REGION23R {
        REGION23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline]
    pub fn region24(&self) -> REGION24R {
        REGION24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline]
    pub fn region25(&self) -> REGION25R {
        REGION25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline]
    pub fn region26(&self) -> REGION26R {
        REGION26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline]
    pub fn region27(&self) -> REGION27R {
        REGION27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline]
    pub fn region28(&self) -> REGION28R {
        REGION28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline]
    pub fn region29(&self) -> REGION29R {
        REGION29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline]
    pub fn region30(&self) -> REGION30R {
        REGION30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline]
    pub fn region31(&self) -> REGION31R {
        REGION31R::_from({
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
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline]
    pub fn region0(&mut self) -> _REGION0W {
        _REGION0W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline]
    pub fn region1(&mut self) -> _REGION1W {
        _REGION1W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline]
    pub fn region2(&mut self) -> _REGION2W {
        _REGION2W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline]
    pub fn region3(&mut self) -> _REGION3W {
        _REGION3W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline]
    pub fn region4(&mut self) -> _REGION4W {
        _REGION4W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline]
    pub fn region5(&mut self) -> _REGION5W {
        _REGION5W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline]
    pub fn region6(&mut self) -> _REGION6W {
        _REGION6W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline]
    pub fn region7(&mut self) -> _REGION7W {
        _REGION7W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline]
    pub fn region8(&mut self) -> _REGION8W {
        _REGION8W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline]
    pub fn region9(&mut self) -> _REGION9W {
        _REGION9W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline]
    pub fn region10(&mut self) -> _REGION10W {
        _REGION10W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline]
    pub fn region11(&mut self) -> _REGION11W {
        _REGION11W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline]
    pub fn region12(&mut self) -> _REGION12W {
        _REGION12W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline]
    pub fn region13(&mut self) -> _REGION13W {
        _REGION13W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline]
    pub fn region14(&mut self) -> _REGION14W {
        _REGION14W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline]
    pub fn region15(&mut self) -> _REGION15W {
        _REGION15W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline]
    pub fn region16(&mut self) -> _REGION16W {
        _REGION16W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline]
    pub fn region17(&mut self) -> _REGION17W {
        _REGION17W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline]
    pub fn region18(&mut self) -> _REGION18W {
        _REGION18W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline]
    pub fn region19(&mut self) -> _REGION19W {
        _REGION19W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline]
    pub fn region20(&mut self) -> _REGION20W {
        _REGION20W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline]
    pub fn region21(&mut self) -> _REGION21W {
        _REGION21W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline]
    pub fn region22(&mut self) -> _REGION22W {
        _REGION22W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline]
    pub fn region23(&mut self) -> _REGION23W {
        _REGION23W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline]
    pub fn region24(&mut self) -> _REGION24W {
        _REGION24W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline]
    pub fn region25(&mut self) -> _REGION25W {
        _REGION25W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline]
    pub fn region26(&mut self) -> _REGION26W {
        _REGION26W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline]
    pub fn region27(&mut self) -> _REGION27W {
        _REGION27W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline]
    pub fn region28(&mut self) -> _REGION28W {
        _REGION28W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline]
    pub fn region29(&mut self) -> _REGION29W {
        _REGION29W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline]
    pub fn region30(&mut self) -> _REGION30W {
        _REGION30W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline]
    pub fn region31(&mut self) -> _REGION31W {
        _REGION31W { w: self }
    }
}
