#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHENCLR {
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
#[doc = "Possible values of the field `CH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH0R {
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
            CH0R::DISABLED => false,
            CH0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0R {
        match value {
            false => CH0R::DISABLED,
            true => CH0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH0R::ENABLED
    }
}
#[doc = "Possible values of the field `CH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH1R {
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
            CH1R::DISABLED => false,
            CH1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1R {
        match value {
            false => CH1R::DISABLED,
            true => CH1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH1R::ENABLED
    }
}
#[doc = "Possible values of the field `CH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH2R {
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
            CH2R::DISABLED => false,
            CH2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2R {
        match value {
            false => CH2R::DISABLED,
            true => CH2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH2R::ENABLED
    }
}
#[doc = "Possible values of the field `CH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH3R {
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
            CH3R::DISABLED => false,
            CH3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3R {
        match value {
            false => CH3R::DISABLED,
            true => CH3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH3R::ENABLED
    }
}
#[doc = "Possible values of the field `CH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH4R {
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
            CH4R::DISABLED => false,
            CH4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4R {
        match value {
            false => CH4R::DISABLED,
            true => CH4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH4R::ENABLED
    }
}
#[doc = "Possible values of the field `CH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH5R {
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
            CH5R::DISABLED => false,
            CH5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5R {
        match value {
            false => CH5R::DISABLED,
            true => CH5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH5R::ENABLED
    }
}
#[doc = "Possible values of the field `CH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH6R {
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
            CH6R::DISABLED => false,
            CH6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6R {
        match value {
            false => CH6R::DISABLED,
            true => CH6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH6R::ENABLED
    }
}
#[doc = "Possible values of the field `CH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH7R {
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
            CH7R::DISABLED => false,
            CH7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7R {
        match value {
            false => CH7R::DISABLED,
            true => CH7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH7R::ENABLED
    }
}
#[doc = "Possible values of the field `CH8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH8R {
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
            CH8R::DISABLED => false,
            CH8R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8R {
        match value {
            false => CH8R::DISABLED,
            true => CH8R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH8R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH8R::ENABLED
    }
}
#[doc = "Possible values of the field `CH9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH9R {
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
            CH9R::DISABLED => false,
            CH9R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH9R {
        match value {
            false => CH9R::DISABLED,
            true => CH9R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH9R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH9R::ENABLED
    }
}
#[doc = "Possible values of the field `CH10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH10R {
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
            CH10R::DISABLED => false,
            CH10R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH10R {
        match value {
            false => CH10R::DISABLED,
            true => CH10R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH10R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH10R::ENABLED
    }
}
#[doc = "Possible values of the field `CH11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH11R {
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
            CH11R::DISABLED => false,
            CH11R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH11R {
        match value {
            false => CH11R::DISABLED,
            true => CH11R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH11R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH11R::ENABLED
    }
}
#[doc = "Possible values of the field `CH12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH12R {
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
            CH12R::DISABLED => false,
            CH12R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH12R {
        match value {
            false => CH12R::DISABLED,
            true => CH12R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH12R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH12R::ENABLED
    }
}
#[doc = "Possible values of the field `CH13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH13R {
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
            CH13R::DISABLED => false,
            CH13R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH13R {
        match value {
            false => CH13R::DISABLED,
            true => CH13R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH13R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH13R::ENABLED
    }
}
#[doc = "Possible values of the field `CH14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH14R {
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
            CH14R::DISABLED => false,
            CH14R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH14R {
        match value {
            false => CH14R::DISABLED,
            true => CH14R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH14R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH14R::ENABLED
    }
}
#[doc = "Possible values of the field `CH15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH15R {
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
            CH15R::DISABLED => false,
            CH15R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH15R {
        match value {
            false => CH15R::DISABLED,
            true => CH15R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH15R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH15R::ENABLED
    }
}
#[doc = "Possible values of the field `CH20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH20R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH20R {
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
            CH20R::DISABLED => false,
            CH20R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH20R {
        match value {
            false => CH20R::DISABLED,
            true => CH20R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH20R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH20R::ENABLED
    }
}
#[doc = "Possible values of the field `CH21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH21R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH21R {
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
            CH21R::DISABLED => false,
            CH21R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH21R {
        match value {
            false => CH21R::DISABLED,
            true => CH21R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH21R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH21R::ENABLED
    }
}
#[doc = "Possible values of the field `CH22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH22R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH22R {
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
            CH22R::DISABLED => false,
            CH22R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH22R {
        match value {
            false => CH22R::DISABLED,
            true => CH22R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH22R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH22R::ENABLED
    }
}
#[doc = "Possible values of the field `CH23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH23R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH23R {
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
            CH23R::DISABLED => false,
            CH23R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH23R {
        match value {
            false => CH23R::DISABLED,
            true => CH23R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH23R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH23R::ENABLED
    }
}
#[doc = "Possible values of the field `CH24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH24R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH24R {
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
            CH24R::DISABLED => false,
            CH24R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH24R {
        match value {
            false => CH24R::DISABLED,
            true => CH24R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH24R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH24R::ENABLED
    }
}
#[doc = "Possible values of the field `CH25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH25R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH25R {
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
            CH25R::DISABLED => false,
            CH25R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH25R {
        match value {
            false => CH25R::DISABLED,
            true => CH25R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH25R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH25R::ENABLED
    }
}
#[doc = "Possible values of the field `CH26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH26R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH26R {
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
            CH26R::DISABLED => false,
            CH26R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH26R {
        match value {
            false => CH26R::DISABLED,
            true => CH26R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH26R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH26R::ENABLED
    }
}
#[doc = "Possible values of the field `CH27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH27R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH27R {
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
            CH27R::DISABLED => false,
            CH27R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH27R {
        match value {
            false => CH27R::DISABLED,
            true => CH27R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH27R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH27R::ENABLED
    }
}
#[doc = "Possible values of the field `CH28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH28R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH28R {
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
            CH28R::DISABLED => false,
            CH28R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH28R {
        match value {
            false => CH28R::DISABLED,
            true => CH28R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH28R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH28R::ENABLED
    }
}
#[doc = "Possible values of the field `CH29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH29R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH29R {
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
            CH29R::DISABLED => false,
            CH29R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH29R {
        match value {
            false => CH29R::DISABLED,
            true => CH29R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH29R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH29R::ENABLED
    }
}
#[doc = "Possible values of the field `CH30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH30R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH30R {
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
            CH30R::DISABLED => false,
            CH30R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH30R {
        match value {
            false => CH30R::DISABLED,
            true => CH30R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH30R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH30R::ENABLED
    }
}
#[doc = "Possible values of the field `CH31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH31R {
    #[doc = "Channel disabled."]
    DISABLED,
    #[doc = "Channel enabled."]
    ENABLED,
}
impl CH31R {
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
            CH31R::DISABLED => false,
            CH31R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH31R {
        match value {
            false => CH31R::DISABLED,
            true => CH31R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH31R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH31R::ENABLED
    }
}
#[doc = "Values that can be written to the field `CH0`"]
pub enum CH0W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH0W::CLEAR)
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
#[doc = "Values that can be written to the field `CH1`"]
pub enum CH1W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH1W::CLEAR)
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
#[doc = "Values that can be written to the field `CH2`"]
pub enum CH2W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _CH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH2W::CLEAR)
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
#[doc = "Values that can be written to the field `CH3`"]
pub enum CH3W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _CH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH3W::CLEAR)
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
#[doc = "Values that can be written to the field `CH4`"]
pub enum CH4W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _CH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH4W::CLEAR)
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
#[doc = "Values that can be written to the field `CH5`"]
pub enum CH5W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _CH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH5W::CLEAR)
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
#[doc = "Values that can be written to the field `CH6`"]
pub enum CH6W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _CH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH6W::CLEAR)
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
#[doc = "Values that can be written to the field `CH7`"]
pub enum CH7W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _CH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH7W::CLEAR)
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
#[doc = "Values that can be written to the field `CH8`"]
pub enum CH8W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8W<'a> {
    w: &'a mut W,
}
impl<'a> _CH8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH8W::CLEAR)
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
#[doc = "Values that can be written to the field `CH9`"]
pub enum CH9W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH9W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH9W<'a> {
    w: &'a mut W,
}
impl<'a> _CH9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH9W::CLEAR)
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
#[doc = "Values that can be written to the field `CH10`"]
pub enum CH10W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH10W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH10W<'a> {
    w: &'a mut W,
}
impl<'a> _CH10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH10W::CLEAR)
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
#[doc = "Values that can be written to the field `CH11`"]
pub enum CH11W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH11W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH11W<'a> {
    w: &'a mut W,
}
impl<'a> _CH11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH11W::CLEAR)
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
#[doc = "Values that can be written to the field `CH12`"]
pub enum CH12W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH12W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH12W<'a> {
    w: &'a mut W,
}
impl<'a> _CH12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH12W::CLEAR)
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
#[doc = "Values that can be written to the field `CH13`"]
pub enum CH13W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH13W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH13W<'a> {
    w: &'a mut W,
}
impl<'a> _CH13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH13W::CLEAR)
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
#[doc = "Values that can be written to the field `CH14`"]
pub enum CH14W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH14W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH14W<'a> {
    w: &'a mut W,
}
impl<'a> _CH14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH14W::CLEAR)
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
#[doc = "Values that can be written to the field `CH15`"]
pub enum CH15W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH15W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH15W<'a> {
    w: &'a mut W,
}
impl<'a> _CH15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH15W::CLEAR)
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
#[doc = "Values that can be written to the field `CH20`"]
pub enum CH20W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH20W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH20W<'a> {
    w: &'a mut W,
}
impl<'a> _CH20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH20W::CLEAR)
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
#[doc = "Values that can be written to the field `CH21`"]
pub enum CH21W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH21W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH21W<'a> {
    w: &'a mut W,
}
impl<'a> _CH21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH21W::CLEAR)
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
#[doc = "Values that can be written to the field `CH22`"]
pub enum CH22W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH22W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH22W<'a> {
    w: &'a mut W,
}
impl<'a> _CH22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH22W::CLEAR)
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
#[doc = "Values that can be written to the field `CH23`"]
pub enum CH23W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH23W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH23W<'a> {
    w: &'a mut W,
}
impl<'a> _CH23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH23W::CLEAR)
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
#[doc = "Values that can be written to the field `CH24`"]
pub enum CH24W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH24W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH24W<'a> {
    w: &'a mut W,
}
impl<'a> _CH24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH24W::CLEAR)
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
#[doc = "Values that can be written to the field `CH25`"]
pub enum CH25W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH25W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH25W<'a> {
    w: &'a mut W,
}
impl<'a> _CH25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH25W::CLEAR)
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
#[doc = "Values that can be written to the field `CH26`"]
pub enum CH26W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH26W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH26W<'a> {
    w: &'a mut W,
}
impl<'a> _CH26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH26W::CLEAR)
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
#[doc = "Values that can be written to the field `CH27`"]
pub enum CH27W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH27W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH27W<'a> {
    w: &'a mut W,
}
impl<'a> _CH27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH27W::CLEAR)
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
#[doc = "Values that can be written to the field `CH28`"]
pub enum CH28W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH28W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH28W<'a> {
    w: &'a mut W,
}
impl<'a> _CH28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH28W::CLEAR)
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
#[doc = "Values that can be written to the field `CH29`"]
pub enum CH29W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH29W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH29W<'a> {
    w: &'a mut W,
}
impl<'a> _CH29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH29W::CLEAR)
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
#[doc = "Values that can be written to the field `CH30`"]
pub enum CH30W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH30W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH30W<'a> {
    w: &'a mut W,
}
impl<'a> _CH30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH30W::CLEAR)
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
#[doc = "Values that can be written to the field `CH31`"]
pub enum CH31W {
    #[doc = "Disable channel on write."]
    CLEAR,
}
impl CH31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH31W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH31W<'a> {
    w: &'a mut W,
}
impl<'a> _CH31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH31W::CLEAR)
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
    #[doc = "Bit 0 - Disable PPI channel 0."]
    #[inline]
    pub fn ch0(&self) -> CH0R {
        CH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Disable PPI channel 1."]
    #[inline]
    pub fn ch1(&self) -> CH1R {
        CH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Disable PPI channel 2."]
    #[inline]
    pub fn ch2(&self) -> CH2R {
        CH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Disable PPI channel 3."]
    #[inline]
    pub fn ch3(&self) -> CH3R {
        CH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Disable PPI channel 4."]
    #[inline]
    pub fn ch4(&self) -> CH4R {
        CH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Disable PPI channel 5."]
    #[inline]
    pub fn ch5(&self) -> CH5R {
        CH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Disable PPI channel 6."]
    #[inline]
    pub fn ch6(&self) -> CH6R {
        CH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Disable PPI channel 7."]
    #[inline]
    pub fn ch7(&self) -> CH7R {
        CH7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Disable PPI channel 8."]
    #[inline]
    pub fn ch8(&self) -> CH8R {
        CH8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Disable PPI channel 9."]
    #[inline]
    pub fn ch9(&self) -> CH9R {
        CH9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Disable PPI channel 10."]
    #[inline]
    pub fn ch10(&self) -> CH10R {
        CH10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Disable PPI channel 11."]
    #[inline]
    pub fn ch11(&self) -> CH11R {
        CH11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Disable PPI channel 12."]
    #[inline]
    pub fn ch12(&self) -> CH12R {
        CH12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Disable PPI channel 13."]
    #[inline]
    pub fn ch13(&self) -> CH13R {
        CH13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Disable PPI channel 14."]
    #[inline]
    pub fn ch14(&self) -> CH14R {
        CH14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Disable PPI channel 15."]
    #[inline]
    pub fn ch15(&self) -> CH15R {
        CH15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Disable PPI channel 20."]
    #[inline]
    pub fn ch20(&self) -> CH20R {
        CH20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Disable PPI channel 21."]
    #[inline]
    pub fn ch21(&self) -> CH21R {
        CH21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Disable PPI channel 22."]
    #[inline]
    pub fn ch22(&self) -> CH22R {
        CH22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Disable PPI channel 23."]
    #[inline]
    pub fn ch23(&self) -> CH23R {
        CH23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Disable PPI channel 24."]
    #[inline]
    pub fn ch24(&self) -> CH24R {
        CH24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Disable PPI channel 25."]
    #[inline]
    pub fn ch25(&self) -> CH25R {
        CH25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Disable PPI channel 26."]
    #[inline]
    pub fn ch26(&self) -> CH26R {
        CH26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Disable PPI channel 27."]
    #[inline]
    pub fn ch27(&self) -> CH27R {
        CH27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Disable PPI channel 28."]
    #[inline]
    pub fn ch28(&self) -> CH28R {
        CH28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Disable PPI channel 29."]
    #[inline]
    pub fn ch29(&self) -> CH29R {
        CH29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Disable PPI channel 30."]
    #[inline]
    pub fn ch30(&self) -> CH30R {
        CH30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Disable PPI channel 31."]
    #[inline]
    pub fn ch31(&self) -> CH31R {
        CH31R::_from({
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
    #[doc = "Bit 0 - Disable PPI channel 0."]
    #[inline]
    pub fn ch0(&mut self) -> _CH0W {
        _CH0W { w: self }
    }
    #[doc = "Bit 1 - Disable PPI channel 1."]
    #[inline]
    pub fn ch1(&mut self) -> _CH1W {
        _CH1W { w: self }
    }
    #[doc = "Bit 2 - Disable PPI channel 2."]
    #[inline]
    pub fn ch2(&mut self) -> _CH2W {
        _CH2W { w: self }
    }
    #[doc = "Bit 3 - Disable PPI channel 3."]
    #[inline]
    pub fn ch3(&mut self) -> _CH3W {
        _CH3W { w: self }
    }
    #[doc = "Bit 4 - Disable PPI channel 4."]
    #[inline]
    pub fn ch4(&mut self) -> _CH4W {
        _CH4W { w: self }
    }
    #[doc = "Bit 5 - Disable PPI channel 5."]
    #[inline]
    pub fn ch5(&mut self) -> _CH5W {
        _CH5W { w: self }
    }
    #[doc = "Bit 6 - Disable PPI channel 6."]
    #[inline]
    pub fn ch6(&mut self) -> _CH6W {
        _CH6W { w: self }
    }
    #[doc = "Bit 7 - Disable PPI channel 7."]
    #[inline]
    pub fn ch7(&mut self) -> _CH7W {
        _CH7W { w: self }
    }
    #[doc = "Bit 8 - Disable PPI channel 8."]
    #[inline]
    pub fn ch8(&mut self) -> _CH8W {
        _CH8W { w: self }
    }
    #[doc = "Bit 9 - Disable PPI channel 9."]
    #[inline]
    pub fn ch9(&mut self) -> _CH9W {
        _CH9W { w: self }
    }
    #[doc = "Bit 10 - Disable PPI channel 10."]
    #[inline]
    pub fn ch10(&mut self) -> _CH10W {
        _CH10W { w: self }
    }
    #[doc = "Bit 11 - Disable PPI channel 11."]
    #[inline]
    pub fn ch11(&mut self) -> _CH11W {
        _CH11W { w: self }
    }
    #[doc = "Bit 12 - Disable PPI channel 12."]
    #[inline]
    pub fn ch12(&mut self) -> _CH12W {
        _CH12W { w: self }
    }
    #[doc = "Bit 13 - Disable PPI channel 13."]
    #[inline]
    pub fn ch13(&mut self) -> _CH13W {
        _CH13W { w: self }
    }
    #[doc = "Bit 14 - Disable PPI channel 14."]
    #[inline]
    pub fn ch14(&mut self) -> _CH14W {
        _CH14W { w: self }
    }
    #[doc = "Bit 15 - Disable PPI channel 15."]
    #[inline]
    pub fn ch15(&mut self) -> _CH15W {
        _CH15W { w: self }
    }
    #[doc = "Bit 20 - Disable PPI channel 20."]
    #[inline]
    pub fn ch20(&mut self) -> _CH20W {
        _CH20W { w: self }
    }
    #[doc = "Bit 21 - Disable PPI channel 21."]
    #[inline]
    pub fn ch21(&mut self) -> _CH21W {
        _CH21W { w: self }
    }
    #[doc = "Bit 22 - Disable PPI channel 22."]
    #[inline]
    pub fn ch22(&mut self) -> _CH22W {
        _CH22W { w: self }
    }
    #[doc = "Bit 23 - Disable PPI channel 23."]
    #[inline]
    pub fn ch23(&mut self) -> _CH23W {
        _CH23W { w: self }
    }
    #[doc = "Bit 24 - Disable PPI channel 24."]
    #[inline]
    pub fn ch24(&mut self) -> _CH24W {
        _CH24W { w: self }
    }
    #[doc = "Bit 25 - Disable PPI channel 25."]
    #[inline]
    pub fn ch25(&mut self) -> _CH25W {
        _CH25W { w: self }
    }
    #[doc = "Bit 26 - Disable PPI channel 26."]
    #[inline]
    pub fn ch26(&mut self) -> _CH26W {
        _CH26W { w: self }
    }
    #[doc = "Bit 27 - Disable PPI channel 27."]
    #[inline]
    pub fn ch27(&mut self) -> _CH27W {
        _CH27W { w: self }
    }
    #[doc = "Bit 28 - Disable PPI channel 28."]
    #[inline]
    pub fn ch28(&mut self) -> _CH28W {
        _CH28W { w: self }
    }
    #[doc = "Bit 29 - Disable PPI channel 29."]
    #[inline]
    pub fn ch29(&mut self) -> _CH29W {
        _CH29W { w: self }
    }
    #[doc = "Bit 30 - Disable PPI channel 30."]
    #[inline]
    pub fn ch30(&mut self) -> _CH30W {
        _CH30W { w: self }
    }
    #[doc = "Bit 31 - Disable PPI channel 31."]
    #[inline]
    pub fn ch31(&mut self) -> _CH31W {
        _CH31W { w: self }
    }
}
