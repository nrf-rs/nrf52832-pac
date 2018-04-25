#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG1 {
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
#[doc = "Possible values of the field `REGION32`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION32R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION32R {
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
            REGION32R::DISABLED => false,
            REGION32R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION32R {
        match value {
            false => REGION32R::DISABLED,
            true => REGION32R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION32R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION32R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION33`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION33R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION33R {
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
            REGION33R::DISABLED => false,
            REGION33R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION33R {
        match value {
            false => REGION33R::DISABLED,
            true => REGION33R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION33R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION33R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION34`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION34R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION34R {
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
            REGION34R::DISABLED => false,
            REGION34R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION34R {
        match value {
            false => REGION34R::DISABLED,
            true => REGION34R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION34R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION34R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION35`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION35R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION35R {
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
            REGION35R::DISABLED => false,
            REGION35R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION35R {
        match value {
            false => REGION35R::DISABLED,
            true => REGION35R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION35R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION35R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION36`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION36R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION36R {
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
            REGION36R::DISABLED => false,
            REGION36R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION36R {
        match value {
            false => REGION36R::DISABLED,
            true => REGION36R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION36R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION36R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION37`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION37R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION37R {
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
            REGION37R::DISABLED => false,
            REGION37R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION37R {
        match value {
            false => REGION37R::DISABLED,
            true => REGION37R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION37R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION37R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION38`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION38R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION38R {
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
            REGION38R::DISABLED => false,
            REGION38R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION38R {
        match value {
            false => REGION38R::DISABLED,
            true => REGION38R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION38R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION38R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION39`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION39R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION39R {
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
            REGION39R::DISABLED => false,
            REGION39R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION39R {
        match value {
            false => REGION39R::DISABLED,
            true => REGION39R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION39R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION39R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION40`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION40R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION40R {
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
            REGION40R::DISABLED => false,
            REGION40R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION40R {
        match value {
            false => REGION40R::DISABLED,
            true => REGION40R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION40R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION40R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION41`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION41R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION41R {
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
            REGION41R::DISABLED => false,
            REGION41R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION41R {
        match value {
            false => REGION41R::DISABLED,
            true => REGION41R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION41R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION41R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION42`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION42R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION42R {
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
            REGION42R::DISABLED => false,
            REGION42R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION42R {
        match value {
            false => REGION42R::DISABLED,
            true => REGION42R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION42R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION42R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION43`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION43R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION43R {
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
            REGION43R::DISABLED => false,
            REGION43R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION43R {
        match value {
            false => REGION43R::DISABLED,
            true => REGION43R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION43R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION43R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION44`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION44R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION44R {
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
            REGION44R::DISABLED => false,
            REGION44R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION44R {
        match value {
            false => REGION44R::DISABLED,
            true => REGION44R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION44R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION44R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION45R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION45R {
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
            REGION45R::DISABLED => false,
            REGION45R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION45R {
        match value {
            false => REGION45R::DISABLED,
            true => REGION45R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION45R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION45R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION46`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION46R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION46R {
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
            REGION46R::DISABLED => false,
            REGION46R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION46R {
        match value {
            false => REGION46R::DISABLED,
            true => REGION46R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION46R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION46R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION47`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION47R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION47R {
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
            REGION47R::DISABLED => false,
            REGION47R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION47R {
        match value {
            false => REGION47R::DISABLED,
            true => REGION47R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION47R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION47R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION48`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION48R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION48R {
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
            REGION48R::DISABLED => false,
            REGION48R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION48R {
        match value {
            false => REGION48R::DISABLED,
            true => REGION48R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION48R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION48R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION49`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION49R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION49R {
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
            REGION49R::DISABLED => false,
            REGION49R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION49R {
        match value {
            false => REGION49R::DISABLED,
            true => REGION49R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION49R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION49R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION50`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION50R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION50R {
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
            REGION50R::DISABLED => false,
            REGION50R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION50R {
        match value {
            false => REGION50R::DISABLED,
            true => REGION50R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION50R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION50R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION51`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION51R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION51R {
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
            REGION51R::DISABLED => false,
            REGION51R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION51R {
        match value {
            false => REGION51R::DISABLED,
            true => REGION51R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION51R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION51R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION52`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION52R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION52R {
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
            REGION52R::DISABLED => false,
            REGION52R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION52R {
        match value {
            false => REGION52R::DISABLED,
            true => REGION52R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION52R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION52R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION53`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION53R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION53R {
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
            REGION53R::DISABLED => false,
            REGION53R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION53R {
        match value {
            false => REGION53R::DISABLED,
            true => REGION53R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION53R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION53R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION54`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION54R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION54R {
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
            REGION54R::DISABLED => false,
            REGION54R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION54R {
        match value {
            false => REGION54R::DISABLED,
            true => REGION54R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION54R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION54R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION55`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION55R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION55R {
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
            REGION55R::DISABLED => false,
            REGION55R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION55R {
        match value {
            false => REGION55R::DISABLED,
            true => REGION55R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION55R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION55R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION56`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION56R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION56R {
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
            REGION56R::DISABLED => false,
            REGION56R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION56R {
        match value {
            false => REGION56R::DISABLED,
            true => REGION56R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION56R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION56R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION57`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION57R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION57R {
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
            REGION57R::DISABLED => false,
            REGION57R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION57R {
        match value {
            false => REGION57R::DISABLED,
            true => REGION57R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION57R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION57R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION58`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION58R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION58R {
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
            REGION58R::DISABLED => false,
            REGION58R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION58R {
        match value {
            false => REGION58R::DISABLED,
            true => REGION58R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION58R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION58R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION59`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION59R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION59R {
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
            REGION59R::DISABLED => false,
            REGION59R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION59R {
        match value {
            false => REGION59R::DISABLED,
            true => REGION59R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION59R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION59R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION60`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION60R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION60R {
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
            REGION60R::DISABLED => false,
            REGION60R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION60R {
        match value {
            false => REGION60R::DISABLED,
            true => REGION60R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION60R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION60R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION61`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION61R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION61R {
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
            REGION61R::DISABLED => false,
            REGION61R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION61R {
        match value {
            false => REGION61R::DISABLED,
            true => REGION61R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION61R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION61R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION62`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION62R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION62R {
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
            REGION62R::DISABLED => false,
            REGION62R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION62R {
        match value {
            false => REGION62R::DISABLED,
            true => REGION62R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION62R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION62R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION63`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION63R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION63R {
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
            REGION63R::DISABLED => false,
            REGION63R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION63R {
        match value {
            false => REGION63R::DISABLED,
            true => REGION63R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION63R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION63R::ENABLED
    }
}
#[doc = "Values that can be written to the field `REGION32`"]
pub enum REGION32W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION32W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION32W::DISABLED => false,
            REGION32W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION32W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION32W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION32W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION32W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION32W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION33`"]
pub enum REGION33W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION33W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION33W::DISABLED => false,
            REGION33W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION33W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION33W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION33W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION33W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION33W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION34`"]
pub enum REGION34W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION34W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION34W::DISABLED => false,
            REGION34W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION34W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION34W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION34W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION34W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION34W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION35`"]
pub enum REGION35W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION35W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION35W::DISABLED => false,
            REGION35W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION35W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION35W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION35W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION35W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION35W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION36`"]
pub enum REGION36W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION36W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION36W::DISABLED => false,
            REGION36W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION36W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION36W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION36W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION36W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION36W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION37`"]
pub enum REGION37W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION37W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION37W::DISABLED => false,
            REGION37W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION37W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION37W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION37W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION37W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION37W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION38`"]
pub enum REGION38W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION38W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION38W::DISABLED => false,
            REGION38W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION38W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION38W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION38W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION38W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION38W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION39`"]
pub enum REGION39W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION39W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION39W::DISABLED => false,
            REGION39W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION39W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION39W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION39W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION39W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION39W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION40`"]
pub enum REGION40W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION40W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION40W::DISABLED => false,
            REGION40W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION40W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION40W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION40W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION40W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION40W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION41`"]
pub enum REGION41W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION41W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION41W::DISABLED => false,
            REGION41W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION41W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION41W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION41W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION41W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION41W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION42`"]
pub enum REGION42W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION42W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION42W::DISABLED => false,
            REGION42W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION42W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION42W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION42W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION42W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION42W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION43`"]
pub enum REGION43W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION43W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION43W::DISABLED => false,
            REGION43W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION43W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION43W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION43W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION43W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION43W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION44`"]
pub enum REGION44W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION44W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION44W::DISABLED => false,
            REGION44W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION44W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION44W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION44W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION44W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION44W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION45`"]
pub enum REGION45W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION45W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION45W::DISABLED => false,
            REGION45W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION45W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION45W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION45W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION45W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION45W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION46`"]
pub enum REGION46W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION46W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION46W::DISABLED => false,
            REGION46W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION46W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION46W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION46W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION46W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION46W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION47`"]
pub enum REGION47W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION47W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION47W::DISABLED => false,
            REGION47W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION47W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION47W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION47W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION47W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION47W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION48`"]
pub enum REGION48W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION48W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION48W::DISABLED => false,
            REGION48W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION48W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION48W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION48W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION48W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION48W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION49`"]
pub enum REGION49W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION49W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION49W::DISABLED => false,
            REGION49W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION49W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION49W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION49W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION49W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION49W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION50`"]
pub enum REGION50W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION50W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION50W::DISABLED => false,
            REGION50W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION50W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION50W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION50W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION50W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION50W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION51`"]
pub enum REGION51W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION51W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION51W::DISABLED => false,
            REGION51W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION51W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION51W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION51W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION51W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION51W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION52`"]
pub enum REGION52W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION52W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION52W::DISABLED => false,
            REGION52W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION52W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION52W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION52W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION52W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION52W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION53`"]
pub enum REGION53W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION53W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION53W::DISABLED => false,
            REGION53W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION53W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION53W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION53W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION53W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION53W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION54`"]
pub enum REGION54W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION54W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION54W::DISABLED => false,
            REGION54W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION54W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION54W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION54W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION54W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION54W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION55`"]
pub enum REGION55W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION55W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION55W::DISABLED => false,
            REGION55W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION55W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION55W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION55W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION55W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION55W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION56`"]
pub enum REGION56W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION56W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION56W::DISABLED => false,
            REGION56W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION56W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION56W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION56W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION56W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION56W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION57`"]
pub enum REGION57W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION57W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION57W::DISABLED => false,
            REGION57W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION57W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION57W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION57W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION57W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION57W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION58`"]
pub enum REGION58W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION58W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION58W::DISABLED => false,
            REGION58W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION58W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION58W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION58W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION58W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION58W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION59`"]
pub enum REGION59W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION59W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION59W::DISABLED => false,
            REGION59W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION59W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION59W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION59W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION59W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION59W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION60`"]
pub enum REGION60W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION60W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION60W::DISABLED => false,
            REGION60W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION60W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION60W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION60W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION60W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION60W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION61`"]
pub enum REGION61W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION61W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION61W::DISABLED => false,
            REGION61W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION61W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION61W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION61W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION61W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION61W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION62`"]
pub enum REGION62W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION62W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION62W::DISABLED => false,
            REGION62W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION62W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION62W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION62W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION62W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION62W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION63`"]
pub enum REGION63W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION63W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION63W::DISABLED => false,
            REGION63W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION63W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION63W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION63W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION63W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION63W::ENABLED)
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
    #[doc = "Bit 0 - Enable protection for region 32. Write '0' has no effect."]
    #[inline]
    pub fn region32(&self) -> REGION32R {
        REGION32R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable protection for region 33. Write '0' has no effect."]
    #[inline]
    pub fn region33(&self) -> REGION33R {
        REGION33R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable protection for region 34. Write '0' has no effect."]
    #[inline]
    pub fn region34(&self) -> REGION34R {
        REGION34R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable protection for region 35. Write '0' has no effect."]
    #[inline]
    pub fn region35(&self) -> REGION35R {
        REGION35R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable protection for region 36. Write '0' has no effect."]
    #[inline]
    pub fn region36(&self) -> REGION36R {
        REGION36R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable protection for region 37. Write '0' has no effect."]
    #[inline]
    pub fn region37(&self) -> REGION37R {
        REGION37R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable protection for region 38. Write '0' has no effect."]
    #[inline]
    pub fn region38(&self) -> REGION38R {
        REGION38R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable protection for region 39. Write '0' has no effect."]
    #[inline]
    pub fn region39(&self) -> REGION39R {
        REGION39R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable protection for region 40. Write '0' has no effect."]
    #[inline]
    pub fn region40(&self) -> REGION40R {
        REGION40R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable protection for region 41. Write '0' has no effect."]
    #[inline]
    pub fn region41(&self) -> REGION41R {
        REGION41R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable protection for region 42. Write '0' has no effect."]
    #[inline]
    pub fn region42(&self) -> REGION42R {
        REGION42R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable protection for region 43. Write '0' has no effect."]
    #[inline]
    pub fn region43(&self) -> REGION43R {
        REGION43R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable protection for region 44. Write '0' has no effect."]
    #[inline]
    pub fn region44(&self) -> REGION44R {
        REGION44R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable protection for region 45. Write '0' has no effect."]
    #[inline]
    pub fn region45(&self) -> REGION45R {
        REGION45R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable protection for region 46. Write '0' has no effect."]
    #[inline]
    pub fn region46(&self) -> REGION46R {
        REGION46R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable protection for region 47. Write '0' has no effect."]
    #[inline]
    pub fn region47(&self) -> REGION47R {
        REGION47R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable protection for region 48. Write '0' has no effect."]
    #[inline]
    pub fn region48(&self) -> REGION48R {
        REGION48R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable protection for region 49. Write '0' has no effect."]
    #[inline]
    pub fn region49(&self) -> REGION49R {
        REGION49R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable protection for region 50. Write '0' has no effect."]
    #[inline]
    pub fn region50(&self) -> REGION50R {
        REGION50R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable protection for region 51. Write '0' has no effect."]
    #[inline]
    pub fn region51(&self) -> REGION51R {
        REGION51R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable protection for region 52. Write '0' has no effect."]
    #[inline]
    pub fn region52(&self) -> REGION52R {
        REGION52R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enable protection for region 53. Write '0' has no effect."]
    #[inline]
    pub fn region53(&self) -> REGION53R {
        REGION53R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enable protection for region 54. Write '0' has no effect."]
    #[inline]
    pub fn region54(&self) -> REGION54R {
        REGION54R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable protection for region 55. Write '0' has no effect."]
    #[inline]
    pub fn region55(&self) -> REGION55R {
        REGION55R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable protection for region 56. Write '0' has no effect."]
    #[inline]
    pub fn region56(&self) -> REGION56R {
        REGION56R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable protection for region 57. Write '0' has no effect."]
    #[inline]
    pub fn region57(&self) -> REGION57R {
        REGION57R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable protection for region 58. Write '0' has no effect."]
    #[inline]
    pub fn region58(&self) -> REGION58R {
        REGION58R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable protection for region 59. Write '0' has no effect."]
    #[inline]
    pub fn region59(&self) -> REGION59R {
        REGION59R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable protection for region 60. Write '0' has no effect."]
    #[inline]
    pub fn region60(&self) -> REGION60R {
        REGION60R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enable protection for region 61. Write '0' has no effect."]
    #[inline]
    pub fn region61(&self) -> REGION61R {
        REGION61R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable protection for region 62. Write '0' has no effect."]
    #[inline]
    pub fn region62(&self) -> REGION62R {
        REGION62R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable protection for region 63. Write '0' has no effect."]
    #[inline]
    pub fn region63(&self) -> REGION63R {
        REGION63R::_from({
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
    #[doc = "Bit 0 - Enable protection for region 32. Write '0' has no effect."]
    #[inline]
    pub fn region32(&mut self) -> _REGION32W {
        _REGION32W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 33. Write '0' has no effect."]
    #[inline]
    pub fn region33(&mut self) -> _REGION33W {
        _REGION33W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 34. Write '0' has no effect."]
    #[inline]
    pub fn region34(&mut self) -> _REGION34W {
        _REGION34W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 35. Write '0' has no effect."]
    #[inline]
    pub fn region35(&mut self) -> _REGION35W {
        _REGION35W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 36. Write '0' has no effect."]
    #[inline]
    pub fn region36(&mut self) -> _REGION36W {
        _REGION36W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 37. Write '0' has no effect."]
    #[inline]
    pub fn region37(&mut self) -> _REGION37W {
        _REGION37W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 38. Write '0' has no effect."]
    #[inline]
    pub fn region38(&mut self) -> _REGION38W {
        _REGION38W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 39. Write '0' has no effect."]
    #[inline]
    pub fn region39(&mut self) -> _REGION39W {
        _REGION39W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 40. Write '0' has no effect."]
    #[inline]
    pub fn region40(&mut self) -> _REGION40W {
        _REGION40W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 41. Write '0' has no effect."]
    #[inline]
    pub fn region41(&mut self) -> _REGION41W {
        _REGION41W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 42. Write '0' has no effect."]
    #[inline]
    pub fn region42(&mut self) -> _REGION42W {
        _REGION42W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 43. Write '0' has no effect."]
    #[inline]
    pub fn region43(&mut self) -> _REGION43W {
        _REGION43W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 44. Write '0' has no effect."]
    #[inline]
    pub fn region44(&mut self) -> _REGION44W {
        _REGION44W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 45. Write '0' has no effect."]
    #[inline]
    pub fn region45(&mut self) -> _REGION45W {
        _REGION45W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 46. Write '0' has no effect."]
    #[inline]
    pub fn region46(&mut self) -> _REGION46W {
        _REGION46W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 47. Write '0' has no effect."]
    #[inline]
    pub fn region47(&mut self) -> _REGION47W {
        _REGION47W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 48. Write '0' has no effect."]
    #[inline]
    pub fn region48(&mut self) -> _REGION48W {
        _REGION48W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 49. Write '0' has no effect."]
    #[inline]
    pub fn region49(&mut self) -> _REGION49W {
        _REGION49W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 50. Write '0' has no effect."]
    #[inline]
    pub fn region50(&mut self) -> _REGION50W {
        _REGION50W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 51. Write '0' has no effect."]
    #[inline]
    pub fn region51(&mut self) -> _REGION51W {
        _REGION51W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 52. Write '0' has no effect."]
    #[inline]
    pub fn region52(&mut self) -> _REGION52W {
        _REGION52W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 53. Write '0' has no effect."]
    #[inline]
    pub fn region53(&mut self) -> _REGION53W {
        _REGION53W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 54. Write '0' has no effect."]
    #[inline]
    pub fn region54(&mut self) -> _REGION54W {
        _REGION54W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 55. Write '0' has no effect."]
    #[inline]
    pub fn region55(&mut self) -> _REGION55W {
        _REGION55W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 56. Write '0' has no effect."]
    #[inline]
    pub fn region56(&mut self) -> _REGION56W {
        _REGION56W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 57. Write '0' has no effect."]
    #[inline]
    pub fn region57(&mut self) -> _REGION57W {
        _REGION57W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 58. Write '0' has no effect."]
    #[inline]
    pub fn region58(&mut self) -> _REGION58W {
        _REGION58W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 59. Write '0' has no effect."]
    #[inline]
    pub fn region59(&mut self) -> _REGION59W {
        _REGION59W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 60. Write '0' has no effect."]
    #[inline]
    pub fn region60(&mut self) -> _REGION60W {
        _REGION60W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 61. Write '0' has no effect."]
    #[inline]
    pub fn region61(&mut self) -> _REGION61W {
        _REGION61W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 62. Write '0' has no effect."]
    #[inline]
    pub fn region62(&mut self) -> _REGION62W {
        _REGION62W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 63. Write '0' has no effect."]
    #[inline]
    pub fn region63(&mut self) -> _REGION63W {
        _REGION63W { w: self }
    }
}
