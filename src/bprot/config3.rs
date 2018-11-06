#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG3 {
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
#[doc = "Possible values of the field `REGION96`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION96R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION96R {
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
            REGION96R::DISABLED => false,
            REGION96R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION96R {
        match value {
            false => REGION96R::DISABLED,
            true => REGION96R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION96R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION96R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION97`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION97R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION97R {
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
            REGION97R::DISABLED => false,
            REGION97R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION97R {
        match value {
            false => REGION97R::DISABLED,
            true => REGION97R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION97R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION97R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION98`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION98R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION98R {
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
            REGION98R::DISABLED => false,
            REGION98R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION98R {
        match value {
            false => REGION98R::DISABLED,
            true => REGION98R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION98R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION98R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION99`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION99R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION99R {
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
            REGION99R::DISABLED => false,
            REGION99R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION99R {
        match value {
            false => REGION99R::DISABLED,
            true => REGION99R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION99R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION99R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION100`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION100R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION100R {
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
            REGION100R::DISABLED => false,
            REGION100R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION100R {
        match value {
            false => REGION100R::DISABLED,
            true => REGION100R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION100R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION100R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION101`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION101R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION101R {
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
            REGION101R::DISABLED => false,
            REGION101R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION101R {
        match value {
            false => REGION101R::DISABLED,
            true => REGION101R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION101R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION101R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION102`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION102R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION102R {
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
            REGION102R::DISABLED => false,
            REGION102R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION102R {
        match value {
            false => REGION102R::DISABLED,
            true => REGION102R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION102R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION102R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION103`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION103R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION103R {
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
            REGION103R::DISABLED => false,
            REGION103R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION103R {
        match value {
            false => REGION103R::DISABLED,
            true => REGION103R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION103R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION103R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION104`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION104R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION104R {
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
            REGION104R::DISABLED => false,
            REGION104R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION104R {
        match value {
            false => REGION104R::DISABLED,
            true => REGION104R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION104R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION104R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION105`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION105R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION105R {
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
            REGION105R::DISABLED => false,
            REGION105R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION105R {
        match value {
            false => REGION105R::DISABLED,
            true => REGION105R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION105R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION105R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION106`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION106R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION106R {
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
            REGION106R::DISABLED => false,
            REGION106R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION106R {
        match value {
            false => REGION106R::DISABLED,
            true => REGION106R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION106R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION106R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION107`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION107R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION107R {
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
            REGION107R::DISABLED => false,
            REGION107R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION107R {
        match value {
            false => REGION107R::DISABLED,
            true => REGION107R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION107R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION107R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION108`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION108R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION108R {
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
            REGION108R::DISABLED => false,
            REGION108R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION108R {
        match value {
            false => REGION108R::DISABLED,
            true => REGION108R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION108R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION108R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION109`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION109R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION109R {
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
            REGION109R::DISABLED => false,
            REGION109R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION109R {
        match value {
            false => REGION109R::DISABLED,
            true => REGION109R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION109R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION109R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION110`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION110R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION110R {
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
            REGION110R::DISABLED => false,
            REGION110R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION110R {
        match value {
            false => REGION110R::DISABLED,
            true => REGION110R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION110R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION110R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION111`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION111R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION111R {
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
            REGION111R::DISABLED => false,
            REGION111R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION111R {
        match value {
            false => REGION111R::DISABLED,
            true => REGION111R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION111R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION111R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION112`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION112R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION112R {
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
            REGION112R::DISABLED => false,
            REGION112R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION112R {
        match value {
            false => REGION112R::DISABLED,
            true => REGION112R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION112R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION112R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION113`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION113R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION113R {
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
            REGION113R::DISABLED => false,
            REGION113R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION113R {
        match value {
            false => REGION113R::DISABLED,
            true => REGION113R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION113R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION113R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION114`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION114R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION114R {
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
            REGION114R::DISABLED => false,
            REGION114R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION114R {
        match value {
            false => REGION114R::DISABLED,
            true => REGION114R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION114R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION114R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION115`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION115R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION115R {
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
            REGION115R::DISABLED => false,
            REGION115R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION115R {
        match value {
            false => REGION115R::DISABLED,
            true => REGION115R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION115R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION115R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION116`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION116R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION116R {
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
            REGION116R::DISABLED => false,
            REGION116R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION116R {
        match value {
            false => REGION116R::DISABLED,
            true => REGION116R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION116R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION116R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION117`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION117R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION117R {
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
            REGION117R::DISABLED => false,
            REGION117R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION117R {
        match value {
            false => REGION117R::DISABLED,
            true => REGION117R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION117R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION117R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION118`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION118R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION118R {
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
            REGION118R::DISABLED => false,
            REGION118R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION118R {
        match value {
            false => REGION118R::DISABLED,
            true => REGION118R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION118R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION118R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION119`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION119R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION119R {
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
            REGION119R::DISABLED => false,
            REGION119R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION119R {
        match value {
            false => REGION119R::DISABLED,
            true => REGION119R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION119R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION119R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION120`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION120R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION120R {
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
            REGION120R::DISABLED => false,
            REGION120R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION120R {
        match value {
            false => REGION120R::DISABLED,
            true => REGION120R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION120R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION120R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION121`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION121R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION121R {
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
            REGION121R::DISABLED => false,
            REGION121R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION121R {
        match value {
            false => REGION121R::DISABLED,
            true => REGION121R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION121R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION121R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION122`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION122R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION122R {
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
            REGION122R::DISABLED => false,
            REGION122R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION122R {
        match value {
            false => REGION122R::DISABLED,
            true => REGION122R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION122R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION122R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION123`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION123R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION123R {
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
            REGION123R::DISABLED => false,
            REGION123R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION123R {
        match value {
            false => REGION123R::DISABLED,
            true => REGION123R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION123R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION123R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION124`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION124R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION124R {
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
            REGION124R::DISABLED => false,
            REGION124R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION124R {
        match value {
            false => REGION124R::DISABLED,
            true => REGION124R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION124R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION124R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION125`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION125R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION125R {
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
            REGION125R::DISABLED => false,
            REGION125R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION125R {
        match value {
            false => REGION125R::DISABLED,
            true => REGION125R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION125R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION125R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION126`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION126R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION126R {
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
            REGION126R::DISABLED => false,
            REGION126R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION126R {
        match value {
            false => REGION126R::DISABLED,
            true => REGION126R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION126R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION126R::ENABLED
    }
}
#[doc = "Possible values of the field `REGION127`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION127R {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION127R {
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
            REGION127R::DISABLED => false,
            REGION127R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGION127R {
        match value {
            false => REGION127R::DISABLED,
            true => REGION127R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REGION127R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REGION127R::ENABLED
    }
}
#[doc = "Values that can be written to the field `REGION96`"]
pub enum REGION96W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION96W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION96W::DISABLED => false,
            REGION96W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION96W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION96W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION96W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION96W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION96W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION97`"]
pub enum REGION97W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION97W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION97W::DISABLED => false,
            REGION97W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION97W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION97W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION97W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION97W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION97W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION98`"]
pub enum REGION98W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION98W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION98W::DISABLED => false,
            REGION98W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION98W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION98W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION98W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION98W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION98W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION99`"]
pub enum REGION99W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION99W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION99W::DISABLED => false,
            REGION99W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION99W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION99W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION99W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION99W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION99W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION100`"]
pub enum REGION100W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION100W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION100W::DISABLED => false,
            REGION100W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION100W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION100W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION100W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION100W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION100W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION101`"]
pub enum REGION101W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION101W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION101W::DISABLED => false,
            REGION101W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION101W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION101W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION101W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION101W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION101W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION102`"]
pub enum REGION102W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION102W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION102W::DISABLED => false,
            REGION102W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION102W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION102W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION102W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION102W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION102W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION103`"]
pub enum REGION103W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION103W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION103W::DISABLED => false,
            REGION103W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION103W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION103W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION103W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION103W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION103W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION104`"]
pub enum REGION104W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION104W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION104W::DISABLED => false,
            REGION104W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION104W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION104W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION104W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION104W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION104W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION105`"]
pub enum REGION105W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION105W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION105W::DISABLED => false,
            REGION105W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION105W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION105W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION105W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION105W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION105W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION106`"]
pub enum REGION106W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION106W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION106W::DISABLED => false,
            REGION106W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION106W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION106W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION106W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION106W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION106W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION107`"]
pub enum REGION107W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION107W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION107W::DISABLED => false,
            REGION107W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION107W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION107W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION107W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION107W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION107W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION108`"]
pub enum REGION108W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION108W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION108W::DISABLED => false,
            REGION108W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION108W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION108W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION108W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION108W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION108W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION109`"]
pub enum REGION109W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION109W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION109W::DISABLED => false,
            REGION109W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION109W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION109W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION109W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION109W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION109W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION110`"]
pub enum REGION110W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION110W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION110W::DISABLED => false,
            REGION110W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION110W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION110W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION110W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION110W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION110W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION111`"]
pub enum REGION111W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION111W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION111W::DISABLED => false,
            REGION111W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION111W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION111W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION111W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION111W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION111W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION112`"]
pub enum REGION112W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION112W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION112W::DISABLED => false,
            REGION112W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION112W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION112W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION112W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION112W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION112W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION113`"]
pub enum REGION113W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION113W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION113W::DISABLED => false,
            REGION113W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION113W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION113W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION113W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION113W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION113W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION114`"]
pub enum REGION114W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION114W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION114W::DISABLED => false,
            REGION114W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION114W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION114W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION114W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION114W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION114W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION115`"]
pub enum REGION115W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION115W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION115W::DISABLED => false,
            REGION115W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION115W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION115W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION115W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION115W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION115W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION116`"]
pub enum REGION116W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION116W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION116W::DISABLED => false,
            REGION116W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION116W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION116W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION116W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION116W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION116W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION117`"]
pub enum REGION117W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION117W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION117W::DISABLED => false,
            REGION117W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION117W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION117W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION117W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION117W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION117W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION118`"]
pub enum REGION118W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION118W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION118W::DISABLED => false,
            REGION118W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION118W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION118W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION118W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION118W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION118W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION119`"]
pub enum REGION119W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION119W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION119W::DISABLED => false,
            REGION119W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION119W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION119W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION119W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION119W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION119W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION120`"]
pub enum REGION120W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION120W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION120W::DISABLED => false,
            REGION120W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION120W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION120W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION120W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION120W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION120W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION121`"]
pub enum REGION121W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION121W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION121W::DISABLED => false,
            REGION121W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION121W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION121W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION121W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION121W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION121W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION122`"]
pub enum REGION122W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION122W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION122W::DISABLED => false,
            REGION122W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION122W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION122W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION122W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION122W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION122W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION123`"]
pub enum REGION123W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION123W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION123W::DISABLED => false,
            REGION123W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION123W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION123W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION123W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION123W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION123W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION124`"]
pub enum REGION124W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION124W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION124W::DISABLED => false,
            REGION124W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION124W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION124W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION124W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION124W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION124W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION125`"]
pub enum REGION125W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION125W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION125W::DISABLED => false,
            REGION125W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION125W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION125W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION125W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION125W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION125W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION126`"]
pub enum REGION126W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION126W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION126W::DISABLED => false,
            REGION126W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION126W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION126W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION126W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION126W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION126W::ENABLED)
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
#[doc = "Values that can be written to the field `REGION127`"]
pub enum REGION127W {
    #[doc = "Protection disabled"]
    DISABLED,
    #[doc = "Protection enabled"]
    ENABLED,
}
impl REGION127W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGION127W::DISABLED => false,
            REGION127W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGION127W<'a> {
    w: &'a mut W,
}
impl<'a> _REGION127W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGION127W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION127W::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION127W::ENABLED)
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
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline]
    pub fn region96(&self) -> REGION96R {
        REGION96R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline]
    pub fn region97(&self) -> REGION97R {
        REGION97R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline]
    pub fn region98(&self) -> REGION98R {
        REGION98R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline]
    pub fn region99(&self) -> REGION99R {
        REGION99R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline]
    pub fn region100(&self) -> REGION100R {
        REGION100R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline]
    pub fn region101(&self) -> REGION101R {
        REGION101R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline]
    pub fn region102(&self) -> REGION102R {
        REGION102R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline]
    pub fn region103(&self) -> REGION103R {
        REGION103R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline]
    pub fn region104(&self) -> REGION104R {
        REGION104R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline]
    pub fn region105(&self) -> REGION105R {
        REGION105R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline]
    pub fn region106(&self) -> REGION106R {
        REGION106R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline]
    pub fn region107(&self) -> REGION107R {
        REGION107R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline]
    pub fn region108(&self) -> REGION108R {
        REGION108R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline]
    pub fn region109(&self) -> REGION109R {
        REGION109R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline]
    pub fn region110(&self) -> REGION110R {
        REGION110R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline]
    pub fn region111(&self) -> REGION111R {
        REGION111R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline]
    pub fn region112(&self) -> REGION112R {
        REGION112R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline]
    pub fn region113(&self) -> REGION113R {
        REGION113R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline]
    pub fn region114(&self) -> REGION114R {
        REGION114R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline]
    pub fn region115(&self) -> REGION115R {
        REGION115R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline]
    pub fn region116(&self) -> REGION116R {
        REGION116R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline]
    pub fn region117(&self) -> REGION117R {
        REGION117R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline]
    pub fn region118(&self) -> REGION118R {
        REGION118R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline]
    pub fn region119(&self) -> REGION119R {
        REGION119R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline]
    pub fn region120(&self) -> REGION120R {
        REGION120R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline]
    pub fn region121(&self) -> REGION121R {
        REGION121R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline]
    pub fn region122(&self) -> REGION122R {
        REGION122R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline]
    pub fn region123(&self) -> REGION123R {
        REGION123R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline]
    pub fn region124(&self) -> REGION124R {
        REGION124R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline]
    pub fn region125(&self) -> REGION125R {
        REGION125R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline]
    pub fn region126(&self) -> REGION126R {
        REGION126R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline]
    pub fn region127(&self) -> REGION127R {
        REGION127R::_from({
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
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline]
    pub fn region96(&mut self) -> _REGION96W {
        _REGION96W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline]
    pub fn region97(&mut self) -> _REGION97W {
        _REGION97W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline]
    pub fn region98(&mut self) -> _REGION98W {
        _REGION98W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline]
    pub fn region99(&mut self) -> _REGION99W {
        _REGION99W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline]
    pub fn region100(&mut self) -> _REGION100W {
        _REGION100W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline]
    pub fn region101(&mut self) -> _REGION101W {
        _REGION101W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline]
    pub fn region102(&mut self) -> _REGION102W {
        _REGION102W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline]
    pub fn region103(&mut self) -> _REGION103W {
        _REGION103W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline]
    pub fn region104(&mut self) -> _REGION104W {
        _REGION104W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline]
    pub fn region105(&mut self) -> _REGION105W {
        _REGION105W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline]
    pub fn region106(&mut self) -> _REGION106W {
        _REGION106W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline]
    pub fn region107(&mut self) -> _REGION107W {
        _REGION107W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline]
    pub fn region108(&mut self) -> _REGION108W {
        _REGION108W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline]
    pub fn region109(&mut self) -> _REGION109W {
        _REGION109W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline]
    pub fn region110(&mut self) -> _REGION110W {
        _REGION110W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline]
    pub fn region111(&mut self) -> _REGION111W {
        _REGION111W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline]
    pub fn region112(&mut self) -> _REGION112W {
        _REGION112W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline]
    pub fn region113(&mut self) -> _REGION113W {
        _REGION113W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline]
    pub fn region114(&mut self) -> _REGION114W {
        _REGION114W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline]
    pub fn region115(&mut self) -> _REGION115W {
        _REGION115W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline]
    pub fn region116(&mut self) -> _REGION116W {
        _REGION116W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline]
    pub fn region117(&mut self) -> _REGION117W {
        _REGION117W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline]
    pub fn region118(&mut self) -> _REGION118W {
        _REGION118W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline]
    pub fn region119(&mut self) -> _REGION119W {
        _REGION119W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline]
    pub fn region120(&mut self) -> _REGION120W {
        _REGION120W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline]
    pub fn region121(&mut self) -> _REGION121W {
        _REGION121W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline]
    pub fn region122(&mut self) -> _REGION122W {
        _REGION122W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline]
    pub fn region123(&mut self) -> _REGION123W {
        _REGION123W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline]
    pub fn region124(&mut self) -> _REGION124W {
        _REGION124W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline]
    pub fn region125(&mut self) -> _REGION125W {
        _REGION125W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline]
    pub fn region126(&mut self) -> _REGION126W {
        _REGION126W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline]
    pub fn region127(&mut self) -> _REGION127W {
        _REGION127W { w: self }
    }
}
