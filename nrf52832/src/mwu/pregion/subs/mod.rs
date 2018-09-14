#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUBS {
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
#[doc = "Possible values of the field `SR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR0R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR0R {
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
            SR0R::EXCLUDE => false,
            SR0R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR0R {
        match value {
            false => SR0R::EXCLUDE,
            true => SR0R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR0R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR0R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR1R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR1R {
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
            SR1R::EXCLUDE => false,
            SR1R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR1R {
        match value {
            false => SR1R::EXCLUDE,
            true => SR1R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR1R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR1R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR2R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR2R {
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
            SR2R::EXCLUDE => false,
            SR2R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR2R {
        match value {
            false => SR2R::EXCLUDE,
            true => SR2R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR2R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR2R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR3R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR3R {
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
            SR3R::EXCLUDE => false,
            SR3R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR3R {
        match value {
            false => SR3R::EXCLUDE,
            true => SR3R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR3R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR3R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR4R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR4R {
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
            SR4R::EXCLUDE => false,
            SR4R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR4R {
        match value {
            false => SR4R::EXCLUDE,
            true => SR4R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR4R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR4R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR5R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR5R {
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
            SR5R::EXCLUDE => false,
            SR5R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR5R {
        match value {
            false => SR5R::EXCLUDE,
            true => SR5R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR5R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR5R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR6R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR6R {
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
            SR6R::EXCLUDE => false,
            SR6R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR6R {
        match value {
            false => SR6R::EXCLUDE,
            true => SR6R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR6R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR6R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR7R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR7R {
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
            SR7R::EXCLUDE => false,
            SR7R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR7R {
        match value {
            false => SR7R::EXCLUDE,
            true => SR7R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR7R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR7R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR8R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR8R {
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
            SR8R::EXCLUDE => false,
            SR8R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR8R {
        match value {
            false => SR8R::EXCLUDE,
            true => SR8R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR8R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR8R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR9R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR9R {
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
            SR9R::EXCLUDE => false,
            SR9R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR9R {
        match value {
            false => SR9R::EXCLUDE,
            true => SR9R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR9R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR9R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR10R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR10R {
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
            SR10R::EXCLUDE => false,
            SR10R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR10R {
        match value {
            false => SR10R::EXCLUDE,
            true => SR10R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR10R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR10R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR11R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR11R {
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
            SR11R::EXCLUDE => false,
            SR11R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR11R {
        match value {
            false => SR11R::EXCLUDE,
            true => SR11R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR11R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR11R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR12R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR12R {
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
            SR12R::EXCLUDE => false,
            SR12R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR12R {
        match value {
            false => SR12R::EXCLUDE,
            true => SR12R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR12R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR12R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR13R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR13R {
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
            SR13R::EXCLUDE => false,
            SR13R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR13R {
        match value {
            false => SR13R::EXCLUDE,
            true => SR13R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR13R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR13R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR14R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR14R {
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
            SR14R::EXCLUDE => false,
            SR14R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR14R {
        match value {
            false => SR14R::EXCLUDE,
            true => SR14R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR14R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR14R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR15R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR15R {
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
            SR15R::EXCLUDE => false,
            SR15R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR15R {
        match value {
            false => SR15R::EXCLUDE,
            true => SR15R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR15R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR15R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR16R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR16R {
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
            SR16R::EXCLUDE => false,
            SR16R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR16R {
        match value {
            false => SR16R::EXCLUDE,
            true => SR16R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR16R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR16R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR17R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR17R {
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
            SR17R::EXCLUDE => false,
            SR17R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR17R {
        match value {
            false => SR17R::EXCLUDE,
            true => SR17R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR17R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR17R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR18R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR18R {
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
            SR18R::EXCLUDE => false,
            SR18R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR18R {
        match value {
            false => SR18R::EXCLUDE,
            true => SR18R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR18R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR18R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR19R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR19R {
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
            SR19R::EXCLUDE => false,
            SR19R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR19R {
        match value {
            false => SR19R::EXCLUDE,
            true => SR19R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR19R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR19R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR20R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR20R {
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
            SR20R::EXCLUDE => false,
            SR20R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR20R {
        match value {
            false => SR20R::EXCLUDE,
            true => SR20R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR20R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR20R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR21R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR21R {
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
            SR21R::EXCLUDE => false,
            SR21R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR21R {
        match value {
            false => SR21R::EXCLUDE,
            true => SR21R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR21R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR21R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR22R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR22R {
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
            SR22R::EXCLUDE => false,
            SR22R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR22R {
        match value {
            false => SR22R::EXCLUDE,
            true => SR22R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR22R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR22R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR23R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR23R {
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
            SR23R::EXCLUDE => false,
            SR23R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR23R {
        match value {
            false => SR23R::EXCLUDE,
            true => SR23R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR23R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR23R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR24R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR24R {
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
            SR24R::EXCLUDE => false,
            SR24R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR24R {
        match value {
            false => SR24R::EXCLUDE,
            true => SR24R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR24R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR24R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR25R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR25R {
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
            SR25R::EXCLUDE => false,
            SR25R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR25R {
        match value {
            false => SR25R::EXCLUDE,
            true => SR25R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR25R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR25R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR26R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR26R {
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
            SR26R::EXCLUDE => false,
            SR26R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR26R {
        match value {
            false => SR26R::EXCLUDE,
            true => SR26R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR26R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR26R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR27R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR27R {
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
            SR27R::EXCLUDE => false,
            SR27R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR27R {
        match value {
            false => SR27R::EXCLUDE,
            true => SR27R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR27R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR27R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR28R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR28R {
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
            SR28R::EXCLUDE => false,
            SR28R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR28R {
        match value {
            false => SR28R::EXCLUDE,
            true => SR28R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR28R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR28R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR29R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR29R {
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
            SR29R::EXCLUDE => false,
            SR29R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR29R {
        match value {
            false => SR29R::EXCLUDE,
            true => SR29R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR29R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR29R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR30R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR30R {
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
            SR30R::EXCLUDE => false,
            SR30R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR30R {
        match value {
            false => SR30R::EXCLUDE,
            true => SR30R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR30R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR30R::INCLUDE
    }
}
#[doc = "Possible values of the field `SR31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR31R {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR31R {
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
            SR31R::EXCLUDE => false,
            SR31R::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR31R {
        match value {
            false => SR31R::EXCLUDE,
            true => SR31R::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == SR31R::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SR31R::INCLUDE
    }
}
#[doc = "Values that can be written to the field `SR0`"]
pub enum SR0W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR0W::EXCLUDE => false,
            SR0W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR0W<'a> {
    w: &'a mut W,
}
impl<'a> _SR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR0W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR0W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR1`"]
pub enum SR1W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR1W::EXCLUDE => false,
            SR1W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR1W<'a> {
    w: &'a mut W,
}
impl<'a> _SR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR1W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR1W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR2`"]
pub enum SR2W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR2W::EXCLUDE => false,
            SR2W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR2W<'a> {
    w: &'a mut W,
}
impl<'a> _SR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR2W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR2W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR3`"]
pub enum SR3W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR3W::EXCLUDE => false,
            SR3W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR3W<'a> {
    w: &'a mut W,
}
impl<'a> _SR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR3W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR3W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR4`"]
pub enum SR4W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR4W::EXCLUDE => false,
            SR4W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR4W<'a> {
    w: &'a mut W,
}
impl<'a> _SR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR4W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR4W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR5`"]
pub enum SR5W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR5W::EXCLUDE => false,
            SR5W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR5W<'a> {
    w: &'a mut W,
}
impl<'a> _SR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR5W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR5W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR6`"]
pub enum SR6W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR6W::EXCLUDE => false,
            SR6W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR6W<'a> {
    w: &'a mut W,
}
impl<'a> _SR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR6W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR6W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR7`"]
pub enum SR7W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR7W::EXCLUDE => false,
            SR7W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR7W<'a> {
    w: &'a mut W,
}
impl<'a> _SR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR7W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR7W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR8`"]
pub enum SR8W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR8W::EXCLUDE => false,
            SR8W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR8W<'a> {
    w: &'a mut W,
}
impl<'a> _SR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR8W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR8W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR9`"]
pub enum SR9W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR9W::EXCLUDE => false,
            SR9W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR9W<'a> {
    w: &'a mut W,
}
impl<'a> _SR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR9W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR9W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR10`"]
pub enum SR10W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR10W::EXCLUDE => false,
            SR10W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR10W<'a> {
    w: &'a mut W,
}
impl<'a> _SR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR10W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR10W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR11`"]
pub enum SR11W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR11W::EXCLUDE => false,
            SR11W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR11W<'a> {
    w: &'a mut W,
}
impl<'a> _SR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR11W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR11W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR12`"]
pub enum SR12W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR12W::EXCLUDE => false,
            SR12W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR12W<'a> {
    w: &'a mut W,
}
impl<'a> _SR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR12W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR12W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR13`"]
pub enum SR13W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR13W::EXCLUDE => false,
            SR13W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR13W<'a> {
    w: &'a mut W,
}
impl<'a> _SR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR13W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR13W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR14`"]
pub enum SR14W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR14W::EXCLUDE => false,
            SR14W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR14W<'a> {
    w: &'a mut W,
}
impl<'a> _SR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR14W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR14W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR15`"]
pub enum SR15W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR15W::EXCLUDE => false,
            SR15W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR15W<'a> {
    w: &'a mut W,
}
impl<'a> _SR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR15W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR15W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR16`"]
pub enum SR16W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR16W::EXCLUDE => false,
            SR16W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR16W<'a> {
    w: &'a mut W,
}
impl<'a> _SR16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR16W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR16W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR17`"]
pub enum SR17W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR17W::EXCLUDE => false,
            SR17W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR17W<'a> {
    w: &'a mut W,
}
impl<'a> _SR17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR17W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR17W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR18`"]
pub enum SR18W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR18W::EXCLUDE => false,
            SR18W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR18W<'a> {
    w: &'a mut W,
}
impl<'a> _SR18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR18W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR18W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR19`"]
pub enum SR19W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR19W::EXCLUDE => false,
            SR19W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR19W<'a> {
    w: &'a mut W,
}
impl<'a> _SR19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR19W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR19W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR20`"]
pub enum SR20W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR20W::EXCLUDE => false,
            SR20W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR20W<'a> {
    w: &'a mut W,
}
impl<'a> _SR20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR20W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR20W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR21`"]
pub enum SR21W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR21W::EXCLUDE => false,
            SR21W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR21W<'a> {
    w: &'a mut W,
}
impl<'a> _SR21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR21W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR21W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR22`"]
pub enum SR22W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR22W::EXCLUDE => false,
            SR22W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR22W<'a> {
    w: &'a mut W,
}
impl<'a> _SR22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR22W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR22W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR23`"]
pub enum SR23W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR23W::EXCLUDE => false,
            SR23W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR23W<'a> {
    w: &'a mut W,
}
impl<'a> _SR23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR23W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR23W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR24`"]
pub enum SR24W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR24W::EXCLUDE => false,
            SR24W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR24W<'a> {
    w: &'a mut W,
}
impl<'a> _SR24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR24W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR24W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR25`"]
pub enum SR25W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR25W::EXCLUDE => false,
            SR25W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR25W<'a> {
    w: &'a mut W,
}
impl<'a> _SR25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR25W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR25W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR26`"]
pub enum SR26W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR26W::EXCLUDE => false,
            SR26W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR26W<'a> {
    w: &'a mut W,
}
impl<'a> _SR26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR26W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR26W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR27`"]
pub enum SR27W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR27W::EXCLUDE => false,
            SR27W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR27W<'a> {
    w: &'a mut W,
}
impl<'a> _SR27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR27W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR27W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR28`"]
pub enum SR28W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR28W::EXCLUDE => false,
            SR28W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR28W<'a> {
    w: &'a mut W,
}
impl<'a> _SR28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR28W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR28W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR29`"]
pub enum SR29W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR29W::EXCLUDE => false,
            SR29W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR29W<'a> {
    w: &'a mut W,
}
impl<'a> _SR29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR29W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR29W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR30`"]
pub enum SR30W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR30W::EXCLUDE => false,
            SR30W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR30W<'a> {
    w: &'a mut W,
}
impl<'a> _SR30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR30W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR30W::INCLUDE)
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
#[doc = "Values that can be written to the field `SR31`"]
pub enum SR31W {
    #[doc = "Exclude"]
    EXCLUDE,
    #[doc = "Include"]
    INCLUDE,
}
impl SR31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR31W::EXCLUDE => false,
            SR31W::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SR31W<'a> {
    w: &'a mut W,
}
impl<'a> _SR31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SR31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(SR31W::EXCLUDE)
    }
    #[doc = "Include"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SR31W::INCLUDE)
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
    #[doc = "Bit 0 - Include or exclude subregion 0 in region"]
    #[inline]
    pub fn sr0(&self) -> SR0R {
        SR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Include or exclude subregion 1 in region"]
    #[inline]
    pub fn sr1(&self) -> SR1R {
        SR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Include or exclude subregion 2 in region"]
    #[inline]
    pub fn sr2(&self) -> SR2R {
        SR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Include or exclude subregion 3 in region"]
    #[inline]
    pub fn sr3(&self) -> SR3R {
        SR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Include or exclude subregion 4 in region"]
    #[inline]
    pub fn sr4(&self) -> SR4R {
        SR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Include or exclude subregion 5 in region"]
    #[inline]
    pub fn sr5(&self) -> SR5R {
        SR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Include or exclude subregion 6 in region"]
    #[inline]
    pub fn sr6(&self) -> SR6R {
        SR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Include or exclude subregion 7 in region"]
    #[inline]
    pub fn sr7(&self) -> SR7R {
        SR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Include or exclude subregion 8 in region"]
    #[inline]
    pub fn sr8(&self) -> SR8R {
        SR8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Include or exclude subregion 9 in region"]
    #[inline]
    pub fn sr9(&self) -> SR9R {
        SR9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Include or exclude subregion 10 in region"]
    #[inline]
    pub fn sr10(&self) -> SR10R {
        SR10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Include or exclude subregion 11 in region"]
    #[inline]
    pub fn sr11(&self) -> SR11R {
        SR11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Include or exclude subregion 12 in region"]
    #[inline]
    pub fn sr12(&self) -> SR12R {
        SR12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Include or exclude subregion 13 in region"]
    #[inline]
    pub fn sr13(&self) -> SR13R {
        SR13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Include or exclude subregion 14 in region"]
    #[inline]
    pub fn sr14(&self) -> SR14R {
        SR14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Include or exclude subregion 15 in region"]
    #[inline]
    pub fn sr15(&self) -> SR15R {
        SR15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Include or exclude subregion 16 in region"]
    #[inline]
    pub fn sr16(&self) -> SR16R {
        SR16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Include or exclude subregion 17 in region"]
    #[inline]
    pub fn sr17(&self) -> SR17R {
        SR17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Include or exclude subregion 18 in region"]
    #[inline]
    pub fn sr18(&self) -> SR18R {
        SR18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Include or exclude subregion 19 in region"]
    #[inline]
    pub fn sr19(&self) -> SR19R {
        SR19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Include or exclude subregion 20 in region"]
    #[inline]
    pub fn sr20(&self) -> SR20R {
        SR20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Include or exclude subregion 21 in region"]
    #[inline]
    pub fn sr21(&self) -> SR21R {
        SR21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Include or exclude subregion 22 in region"]
    #[inline]
    pub fn sr22(&self) -> SR22R {
        SR22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Include or exclude subregion 23 in region"]
    #[inline]
    pub fn sr23(&self) -> SR23R {
        SR23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Include or exclude subregion 24 in region"]
    #[inline]
    pub fn sr24(&self) -> SR24R {
        SR24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Include or exclude subregion 25 in region"]
    #[inline]
    pub fn sr25(&self) -> SR25R {
        SR25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Include or exclude subregion 26 in region"]
    #[inline]
    pub fn sr26(&self) -> SR26R {
        SR26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Include or exclude subregion 27 in region"]
    #[inline]
    pub fn sr27(&self) -> SR27R {
        SR27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Include or exclude subregion 28 in region"]
    #[inline]
    pub fn sr28(&self) -> SR28R {
        SR28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Include or exclude subregion 29 in region"]
    #[inline]
    pub fn sr29(&self) -> SR29R {
        SR29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Include or exclude subregion 30 in region"]
    #[inline]
    pub fn sr30(&self) -> SR30R {
        SR30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Include or exclude subregion 31 in region"]
    #[inline]
    pub fn sr31(&self) -> SR31R {
        SR31R::_from({
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
    #[doc = "Bit 0 - Include or exclude subregion 0 in region"]
    #[inline]
    pub fn sr0(&mut self) -> _SR0W {
        _SR0W { w: self }
    }
    #[doc = "Bit 1 - Include or exclude subregion 1 in region"]
    #[inline]
    pub fn sr1(&mut self) -> _SR1W {
        _SR1W { w: self }
    }
    #[doc = "Bit 2 - Include or exclude subregion 2 in region"]
    #[inline]
    pub fn sr2(&mut self) -> _SR2W {
        _SR2W { w: self }
    }
    #[doc = "Bit 3 - Include or exclude subregion 3 in region"]
    #[inline]
    pub fn sr3(&mut self) -> _SR3W {
        _SR3W { w: self }
    }
    #[doc = "Bit 4 - Include or exclude subregion 4 in region"]
    #[inline]
    pub fn sr4(&mut self) -> _SR4W {
        _SR4W { w: self }
    }
    #[doc = "Bit 5 - Include or exclude subregion 5 in region"]
    #[inline]
    pub fn sr5(&mut self) -> _SR5W {
        _SR5W { w: self }
    }
    #[doc = "Bit 6 - Include or exclude subregion 6 in region"]
    #[inline]
    pub fn sr6(&mut self) -> _SR6W {
        _SR6W { w: self }
    }
    #[doc = "Bit 7 - Include or exclude subregion 7 in region"]
    #[inline]
    pub fn sr7(&mut self) -> _SR7W {
        _SR7W { w: self }
    }
    #[doc = "Bit 8 - Include or exclude subregion 8 in region"]
    #[inline]
    pub fn sr8(&mut self) -> _SR8W {
        _SR8W { w: self }
    }
    #[doc = "Bit 9 - Include or exclude subregion 9 in region"]
    #[inline]
    pub fn sr9(&mut self) -> _SR9W {
        _SR9W { w: self }
    }
    #[doc = "Bit 10 - Include or exclude subregion 10 in region"]
    #[inline]
    pub fn sr10(&mut self) -> _SR10W {
        _SR10W { w: self }
    }
    #[doc = "Bit 11 - Include or exclude subregion 11 in region"]
    #[inline]
    pub fn sr11(&mut self) -> _SR11W {
        _SR11W { w: self }
    }
    #[doc = "Bit 12 - Include or exclude subregion 12 in region"]
    #[inline]
    pub fn sr12(&mut self) -> _SR12W {
        _SR12W { w: self }
    }
    #[doc = "Bit 13 - Include or exclude subregion 13 in region"]
    #[inline]
    pub fn sr13(&mut self) -> _SR13W {
        _SR13W { w: self }
    }
    #[doc = "Bit 14 - Include or exclude subregion 14 in region"]
    #[inline]
    pub fn sr14(&mut self) -> _SR14W {
        _SR14W { w: self }
    }
    #[doc = "Bit 15 - Include or exclude subregion 15 in region"]
    #[inline]
    pub fn sr15(&mut self) -> _SR15W {
        _SR15W { w: self }
    }
    #[doc = "Bit 16 - Include or exclude subregion 16 in region"]
    #[inline]
    pub fn sr16(&mut self) -> _SR16W {
        _SR16W { w: self }
    }
    #[doc = "Bit 17 - Include or exclude subregion 17 in region"]
    #[inline]
    pub fn sr17(&mut self) -> _SR17W {
        _SR17W { w: self }
    }
    #[doc = "Bit 18 - Include or exclude subregion 18 in region"]
    #[inline]
    pub fn sr18(&mut self) -> _SR18W {
        _SR18W { w: self }
    }
    #[doc = "Bit 19 - Include or exclude subregion 19 in region"]
    #[inline]
    pub fn sr19(&mut self) -> _SR19W {
        _SR19W { w: self }
    }
    #[doc = "Bit 20 - Include or exclude subregion 20 in region"]
    #[inline]
    pub fn sr20(&mut self) -> _SR20W {
        _SR20W { w: self }
    }
    #[doc = "Bit 21 - Include or exclude subregion 21 in region"]
    #[inline]
    pub fn sr21(&mut self) -> _SR21W {
        _SR21W { w: self }
    }
    #[doc = "Bit 22 - Include or exclude subregion 22 in region"]
    #[inline]
    pub fn sr22(&mut self) -> _SR22W {
        _SR22W { w: self }
    }
    #[doc = "Bit 23 - Include or exclude subregion 23 in region"]
    #[inline]
    pub fn sr23(&mut self) -> _SR23W {
        _SR23W { w: self }
    }
    #[doc = "Bit 24 - Include or exclude subregion 24 in region"]
    #[inline]
    pub fn sr24(&mut self) -> _SR24W {
        _SR24W { w: self }
    }
    #[doc = "Bit 25 - Include or exclude subregion 25 in region"]
    #[inline]
    pub fn sr25(&mut self) -> _SR25W {
        _SR25W { w: self }
    }
    #[doc = "Bit 26 - Include or exclude subregion 26 in region"]
    #[inline]
    pub fn sr26(&mut self) -> _SR26W {
        _SR26W { w: self }
    }
    #[doc = "Bit 27 - Include or exclude subregion 27 in region"]
    #[inline]
    pub fn sr27(&mut self) -> _SR27W {
        _SR27W { w: self }
    }
    #[doc = "Bit 28 - Include or exclude subregion 28 in region"]
    #[inline]
    pub fn sr28(&mut self) -> _SR28W {
        _SR28W { w: self }
    }
    #[doc = "Bit 29 - Include or exclude subregion 29 in region"]
    #[inline]
    pub fn sr29(&mut self) -> _SR29W {
        _SR29W { w: self }
    }
    #[doc = "Bit 30 - Include or exclude subregion 30 in region"]
    #[inline]
    pub fn sr30(&mut self) -> _SR30W {
        _SR30W { w: self }
    }
    #[doc = "Bit 31 - Include or exclude subregion 31 in region"]
    #[inline]
    pub fn sr31(&mut self) -> _SR31W {
        _SR31W { w: self }
    }
}
