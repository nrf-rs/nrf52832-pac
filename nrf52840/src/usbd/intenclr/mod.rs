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
#[doc = "Possible values of the field `USBRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESETR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl USBRESETR {
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
            USBRESETR::DISABLED => false,
            USBRESETR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBRESETR {
        match value {
            false => USBRESETR::DISABLED,
            true => USBRESETR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == USBRESETR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == USBRESETR::ENABLED
    }
}
#[doc = "Possible values of the field `STARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl STARTEDR {
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
            STARTEDR::DISABLED => false,
            STARTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTEDR {
        match value {
            false => STARTEDR::DISABLED,
            true => STARTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STARTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == STARTEDR::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN0R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPIN0R {
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
            ENDEPIN0R::DISABLED => false,
            ENDEPIN0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPIN0R {
        match value {
            false => ENDEPIN0R::DISABLED,
            true => ENDEPIN0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN0R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN1R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPIN1R {
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
            ENDEPIN1R::DISABLED => false,
            ENDEPIN1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPIN1R {
        match value {
            false => ENDEPIN1R::DISABLED,
            true => ENDEPIN1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN1R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN2R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPIN2R {
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
            ENDEPIN2R::DISABLED => false,
            ENDEPIN2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPIN2R {
        match value {
            false => ENDEPIN2R::DISABLED,
            true => ENDEPIN2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN2R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN3R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPIN3R {
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
            ENDEPIN3R::DISABLED => false,
            ENDEPIN3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPIN3R {
        match value {
            false => ENDEPIN3R::DISABLED,
            true => ENDEPIN3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN3R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPIN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN4R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPIN4R {
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
            ENDEPIN4R::DISABLED => false,
            ENDEPIN4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPIN4R {
        match value {
            false => ENDEPIN4R::DISABLED,
            true => ENDEPIN4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN4R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPIN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN5R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPIN5R {
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
            ENDEPIN5R::DISABLED => false,
            ENDEPIN5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPIN5R {
        match value {
            false => ENDEPIN5R::DISABLED,
            true => ENDEPIN5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN5R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPIN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN6R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPIN6R {
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
            ENDEPIN6R::DISABLED => false,
            ENDEPIN6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPIN6R {
        match value {
            false => ENDEPIN6R::DISABLED,
            true => ENDEPIN6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN6R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPIN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN7R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPIN7R {
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
            ENDEPIN7R::DISABLED => false,
            ENDEPIN7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPIN7R {
        match value {
            false => ENDEPIN7R::DISABLED,
            true => ENDEPIN7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN7R::ENABLED
    }
}
#[doc = "Possible values of the field `EP0DATADONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONER {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl EP0DATADONER {
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
            EP0DATADONER::DISABLED => false,
            EP0DATADONER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP0DATADONER {
        match value {
            false => EP0DATADONER::DISABLED,
            true => EP0DATADONER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONER::ENABLED
    }
}
#[doc = "Possible values of the field `ENDISOIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOINR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDISOINR {
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
            ENDISOINR::DISABLED => false,
            ENDISOINR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDISOINR {
        match value {
            false => ENDISOINR::DISABLED,
            true => ENDISOINR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDISOINR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDISOINR::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPOUT0R {
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
            ENDEPOUT0R::DISABLED => false,
            ENDEPOUT0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT0R {
        match value {
            false => ENDEPOUT0R::DISABLED,
            true => ENDEPOUT0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT0R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT1R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPOUT1R {
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
            ENDEPOUT1R::DISABLED => false,
            ENDEPOUT1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT1R {
        match value {
            false => ENDEPOUT1R::DISABLED,
            true => ENDEPOUT1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT1R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT2R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPOUT2R {
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
            ENDEPOUT2R::DISABLED => false,
            ENDEPOUT2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT2R {
        match value {
            false => ENDEPOUT2R::DISABLED,
            true => ENDEPOUT2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT2R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT3R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPOUT3R {
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
            ENDEPOUT3R::DISABLED => false,
            ENDEPOUT3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT3R {
        match value {
            false => ENDEPOUT3R::DISABLED,
            true => ENDEPOUT3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT3R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT4R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPOUT4R {
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
            ENDEPOUT4R::DISABLED => false,
            ENDEPOUT4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT4R {
        match value {
            false => ENDEPOUT4R::DISABLED,
            true => ENDEPOUT4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT4R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT5R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPOUT5R {
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
            ENDEPOUT5R::DISABLED => false,
            ENDEPOUT5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT5R {
        match value {
            false => ENDEPOUT5R::DISABLED,
            true => ENDEPOUT5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT5R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT6R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPOUT6R {
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
            ENDEPOUT6R::DISABLED => false,
            ENDEPOUT6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT6R {
        match value {
            false => ENDEPOUT6R::DISABLED,
            true => ENDEPOUT6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT6R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT7R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDEPOUT7R {
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
            ENDEPOUT7R::DISABLED => false,
            ENDEPOUT7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT7R {
        match value {
            false => ENDEPOUT7R::DISABLED,
            true => ENDEPOUT7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT7R::ENABLED
    }
}
#[doc = "Possible values of the field `ENDISOOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOOUTR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDISOOUTR {
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
            ENDISOOUTR::DISABLED => false,
            ENDISOOUTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDISOOUTR {
        match value {
            false => ENDISOOUTR::DISABLED,
            true => ENDISOOUTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDISOOUTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDISOOUTR::ENABLED
    }
}
#[doc = "Possible values of the field `SOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SOFR {
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
            SOFR::DISABLED => false,
            SOFR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFR {
        match value {
            false => SOFR::DISABLED,
            true => SOFR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SOFR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SOFR::ENABLED
    }
}
#[doc = "Possible values of the field `USBEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBEVENTR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl USBEVENTR {
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
            USBEVENTR::DISABLED => false,
            USBEVENTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBEVENTR {
        match value {
            false => USBEVENTR::DISABLED,
            true => USBEVENTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == USBEVENTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == USBEVENTR::ENABLED
    }
}
#[doc = "Possible values of the field `EP0SETUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0SETUPR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl EP0SETUPR {
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
            EP0SETUPR::DISABLED => false,
            EP0SETUPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP0SETUPR {
        match value {
            false => EP0SETUPR::DISABLED,
            true => EP0SETUPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EP0SETUPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EP0SETUPR::ENABLED
    }
}
#[doc = "Possible values of the field `EPDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDATAR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl EPDATAR {
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
            EPDATAR::DISABLED => false,
            EPDATAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPDATAR {
        match value {
            false => EPDATAR::DISABLED,
            true => EPDATAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EPDATAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EPDATAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `USBRESET`"]
pub enum USBRESETW {
    #[doc = "Disable"]
    CLEAR,
}
impl USBRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBRESETW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _USBRESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBRESETW::CLEAR)
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
#[doc = "Values that can be written to the field `STARTED`"]
pub enum STARTEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl STARTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(STARTEDW::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPIN0`"]
pub enum ENDEPIN0W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPIN0W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN0W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPIN1`"]
pub enum ENDEPIN1W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPIN1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPIN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN1W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPIN2`"]
pub enum ENDEPIN2W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPIN2W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPIN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN2W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPIN3`"]
pub enum ENDEPIN3W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPIN3W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPIN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN3W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPIN4`"]
pub enum ENDEPIN4W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPIN4W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPIN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPIN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN4W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPIN5`"]
pub enum ENDEPIN5W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPIN5W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPIN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPIN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN5W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPIN6`"]
pub enum ENDEPIN6W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPIN6W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPIN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPIN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN6W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPIN7`"]
pub enum ENDEPIN7W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPIN7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPIN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPIN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN7W::CLEAR)
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
#[doc = "Values that can be written to the field `EP0DATADONE`"]
pub enum EP0DATADONEW {
    #[doc = "Disable"]
    CLEAR,
}
impl EP0DATADONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EP0DATADONEW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EP0DATADONEW<'a> {
    w: &'a mut W,
}
impl<'a> _EP0DATADONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP0DATADONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0DATADONEW::CLEAR)
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
#[doc = "Values that can be written to the field `ENDISOIN`"]
pub enum ENDISOINW {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDISOINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDISOINW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDISOINW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDISOINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDISOINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDISOINW::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPOUT0`"]
pub enum ENDEPOUT0W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPOUT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT0W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT0W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPOUT1`"]
pub enum ENDEPOUT1W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPOUT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT1W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPOUT2`"]
pub enum ENDEPOUT2W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPOUT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT2W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT2W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPOUT3`"]
pub enum ENDEPOUT3W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPOUT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT3W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT3W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPOUT4`"]
pub enum ENDEPOUT4W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPOUT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT4W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT4W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT4W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPOUT5`"]
pub enum ENDEPOUT5W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPOUT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT5W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT5W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT5W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPOUT6`"]
pub enum ENDEPOUT6W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPOUT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT6W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT6W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDEPOUT7`"]
pub enum ENDEPOUT7W {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDEPOUT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT7W::CLEAR)
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
#[doc = "Values that can be written to the field `ENDISOOUT`"]
pub enum ENDISOOUTW {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDISOOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDISOOUTW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDISOOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDISOOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDISOOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDISOOUTW::CLEAR)
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
#[doc = "Values that can be written to the field `SOF`"]
pub enum SOFW {
    #[doc = "Disable"]
    CLEAR,
}
impl SOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SOFW::CLEAR)
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
#[doc = "Values that can be written to the field `USBEVENT`"]
pub enum USBEVENTW {
    #[doc = "Disable"]
    CLEAR,
}
impl USBEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBEVENTW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _USBEVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBEVENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBEVENTW::CLEAR)
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
#[doc = "Values that can be written to the field `EP0SETUP`"]
pub enum EP0SETUPW {
    #[doc = "Disable"]
    CLEAR,
}
impl EP0SETUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EP0SETUPW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EP0SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EP0SETUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP0SETUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0SETUPW::CLEAR)
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
#[doc = "Values that can be written to the field `EPDATA`"]
pub enum EPDATAW {
    #[doc = "Disable"]
    CLEAR,
}
impl EPDATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPDATAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPDATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EPDATAW::CLEAR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write '1' to disable interrupt for USBRESET event"]
    #[inline]
    pub fn usbreset(&self) -> USBRESETR {
        USBRESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for STARTED event"]
    #[inline]
    pub fn started(&self) -> STARTEDR {
        STARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for ENDEPIN[0] event"]
    #[inline]
    pub fn endepin0(&self) -> ENDEPIN0R {
        ENDEPIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for ENDEPIN[1] event"]
    #[inline]
    pub fn endepin1(&self) -> ENDEPIN1R {
        ENDEPIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for ENDEPIN[2] event"]
    #[inline]
    pub fn endepin2(&self) -> ENDEPIN2R {
        ENDEPIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for ENDEPIN[3] event"]
    #[inline]
    pub fn endepin3(&self) -> ENDEPIN3R {
        ENDEPIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for ENDEPIN[4] event"]
    #[inline]
    pub fn endepin4(&self) -> ENDEPIN4R {
        ENDEPIN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for ENDEPIN[5] event"]
    #[inline]
    pub fn endepin5(&self) -> ENDEPIN5R {
        ENDEPIN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for ENDEPIN[6] event"]
    #[inline]
    pub fn endepin6(&self) -> ENDEPIN6R {
        ENDEPIN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for ENDEPIN[7] event"]
    #[inline]
    pub fn endepin7(&self) -> ENDEPIN7R {
        ENDEPIN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for EP0DATADONE event"]
    #[inline]
    pub fn ep0datadone(&self) -> EP0DATADONER {
        EP0DATADONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for ENDISOIN event"]
    #[inline]
    pub fn endisoin(&self) -> ENDISOINR {
        ENDISOINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for ENDEPOUT[0] event"]
    #[inline]
    pub fn endepout0(&self) -> ENDEPOUT0R {
        ENDEPOUT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for ENDEPOUT[1] event"]
    #[inline]
    pub fn endepout1(&self) -> ENDEPOUT1R {
        ENDEPOUT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for ENDEPOUT[2] event"]
    #[inline]
    pub fn endepout2(&self) -> ENDEPOUT2R {
        ENDEPOUT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for ENDEPOUT[3] event"]
    #[inline]
    pub fn endepout3(&self) -> ENDEPOUT3R {
        ENDEPOUT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for ENDEPOUT[4] event"]
    #[inline]
    pub fn endepout4(&self) -> ENDEPOUT4R {
        ENDEPOUT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for ENDEPOUT[5] event"]
    #[inline]
    pub fn endepout5(&self) -> ENDEPOUT5R {
        ENDEPOUT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for ENDEPOUT[6] event"]
    #[inline]
    pub fn endepout6(&self) -> ENDEPOUT6R {
        ENDEPOUT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for ENDEPOUT[7] event"]
    #[inline]
    pub fn endepout7(&self) -> ENDEPOUT7R {
        ENDEPOUT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for ENDISOOUT event"]
    #[inline]
    pub fn endisoout(&self) -> ENDISOOUTR {
        ENDISOOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for SOF event"]
    #[inline]
    pub fn sof(&self) -> SOFR {
        SOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for USBEVENT event"]
    #[inline]
    pub fn usbevent(&self) -> USBEVENTR {
        USBEVENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for EP0SETUP event"]
    #[inline]
    pub fn ep0setup(&self) -> EP0SETUPR {
        EP0SETUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Write '1' to disable interrupt for EPDATA event"]
    #[inline]
    pub fn epdata(&self) -> EPDATAR {
        EPDATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Write '1' to disable interrupt for USBRESET event"]
    #[inline]
    pub fn usbreset(&mut self) -> _USBRESETW {
        _USBRESETW { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for STARTED event"]
    #[inline]
    pub fn started(&mut self) -> _STARTEDW {
        _STARTEDW { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for ENDEPIN[0] event"]
    #[inline]
    pub fn endepin0(&mut self) -> _ENDEPIN0W {
        _ENDEPIN0W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for ENDEPIN[1] event"]
    #[inline]
    pub fn endepin1(&mut self) -> _ENDEPIN1W {
        _ENDEPIN1W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for ENDEPIN[2] event"]
    #[inline]
    pub fn endepin2(&mut self) -> _ENDEPIN2W {
        _ENDEPIN2W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for ENDEPIN[3] event"]
    #[inline]
    pub fn endepin3(&mut self) -> _ENDEPIN3W {
        _ENDEPIN3W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for ENDEPIN[4] event"]
    #[inline]
    pub fn endepin4(&mut self) -> _ENDEPIN4W {
        _ENDEPIN4W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for ENDEPIN[5] event"]
    #[inline]
    pub fn endepin5(&mut self) -> _ENDEPIN5W {
        _ENDEPIN5W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for ENDEPIN[6] event"]
    #[inline]
    pub fn endepin6(&mut self) -> _ENDEPIN6W {
        _ENDEPIN6W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for ENDEPIN[7] event"]
    #[inline]
    pub fn endepin7(&mut self) -> _ENDEPIN7W {
        _ENDEPIN7W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for EP0DATADONE event"]
    #[inline]
    pub fn ep0datadone(&mut self) -> _EP0DATADONEW {
        _EP0DATADONEW { w: self }
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for ENDISOIN event"]
    #[inline]
    pub fn endisoin(&mut self) -> _ENDISOINW {
        _ENDISOINW { w: self }
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for ENDEPOUT[0] event"]
    #[inline]
    pub fn endepout0(&mut self) -> _ENDEPOUT0W {
        _ENDEPOUT0W { w: self }
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for ENDEPOUT[1] event"]
    #[inline]
    pub fn endepout1(&mut self) -> _ENDEPOUT1W {
        _ENDEPOUT1W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for ENDEPOUT[2] event"]
    #[inline]
    pub fn endepout2(&mut self) -> _ENDEPOUT2W {
        _ENDEPOUT2W { w: self }
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for ENDEPOUT[3] event"]
    #[inline]
    pub fn endepout3(&mut self) -> _ENDEPOUT3W {
        _ENDEPOUT3W { w: self }
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for ENDEPOUT[4] event"]
    #[inline]
    pub fn endepout4(&mut self) -> _ENDEPOUT4W {
        _ENDEPOUT4W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for ENDEPOUT[5] event"]
    #[inline]
    pub fn endepout5(&mut self) -> _ENDEPOUT5W {
        _ENDEPOUT5W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for ENDEPOUT[6] event"]
    #[inline]
    pub fn endepout6(&mut self) -> _ENDEPOUT6W {
        _ENDEPOUT6W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for ENDEPOUT[7] event"]
    #[inline]
    pub fn endepout7(&mut self) -> _ENDEPOUT7W {
        _ENDEPOUT7W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for ENDISOOUT event"]
    #[inline]
    pub fn endisoout(&mut self) -> _ENDISOOUTW {
        _ENDISOOUTW { w: self }
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for SOF event"]
    #[inline]
    pub fn sof(&mut self) -> _SOFW {
        _SOFW { w: self }
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for USBEVENT event"]
    #[inline]
    pub fn usbevent(&mut self) -> _USBEVENTW {
        _USBEVENTW { w: self }
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for EP0SETUP event"]
    #[inline]
    pub fn ep0setup(&mut self) -> _EP0SETUPW {
        _EP0SETUPW { w: self }
    }
    #[doc = "Bit 24 - Write '1' to disable interrupt for EPDATA event"]
    #[inline]
    pub fn epdata(&mut self) -> _EPDATAW {
        _EPDATAW { w: self }
    }
}
