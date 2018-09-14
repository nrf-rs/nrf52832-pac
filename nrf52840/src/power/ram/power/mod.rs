#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POWER {
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
#[doc = "Possible values of the field `S0POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S0POWERR {
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
            S0POWERR::OFF => false,
            S0POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0POWERR {
        match value {
            false => S0POWERR::OFF,
            true => S0POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S0POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S0POWERR::ON
    }
}
#[doc = "Possible values of the field `S1POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S1POWERR {
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
            S1POWERR::OFF => false,
            S1POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1POWERR {
        match value {
            false => S1POWERR::OFF,
            true => S1POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S1POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S1POWERR::ON
    }
}
#[doc = "Possible values of the field `S2POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S2POWERR {
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
            S2POWERR::OFF => false,
            S2POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2POWERR {
        match value {
            false => S2POWERR::OFF,
            true => S2POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S2POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S2POWERR::ON
    }
}
#[doc = "Possible values of the field `S3POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S3POWERR {
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
            S3POWERR::OFF => false,
            S3POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3POWERR {
        match value {
            false => S3POWERR::OFF,
            true => S3POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S3POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S3POWERR::ON
    }
}
#[doc = "Possible values of the field `S4POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S4POWERR {
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
            S4POWERR::OFF => false,
            S4POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S4POWERR {
        match value {
            false => S4POWERR::OFF,
            true => S4POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S4POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S4POWERR::ON
    }
}
#[doc = "Possible values of the field `S5POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S5POWERR {
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
            S5POWERR::OFF => false,
            S5POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S5POWERR {
        match value {
            false => S5POWERR::OFF,
            true => S5POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S5POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S5POWERR::ON
    }
}
#[doc = "Possible values of the field `S6POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S6POWERR {
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
            S6POWERR::OFF => false,
            S6POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S6POWERR {
        match value {
            false => S6POWERR::OFF,
            true => S6POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S6POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S6POWERR::ON
    }
}
#[doc = "Possible values of the field `S7POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S7POWERR {
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
            S7POWERR::OFF => false,
            S7POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S7POWERR {
        match value {
            false => S7POWERR::OFF,
            true => S7POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S7POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S7POWERR::ON
    }
}
#[doc = "Possible values of the field `S8POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S8POWERR {
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
            S8POWERR::OFF => false,
            S8POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S8POWERR {
        match value {
            false => S8POWERR::OFF,
            true => S8POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S8POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S8POWERR::ON
    }
}
#[doc = "Possible values of the field `S9POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S9POWERR {
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
            S9POWERR::OFF => false,
            S9POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S9POWERR {
        match value {
            false => S9POWERR::OFF,
            true => S9POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S9POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S9POWERR::ON
    }
}
#[doc = "Possible values of the field `S10POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S10POWERR {
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
            S10POWERR::OFF => false,
            S10POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S10POWERR {
        match value {
            false => S10POWERR::OFF,
            true => S10POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S10POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S10POWERR::ON
    }
}
#[doc = "Possible values of the field `S11POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S11POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S11POWERR {
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
            S11POWERR::OFF => false,
            S11POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S11POWERR {
        match value {
            false => S11POWERR::OFF,
            true => S11POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S11POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S11POWERR::ON
    }
}
#[doc = "Possible values of the field `S12POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S12POWERR {
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
            S12POWERR::OFF => false,
            S12POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S12POWERR {
        match value {
            false => S12POWERR::OFF,
            true => S12POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S12POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S12POWERR::ON
    }
}
#[doc = "Possible values of the field `S13POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S13POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S13POWERR {
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
            S13POWERR::OFF => false,
            S13POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S13POWERR {
        match value {
            false => S13POWERR::OFF,
            true => S13POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S13POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S13POWERR::ON
    }
}
#[doc = "Possible values of the field `S14POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S14POWERR {
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
            S14POWERR::OFF => false,
            S14POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S14POWERR {
        match value {
            false => S14POWERR::OFF,
            true => S14POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S14POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S14POWERR::ON
    }
}
#[doc = "Possible values of the field `S15POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S15POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S15POWERR {
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
            S15POWERR::OFF => false,
            S15POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S15POWERR {
        match value {
            false => S15POWERR::OFF,
            true => S15POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S15POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S15POWERR::ON
    }
}
#[doc = "Possible values of the field `S0RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S0RETENTIONR {
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
            S0RETENTIONR::OFF => false,
            S0RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0RETENTIONR {
        match value {
            false => S0RETENTIONR::OFF,
            true => S0RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S0RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S0RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S1RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S1RETENTIONR {
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
            S1RETENTIONR::OFF => false,
            S1RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1RETENTIONR {
        match value {
            false => S1RETENTIONR::OFF,
            true => S1RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S1RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S1RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S2RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S2RETENTIONR {
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
            S2RETENTIONR::OFF => false,
            S2RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2RETENTIONR {
        match value {
            false => S2RETENTIONR::OFF,
            true => S2RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S2RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S2RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S3RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S3RETENTIONR {
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
            S3RETENTIONR::OFF => false,
            S3RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3RETENTIONR {
        match value {
            false => S3RETENTIONR::OFF,
            true => S3RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S3RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S3RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S4RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S4RETENTIONR {
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
            S4RETENTIONR::OFF => false,
            S4RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S4RETENTIONR {
        match value {
            false => S4RETENTIONR::OFF,
            true => S4RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S4RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S4RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S5RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S5RETENTIONR {
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
            S5RETENTIONR::OFF => false,
            S5RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S5RETENTIONR {
        match value {
            false => S5RETENTIONR::OFF,
            true => S5RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S5RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S5RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S6RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S6RETENTIONR {
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
            S6RETENTIONR::OFF => false,
            S6RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S6RETENTIONR {
        match value {
            false => S6RETENTIONR::OFF,
            true => S6RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S6RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S6RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S7RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S7RETENTIONR {
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
            S7RETENTIONR::OFF => false,
            S7RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S7RETENTIONR {
        match value {
            false => S7RETENTIONR::OFF,
            true => S7RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S7RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S7RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S8RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S8RETENTIONR {
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
            S8RETENTIONR::OFF => false,
            S8RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S8RETENTIONR {
        match value {
            false => S8RETENTIONR::OFF,
            true => S8RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S8RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S8RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S9RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S9RETENTIONR {
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
            S9RETENTIONR::OFF => false,
            S9RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S9RETENTIONR {
        match value {
            false => S9RETENTIONR::OFF,
            true => S9RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S9RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S9RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S10RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S10RETENTIONR {
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
            S10RETENTIONR::OFF => false,
            S10RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S10RETENTIONR {
        match value {
            false => S10RETENTIONR::OFF,
            true => S10RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S10RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S10RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S11RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S11RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S11RETENTIONR {
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
            S11RETENTIONR::OFF => false,
            S11RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S11RETENTIONR {
        match value {
            false => S11RETENTIONR::OFF,
            true => S11RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S11RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S11RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S12RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S12RETENTIONR {
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
            S12RETENTIONR::OFF => false,
            S12RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S12RETENTIONR {
        match value {
            false => S12RETENTIONR::OFF,
            true => S12RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S12RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S12RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S13RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S13RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S13RETENTIONR {
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
            S13RETENTIONR::OFF => false,
            S13RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S13RETENTIONR {
        match value {
            false => S13RETENTIONR::OFF,
            true => S13RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S13RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S13RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S14RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S14RETENTIONR {
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
            S14RETENTIONR::OFF => false,
            S14RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S14RETENTIONR {
        match value {
            false => S14RETENTIONR::OFF,
            true => S14RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S14RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S14RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S15RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S15RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S15RETENTIONR {
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
            S15RETENTIONR::OFF => false,
            S15RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S15RETENTIONR {
        match value {
            false => S15RETENTIONR::OFF,
            true => S15RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S15RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S15RETENTIONR::ON
    }
}
#[doc = "Values that can be written to the field `S0POWER`"]
pub enum S0POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S0POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S0POWERW::OFF => false,
            S0POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S0POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S0POWERW::ON)
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
#[doc = "Values that can be written to the field `S1POWER`"]
pub enum S1POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S1POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1POWERW::OFF => false,
            S1POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S1POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S1POWERW::ON)
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
#[doc = "Values that can be written to the field `S2POWER`"]
pub enum S2POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S2POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S2POWERW::OFF => false,
            S2POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S2POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S2POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S2POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S2POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S2POWERW::ON)
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
#[doc = "Values that can be written to the field `S3POWER`"]
pub enum S3POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S3POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S3POWERW::OFF => false,
            S3POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S3POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S3POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S3POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S3POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S3POWERW::ON)
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
#[doc = "Values that can be written to the field `S4POWER`"]
pub enum S4POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S4POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S4POWERW::OFF => false,
            S4POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S4POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S4POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S4POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S4POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S4POWERW::ON)
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
#[doc = "Values that can be written to the field `S5POWER`"]
pub enum S5POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S5POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S5POWERW::OFF => false,
            S5POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S5POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S5POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S5POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S5POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S5POWERW::ON)
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
#[doc = "Values that can be written to the field `S6POWER`"]
pub enum S6POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S6POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S6POWERW::OFF => false,
            S6POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S6POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S6POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S6POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S6POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S6POWERW::ON)
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
#[doc = "Values that can be written to the field `S7POWER`"]
pub enum S7POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S7POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S7POWERW::OFF => false,
            S7POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S7POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S7POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S7POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S7POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S7POWERW::ON)
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
#[doc = "Values that can be written to the field `S8POWER`"]
pub enum S8POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S8POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S8POWERW::OFF => false,
            S8POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S8POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S8POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S8POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S8POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S8POWERW::ON)
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
#[doc = "Values that can be written to the field `S9POWER`"]
pub enum S9POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S9POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S9POWERW::OFF => false,
            S9POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S9POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S9POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S9POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S9POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S9POWERW::ON)
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
#[doc = "Values that can be written to the field `S10POWER`"]
pub enum S10POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S10POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S10POWERW::OFF => false,
            S10POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S10POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S10POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S10POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S10POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S10POWERW::ON)
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
#[doc = "Values that can be written to the field `S11POWER`"]
pub enum S11POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S11POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S11POWERW::OFF => false,
            S11POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S11POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S11POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S11POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S11POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S11POWERW::ON)
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
#[doc = "Values that can be written to the field `S12POWER`"]
pub enum S12POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S12POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S12POWERW::OFF => false,
            S12POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S12POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S12POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S12POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S12POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S12POWERW::ON)
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
#[doc = "Values that can be written to the field `S13POWER`"]
pub enum S13POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S13POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S13POWERW::OFF => false,
            S13POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S13POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S13POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S13POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S13POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S13POWERW::ON)
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
#[doc = "Values that can be written to the field `S14POWER`"]
pub enum S14POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S14POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S14POWERW::OFF => false,
            S14POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S14POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S14POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S14POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S14POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S14POWERW::ON)
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
#[doc = "Values that can be written to the field `S15POWER`"]
pub enum S15POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S15POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S15POWERW::OFF => false,
            S15POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S15POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S15POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S15POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S15POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S15POWERW::ON)
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
#[doc = "Values that can be written to the field `S0RETENTION`"]
pub enum S0RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S0RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S0RETENTIONW::OFF => false,
            S0RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S0RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S0RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S1RETENTION`"]
pub enum S1RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S1RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1RETENTIONW::OFF => false,
            S1RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S1RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S1RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S2RETENTION`"]
pub enum S2RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S2RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S2RETENTIONW::OFF => false,
            S2RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S2RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S2RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S2RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S2RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S2RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S3RETENTION`"]
pub enum S3RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S3RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S3RETENTIONW::OFF => false,
            S3RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S3RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S3RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S3RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S3RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S3RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S4RETENTION`"]
pub enum S4RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S4RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S4RETENTIONW::OFF => false,
            S4RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S4RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S4RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S4RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S4RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S4RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S5RETENTION`"]
pub enum S5RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S5RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S5RETENTIONW::OFF => false,
            S5RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S5RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S5RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S5RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S5RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S5RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S6RETENTION`"]
pub enum S6RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S6RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S6RETENTIONW::OFF => false,
            S6RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S6RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S6RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S6RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S6RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S6RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S7RETENTION`"]
pub enum S7RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S7RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S7RETENTIONW::OFF => false,
            S7RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S7RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S7RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S7RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S7RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S7RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S8RETENTION`"]
pub enum S8RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S8RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S8RETENTIONW::OFF => false,
            S8RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S8RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S8RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S8RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S8RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S8RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S9RETENTION`"]
pub enum S9RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S9RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S9RETENTIONW::OFF => false,
            S9RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S9RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S9RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S9RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S9RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S9RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S10RETENTION`"]
pub enum S10RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S10RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S10RETENTIONW::OFF => false,
            S10RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S10RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S10RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S10RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S10RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S10RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S11RETENTION`"]
pub enum S11RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S11RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S11RETENTIONW::OFF => false,
            S11RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S11RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S11RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S11RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S11RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S11RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S12RETENTION`"]
pub enum S12RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S12RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S12RETENTIONW::OFF => false,
            S12RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S12RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S12RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S12RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S12RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S12RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S13RETENTION`"]
pub enum S13RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S13RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S13RETENTIONW::OFF => false,
            S13RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S13RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S13RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S13RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S13RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S13RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S14RETENTION`"]
pub enum S14RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S14RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S14RETENTIONW::OFF => false,
            S14RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S14RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S14RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S14RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S14RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S14RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S15RETENTION`"]
pub enum S15RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S15RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S15RETENTIONW::OFF => false,
            S15RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S15RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S15RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S15RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S15RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S15RETENTIONW::ON)
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
    #[doc = "Bit 0 - Keep RAM section S0 on or off in System ON mode."]
    #[inline]
    pub fn s0power(&self) -> S0POWERR {
        S0POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Keep RAM section S1 on or off in System ON mode."]
    #[inline]
    pub fn s1power(&self) -> S1POWERR {
        S1POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Keep RAM section S2 on or off in System ON mode."]
    #[inline]
    pub fn s2power(&self) -> S2POWERR {
        S2POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Keep RAM section S3 on or off in System ON mode."]
    #[inline]
    pub fn s3power(&self) -> S3POWERR {
        S3POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Keep RAM section S4 on or off in System ON mode."]
    #[inline]
    pub fn s4power(&self) -> S4POWERR {
        S4POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Keep RAM section S5 on or off in System ON mode."]
    #[inline]
    pub fn s5power(&self) -> S5POWERR {
        S5POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Keep RAM section S6 on or off in System ON mode."]
    #[inline]
    pub fn s6power(&self) -> S6POWERR {
        S6POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Keep RAM section S7 on or off in System ON mode."]
    #[inline]
    pub fn s7power(&self) -> S7POWERR {
        S7POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Keep RAM section S8 on or off in System ON mode."]
    #[inline]
    pub fn s8power(&self) -> S8POWERR {
        S8POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Keep RAM section S9 on or off in System ON mode."]
    #[inline]
    pub fn s9power(&self) -> S9POWERR {
        S9POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Keep RAM section S10 on or off in System ON mode."]
    #[inline]
    pub fn s10power(&self) -> S10POWERR {
        S10POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Keep RAM section S11 on or off in System ON mode."]
    #[inline]
    pub fn s11power(&self) -> S11POWERR {
        S11POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Keep RAM section S12 on or off in System ON mode."]
    #[inline]
    pub fn s12power(&self) -> S12POWERR {
        S12POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Keep RAM section S13 on or off in System ON mode."]
    #[inline]
    pub fn s13power(&self) -> S13POWERR {
        S13POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Keep RAM section S14 on or off in System ON mode."]
    #[inline]
    pub fn s14power(&self) -> S14POWERR {
        S14POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Keep RAM section S15 on or off in System ON mode."]
    #[inline]
    pub fn s15power(&self) -> S15POWERR {
        S15POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is off"]
    #[inline]
    pub fn s0retention(&self) -> S0RETENTIONR {
        S0RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is off"]
    #[inline]
    pub fn s1retention(&self) -> S1RETENTIONR {
        S1RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is off"]
    #[inline]
    pub fn s2retention(&self) -> S2RETENTIONR {
        S2RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is off"]
    #[inline]
    pub fn s3retention(&self) -> S3RETENTIONR {
        S3RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is off"]
    #[inline]
    pub fn s4retention(&self) -> S4RETENTIONR {
        S4RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is off"]
    #[inline]
    pub fn s5retention(&self) -> S5RETENTIONR {
        S5RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is off"]
    #[inline]
    pub fn s6retention(&self) -> S6RETENTIONR {
        S6RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is off"]
    #[inline]
    pub fn s7retention(&self) -> S7RETENTIONR {
        S7RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is off"]
    #[inline]
    pub fn s8retention(&self) -> S8RETENTIONR {
        S8RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is off"]
    #[inline]
    pub fn s9retention(&self) -> S9RETENTIONR {
        S9RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is off"]
    #[inline]
    pub fn s10retention(&self) -> S10RETENTIONR {
        S10RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is off"]
    #[inline]
    pub fn s11retention(&self) -> S11RETENTIONR {
        S11RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is off"]
    #[inline]
    pub fn s12retention(&self) -> S12RETENTIONR {
        S12RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is off"]
    #[inline]
    pub fn s13retention(&self) -> S13RETENTIONR {
        S13RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is off"]
    #[inline]
    pub fn s14retention(&self) -> S14RETENTIONR {
        S14RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is off"]
    #[inline]
    pub fn s15retention(&self) -> S15RETENTIONR {
        S15RETENTIONR::_from({
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
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Keep RAM section S0 on or off in System ON mode."]
    #[inline]
    pub fn s0power(&mut self) -> _S0POWERW {
        _S0POWERW { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 on or off in System ON mode."]
    #[inline]
    pub fn s1power(&mut self) -> _S1POWERW {
        _S1POWERW { w: self }
    }
    #[doc = "Bit 2 - Keep RAM section S2 on or off in System ON mode."]
    #[inline]
    pub fn s2power(&mut self) -> _S2POWERW {
        _S2POWERW { w: self }
    }
    #[doc = "Bit 3 - Keep RAM section S3 on or off in System ON mode."]
    #[inline]
    pub fn s3power(&mut self) -> _S3POWERW {
        _S3POWERW { w: self }
    }
    #[doc = "Bit 4 - Keep RAM section S4 on or off in System ON mode."]
    #[inline]
    pub fn s4power(&mut self) -> _S4POWERW {
        _S4POWERW { w: self }
    }
    #[doc = "Bit 5 - Keep RAM section S5 on or off in System ON mode."]
    #[inline]
    pub fn s5power(&mut self) -> _S5POWERW {
        _S5POWERW { w: self }
    }
    #[doc = "Bit 6 - Keep RAM section S6 on or off in System ON mode."]
    #[inline]
    pub fn s6power(&mut self) -> _S6POWERW {
        _S6POWERW { w: self }
    }
    #[doc = "Bit 7 - Keep RAM section S7 on or off in System ON mode."]
    #[inline]
    pub fn s7power(&mut self) -> _S7POWERW {
        _S7POWERW { w: self }
    }
    #[doc = "Bit 8 - Keep RAM section S8 on or off in System ON mode."]
    #[inline]
    pub fn s8power(&mut self) -> _S8POWERW {
        _S8POWERW { w: self }
    }
    #[doc = "Bit 9 - Keep RAM section S9 on or off in System ON mode."]
    #[inline]
    pub fn s9power(&mut self) -> _S9POWERW {
        _S9POWERW { w: self }
    }
    #[doc = "Bit 10 - Keep RAM section S10 on or off in System ON mode."]
    #[inline]
    pub fn s10power(&mut self) -> _S10POWERW {
        _S10POWERW { w: self }
    }
    #[doc = "Bit 11 - Keep RAM section S11 on or off in System ON mode."]
    #[inline]
    pub fn s11power(&mut self) -> _S11POWERW {
        _S11POWERW { w: self }
    }
    #[doc = "Bit 12 - Keep RAM section S12 on or off in System ON mode."]
    #[inline]
    pub fn s12power(&mut self) -> _S12POWERW {
        _S12POWERW { w: self }
    }
    #[doc = "Bit 13 - Keep RAM section S13 on or off in System ON mode."]
    #[inline]
    pub fn s13power(&mut self) -> _S13POWERW {
        _S13POWERW { w: self }
    }
    #[doc = "Bit 14 - Keep RAM section S14 on or off in System ON mode."]
    #[inline]
    pub fn s14power(&mut self) -> _S14POWERW {
        _S14POWERW { w: self }
    }
    #[doc = "Bit 15 - Keep RAM section S15 on or off in System ON mode."]
    #[inline]
    pub fn s15power(&mut self) -> _S15POWERW {
        _S15POWERW { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is off"]
    #[inline]
    pub fn s0retention(&mut self) -> _S0RETENTIONW {
        _S0RETENTIONW { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is off"]
    #[inline]
    pub fn s1retention(&mut self) -> _S1RETENTIONW {
        _S1RETENTIONW { w: self }
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is off"]
    #[inline]
    pub fn s2retention(&mut self) -> _S2RETENTIONW {
        _S2RETENTIONW { w: self }
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is off"]
    #[inline]
    pub fn s3retention(&mut self) -> _S3RETENTIONW {
        _S3RETENTIONW { w: self }
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is off"]
    #[inline]
    pub fn s4retention(&mut self) -> _S4RETENTIONW {
        _S4RETENTIONW { w: self }
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is off"]
    #[inline]
    pub fn s5retention(&mut self) -> _S5RETENTIONW {
        _S5RETENTIONW { w: self }
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is off"]
    #[inline]
    pub fn s6retention(&mut self) -> _S6RETENTIONW {
        _S6RETENTIONW { w: self }
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is off"]
    #[inline]
    pub fn s7retention(&mut self) -> _S7RETENTIONW {
        _S7RETENTIONW { w: self }
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is off"]
    #[inline]
    pub fn s8retention(&mut self) -> _S8RETENTIONW {
        _S8RETENTIONW { w: self }
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is off"]
    #[inline]
    pub fn s9retention(&mut self) -> _S9RETENTIONW {
        _S9RETENTIONW { w: self }
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is off"]
    #[inline]
    pub fn s10retention(&mut self) -> _S10RETENTIONW {
        _S10RETENTIONW { w: self }
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is off"]
    #[inline]
    pub fn s11retention(&mut self) -> _S11RETENTIONW {
        _S11RETENTIONW { w: self }
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is off"]
    #[inline]
    pub fn s12retention(&mut self) -> _S12RETENTIONW {
        _S12RETENTIONW { w: self }
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is off"]
    #[inline]
    pub fn s13retention(&mut self) -> _S13RETENTIONW {
        _S13RETENTIONW { w: self }
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is off"]
    #[inline]
    pub fn s14retention(&mut self) -> _S14RETENTIONW {
        _S14RETENTIONW { w: self }
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is off"]
    #[inline]
    pub fn s15retention(&mut self) -> _S15RETENTIONW {
        _S15RETENTIONW { w: self }
    }
}
