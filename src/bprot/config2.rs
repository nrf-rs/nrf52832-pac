#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG2 {
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
#[doc = "Possible values of the field `REGION64`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION64R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION64R {
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
            REGION64R::DISABLED => false,
            REGION64R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION64R {
        match value {
            false => REGION64R::DISABLED,
            true => REGION64R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION64R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION64R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION65`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION65R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION65R {
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
            REGION65R::DISABLED => false,
            REGION65R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION65R {
        match value {
            false => REGION65R::DISABLED,
            true => REGION65R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION65R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION65R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION66`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION66R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION66R {
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
            REGION66R::DISABLED => false,
            REGION66R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION66R {
        match value {
            false => REGION66R::DISABLED,
            true => REGION66R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION66R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION66R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION67`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION67R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION67R {
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
            REGION67R::DISABLED => false,
            REGION67R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION67R {
        match value {
            false => REGION67R::DISABLED,
            true => REGION67R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION67R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION67R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION68`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION68R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION68R {
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
            REGION68R::DISABLED => false,
            REGION68R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION68R {
        match value {
            false => REGION68R::DISABLED,
            true => REGION68R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION68R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION68R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION69`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION69R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION69R {
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
            REGION69R::DISABLED => false,
            REGION69R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION69R {
        match value {
            false => REGION69R::DISABLED,
            true => REGION69R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION69R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION69R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION70`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION70R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION70R {
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
            REGION70R::DISABLED => false,
            REGION70R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION70R {
        match value {
            false => REGION70R::DISABLED,
            true => REGION70R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION70R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION70R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION71`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION71R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION71R {
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
            REGION71R::DISABLED => false,
            REGION71R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION71R {
        match value {
            false => REGION71R::DISABLED,
            true => REGION71R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION71R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION71R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION72`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION72R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION72R {
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
            REGION72R::DISABLED => false,
            REGION72R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION72R {
        match value {
            false => REGION72R::DISABLED,
            true => REGION72R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION72R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION72R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION73`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION73R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION73R {
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
            REGION73R::DISABLED => false,
            REGION73R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION73R {
        match value {
            false => REGION73R::DISABLED,
            true => REGION73R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION73R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION73R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION74`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION74R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION74R {
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
            REGION74R::DISABLED => false,
            REGION74R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION74R {
        match value {
            false => REGION74R::DISABLED,
            true => REGION74R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION74R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION74R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION75`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION75R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION75R {
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
            REGION75R::DISABLED => false,
            REGION75R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION75R {
        match value {
            false => REGION75R::DISABLED,
            true => REGION75R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION75R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION75R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION76`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION76R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION76R {
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
            REGION76R::DISABLED => false,
            REGION76R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION76R {
        match value {
            false => REGION76R::DISABLED,
            true => REGION76R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION76R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION76R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION77`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION77R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION77R {
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
            REGION77R::DISABLED => false,
            REGION77R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION77R {
        match value {
            false => REGION77R::DISABLED,
            true => REGION77R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION77R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION77R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION78`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION78R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION78R {
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
            REGION78R::DISABLED => false,
            REGION78R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION78R {
        match value {
            false => REGION78R::DISABLED,
            true => REGION78R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION78R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION78R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION79`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION79R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION79R {
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
            REGION79R::DISABLED => false,
            REGION79R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION79R {
        match value {
            false => REGION79R::DISABLED,
            true => REGION79R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION79R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION79R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION80`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION80R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION80R {
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
            REGION80R::DISABLED => false,
            REGION80R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION80R {
        match value {
            false => REGION80R::DISABLED,
            true => REGION80R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION80R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION80R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION81`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION81R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION81R {
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
            REGION81R::DISABLED => false,
            REGION81R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION81R {
        match value {
            false => REGION81R::DISABLED,
            true => REGION81R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION81R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION81R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION82`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION82R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION82R {
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
            REGION82R::DISABLED => false,
            REGION82R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION82R {
        match value {
            false => REGION82R::DISABLED,
            true => REGION82R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION82R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION82R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION83`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION83R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION83R {
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
            REGION83R::DISABLED => false,
            REGION83R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION83R {
        match value {
            false => REGION83R::DISABLED,
            true => REGION83R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION83R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION83R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION84`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION84R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION84R {
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
            REGION84R::DISABLED => false,
            REGION84R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION84R {
        match value {
            false => REGION84R::DISABLED,
            true => REGION84R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION84R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION84R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION85`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION85R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION85R {
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
            REGION85R::DISABLED => false,
            REGION85R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION85R {
        match value {
            false => REGION85R::DISABLED,
            true => REGION85R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION85R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION85R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION86`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION86R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION86R {
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
            REGION86R::DISABLED => false,
            REGION86R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION86R {
        match value {
            false => REGION86R::DISABLED,
            true => REGION86R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION86R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION86R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION87`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION87R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION87R {
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
            REGION87R::DISABLED => false,
            REGION87R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION87R {
        match value {
            false => REGION87R::DISABLED,
            true => REGION87R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION87R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION87R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION88`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION88R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION88R {
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
            REGION88R::DISABLED => false,
            REGION88R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION88R {
        match value {
            false => REGION88R::DISABLED,
            true => REGION88R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION88R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION88R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION89`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION89R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION89R {
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
            REGION89R::DISABLED => false,
            REGION89R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION89R {
        match value {
            false => REGION89R::DISABLED,
            true => REGION89R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION89R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION89R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION90`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION90R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION90R {
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
            REGION90R::DISABLED => false,
            REGION90R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION90R {
        match value {
            false => REGION90R::DISABLED,
            true => REGION90R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION90R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION90R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION91`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION91R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION91R {
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
            REGION91R::DISABLED => false,
            REGION91R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION91R {
        match value {
            false => REGION91R::DISABLED,
            true => REGION91R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION91R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION91R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION92`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION92R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION92R {
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
            REGION92R::DISABLED => false,
            REGION92R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION92R {
        match value {
            false => REGION92R::DISABLED,
            true => REGION92R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION92R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION92R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION93`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION93R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION93R {
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
            REGION93R::DISABLED => false,
            REGION93R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION93R {
        match value {
            false => REGION93R::DISABLED,
            true => REGION93R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION93R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION93R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION94`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION94R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION94R {
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
            REGION94R::DISABLED => false,
            REGION94R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION94R {
        match value {
            false => REGION94R::DISABLED,
            true => REGION94R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION94R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION94R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION95`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION95R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION95R {
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
            REGION95R::DISABLED => false,
            REGION95R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION95R {
        match value {
            false => REGION95R::DISABLED,
            true => REGION95R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION95R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION95R::ENABLED
    }
}
#[doc = "Values that can be written to the field `REGION64`"]
pub enum REGION64W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION64W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION64W::DISABLED => false,
            REGION64W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION64W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION64W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION64W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION64W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION64W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION65`"]
pub enum REGION65W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION65W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION65W::DISABLED => false,
            REGION65W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION65W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION65W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION65W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION65W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION65W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION66`"]
pub enum REGION66W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION66W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION66W::DISABLED => false,
            REGION66W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION66W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION66W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION66W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION66W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION66W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION67`"]
pub enum REGION67W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION67W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION67W::DISABLED => false,
            REGION67W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION67W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION67W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION67W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION67W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION67W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION68`"]
pub enum REGION68W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION68W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION68W::DISABLED => false,
            REGION68W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION68W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION68W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION68W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION68W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION68W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION69`"]
pub enum REGION69W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION69W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION69W::DISABLED => false,
            REGION69W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION69W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION69W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION69W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION69W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION69W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION70`"]
pub enum REGION70W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION70W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION70W::DISABLED => false,
            REGION70W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION70W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION70W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION70W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION70W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION70W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION71`"]
pub enum REGION71W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION71W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION71W::DISABLED => false,
            REGION71W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION71W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION71W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION71W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION71W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION71W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION72`"]
pub enum REGION72W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION72W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION72W::DISABLED => false,
            REGION72W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION72W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION72W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION72W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION72W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION72W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION73`"]
pub enum REGION73W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION73W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION73W::DISABLED => false,
            REGION73W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION73W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION73W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION73W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION73W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION73W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION74`"]
pub enum REGION74W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION74W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION74W::DISABLED => false,
            REGION74W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION74W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION74W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION74W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION74W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION74W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION75`"]
pub enum REGION75W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION75W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION75W::DISABLED => false,
            REGION75W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION75W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION75W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION75W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION75W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION75W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION76`"]
pub enum REGION76W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION76W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION76W::DISABLED => false,
            REGION76W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION76W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION76W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION76W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION76W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION76W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION77`"]
pub enum REGION77W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION77W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION77W::DISABLED => false,
            REGION77W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION77W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION77W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION77W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION77W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION77W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION78`"]
pub enum REGION78W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION78W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION78W::DISABLED => false,
            REGION78W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION78W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION78W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION78W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION78W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION78W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION79`"]
pub enum REGION79W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION79W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION79W::DISABLED => false,
            REGION79W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION79W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION79W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION79W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION79W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION79W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION80`"]
pub enum REGION80W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION80W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION80W::DISABLED => false,
            REGION80W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION80W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION80W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION80W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION80W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION80W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION81`"]
pub enum REGION81W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION81W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION81W::DISABLED => false,
            REGION81W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION81W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION81W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION81W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION81W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION81W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION82`"]
pub enum REGION82W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION82W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION82W::DISABLED => false,
            REGION82W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION82W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION82W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION82W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION82W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION82W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION83`"]
pub enum REGION83W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION83W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION83W::DISABLED => false,
            REGION83W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION83W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION83W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION83W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION83W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION83W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION84`"]
pub enum REGION84W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION84W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION84W::DISABLED => false,
            REGION84W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION84W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION84W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION84W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION84W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION84W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION85`"]
pub enum REGION85W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION85W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION85W::DISABLED => false,
            REGION85W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION85W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION85W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION85W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION85W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION85W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION86`"]
pub enum REGION86W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION86W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION86W::DISABLED => false,
            REGION86W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION86W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION86W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION86W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION86W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION86W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION87`"]
pub enum REGION87W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION87W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION87W::DISABLED => false,
            REGION87W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION87W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION87W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION87W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION87W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION87W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION88`"]
pub enum REGION88W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION88W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION88W::DISABLED => false,
            REGION88W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION88W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION88W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION88W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION88W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION88W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION89`"]
pub enum REGION89W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION89W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION89W::DISABLED => false,
            REGION89W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION89W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION89W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION89W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION89W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION89W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION90`"]
pub enum REGION90W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION90W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION90W::DISABLED => false,
            REGION90W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION90W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION90W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION90W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION90W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION90W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION91`"]
pub enum REGION91W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION91W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION91W::DISABLED => false,
            REGION91W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION91W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION91W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION91W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION91W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION91W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION92`"]
pub enum REGION92W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION92W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION92W::DISABLED => false,
            REGION92W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION92W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION92W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION92W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION92W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION92W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION93`"]
pub enum REGION93W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION93W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION93W::DISABLED => false,
            REGION93W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION93W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION93W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION93W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION93W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION93W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION94`"]
pub enum REGION94W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION94W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION94W::DISABLED => false,
            REGION94W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION94W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION94W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION94W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION94W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION94W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION95`"]
pub enum REGION95W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION95W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION95W::DISABLED => false,
            REGION95W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION95W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION95W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION95W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION95W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION95W::ENABLED)
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
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline]
    pub fn region64(&self) -> REGION64R {
        REGION64R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline]
    pub fn region65(&self) -> REGION65R {
        REGION65R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline]
    pub fn region66(&self) -> REGION66R {
        REGION66R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline]
    pub fn region67(&self) -> REGION67R {
        REGION67R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline]
    pub fn region68(&self) -> REGION68R {
        REGION68R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline]
    pub fn region69(&self) -> REGION69R {
        REGION69R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline]
    pub fn region70(&self) -> REGION70R {
        REGION70R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline]
    pub fn region71(&self) -> REGION71R {
        REGION71R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline]
    pub fn region72(&self) -> REGION72R {
        REGION72R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline]
    pub fn region73(&self) -> REGION73R {
        REGION73R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline]
    pub fn region74(&self) -> REGION74R {
        REGION74R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline]
    pub fn region75(&self) -> REGION75R {
        REGION75R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline]
    pub fn region76(&self) -> REGION76R {
        REGION76R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline]
    pub fn region77(&self) -> REGION77R {
        REGION77R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline]
    pub fn region78(&self) -> REGION78R {
        REGION78R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline]
    pub fn region79(&self) -> REGION79R {
        REGION79R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline]
    pub fn region80(&self) -> REGION80R {
        REGION80R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline]
    pub fn region81(&self) -> REGION81R {
        REGION81R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline]
    pub fn region82(&self) -> REGION82R {
        REGION82R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline]
    pub fn region83(&self) -> REGION83R {
        REGION83R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline]
    pub fn region84(&self) -> REGION84R {
        REGION84R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline]
    pub fn region85(&self) -> REGION85R {
        REGION85R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline]
    pub fn region86(&self) -> REGION86R {
        REGION86R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline]
    pub fn region87(&self) -> REGION87R {
        REGION87R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline]
    pub fn region88(&self) -> REGION88R {
        REGION88R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline]
    pub fn region89(&self) -> REGION89R {
        REGION89R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline]
    pub fn region90(&self) -> REGION90R {
        REGION90R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline]
    pub fn region91(&self) -> REGION91R {
        REGION91R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline]
    pub fn region92(&self) -> REGION92R {
        REGION92R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline]
    pub fn region93(&self) -> REGION93R {
        REGION93R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline]
    pub fn region94(&self) -> REGION94R {
        REGION94R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline]
    pub fn region95(&self) -> REGION95R {
        REGION95R::_from({
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
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline]
    pub fn region64(&mut self) -> _REGION64W {
        _REGION64W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline]
    pub fn region65(&mut self) -> _REGION65W {
        _REGION65W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline]
    pub fn region66(&mut self) -> _REGION66W {
        _REGION66W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline]
    pub fn region67(&mut self) -> _REGION67W {
        _REGION67W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline]
    pub fn region68(&mut self) -> _REGION68W {
        _REGION68W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline]
    pub fn region69(&mut self) -> _REGION69W {
        _REGION69W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline]
    pub fn region70(&mut self) -> _REGION70W {
        _REGION70W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline]
    pub fn region71(&mut self) -> _REGION71W {
        _REGION71W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline]
    pub fn region72(&mut self) -> _REGION72W {
        _REGION72W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline]
    pub fn region73(&mut self) -> _REGION73W {
        _REGION73W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline]
    pub fn region74(&mut self) -> _REGION74W {
        _REGION74W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline]
    pub fn region75(&mut self) -> _REGION75W {
        _REGION75W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline]
    pub fn region76(&mut self) -> _REGION76W {
        _REGION76W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline]
    pub fn region77(&mut self) -> _REGION77W {
        _REGION77W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline]
    pub fn region78(&mut self) -> _REGION78W {
        _REGION78W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline]
    pub fn region79(&mut self) -> _REGION79W {
        _REGION79W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline]
    pub fn region80(&mut self) -> _REGION80W {
        _REGION80W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline]
    pub fn region81(&mut self) -> _REGION81W {
        _REGION81W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline]
    pub fn region82(&mut self) -> _REGION82W {
        _REGION82W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline]
    pub fn region83(&mut self) -> _REGION83W {
        _REGION83W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline]
    pub fn region84(&mut self) -> _REGION84W {
        _REGION84W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline]
    pub fn region85(&mut self) -> _REGION85W {
        _REGION85W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline]
    pub fn region86(&mut self) -> _REGION86W {
        _REGION86W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline]
    pub fn region87(&mut self) -> _REGION87W {
        _REGION87W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline]
    pub fn region88(&mut self) -> _REGION88W {
        _REGION88W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline]
    pub fn region89(&mut self) -> _REGION89W {
        _REGION89W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline]
    pub fn region90(&mut self) -> _REGION90W {
        _REGION90W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline]
    pub fn region91(&mut self) -> _REGION91W {
        _REGION91W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline]
    pub fn region92(&mut self) -> _REGION92W {
        _REGION92W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline]
    pub fn region93(&mut self) -> _REGION93W {
        _REGION93W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline]
    pub fn region94(&mut self) -> _REGION94W {
        _REGION94W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline]
    pub fn region95(&mut self) -> _REGION95W {
        _REGION95W { w: self }
    }
}
