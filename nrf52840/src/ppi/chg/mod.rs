#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHG {
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH0R::EXCLUDED => false,
            CH0R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0R {
        match value {
            false => CH0R::EXCLUDED,
            true => CH0R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH0R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH0R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH1R::EXCLUDED => false,
            CH1R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1R {
        match value {
            false => CH1R::EXCLUDED,
            true => CH1R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH1R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH1R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH2R::EXCLUDED => false,
            CH2R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2R {
        match value {
            false => CH2R::EXCLUDED,
            true => CH2R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH2R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH2R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH3R::EXCLUDED => false,
            CH3R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3R {
        match value {
            false => CH3R::EXCLUDED,
            true => CH3R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH3R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH3R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH4R::EXCLUDED => false,
            CH4R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4R {
        match value {
            false => CH4R::EXCLUDED,
            true => CH4R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH4R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH4R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH5R::EXCLUDED => false,
            CH5R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5R {
        match value {
            false => CH5R::EXCLUDED,
            true => CH5R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH5R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH5R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH6R::EXCLUDED => false,
            CH6R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6R {
        match value {
            false => CH6R::EXCLUDED,
            true => CH6R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH6R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH6R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH7R::EXCLUDED => false,
            CH7R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7R {
        match value {
            false => CH7R::EXCLUDED,
            true => CH7R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH7R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH7R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH8R::EXCLUDED => false,
            CH8R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8R {
        match value {
            false => CH8R::EXCLUDED,
            true => CH8R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH8R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH8R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH9R::EXCLUDED => false,
            CH9R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH9R {
        match value {
            false => CH9R::EXCLUDED,
            true => CH9R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH9R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH9R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH10R::EXCLUDED => false,
            CH10R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH10R {
        match value {
            false => CH10R::EXCLUDED,
            true => CH10R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH10R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH10R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH11R::EXCLUDED => false,
            CH11R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH11R {
        match value {
            false => CH11R::EXCLUDED,
            true => CH11R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH11R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH11R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH12R::EXCLUDED => false,
            CH12R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH12R {
        match value {
            false => CH12R::EXCLUDED,
            true => CH12R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH12R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH12R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH13R::EXCLUDED => false,
            CH13R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH13R {
        match value {
            false => CH13R::EXCLUDED,
            true => CH13R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH13R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH13R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH14R::EXCLUDED => false,
            CH14R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH14R {
        match value {
            false => CH14R::EXCLUDED,
            true => CH14R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH14R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH14R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH15R::EXCLUDED => false,
            CH15R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH15R {
        match value {
            false => CH15R::EXCLUDED,
            true => CH15R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH15R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH15R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH16R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH16R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH16R::EXCLUDED => false,
            CH16R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH16R {
        match value {
            false => CH16R::EXCLUDED,
            true => CH16R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH16R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH16R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH17R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH17R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH17R::EXCLUDED => false,
            CH17R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH17R {
        match value {
            false => CH17R::EXCLUDED,
            true => CH17R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH17R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH17R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH18R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH18R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH18R::EXCLUDED => false,
            CH18R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH18R {
        match value {
            false => CH18R::EXCLUDED,
            true => CH18R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH18R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH18R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH19R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH19R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH19R::EXCLUDED => false,
            CH19R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH19R {
        match value {
            false => CH19R::EXCLUDED,
            true => CH19R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH19R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH19R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH20R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH20R::EXCLUDED => false,
            CH20R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH20R {
        match value {
            false => CH20R::EXCLUDED,
            true => CH20R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH20R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH20R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH21R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH21R::EXCLUDED => false,
            CH21R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH21R {
        match value {
            false => CH21R::EXCLUDED,
            true => CH21R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH21R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH21R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH22R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH22R::EXCLUDED => false,
            CH22R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH22R {
        match value {
            false => CH22R::EXCLUDED,
            true => CH22R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH22R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH22R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH23R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH23R::EXCLUDED => false,
            CH23R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH23R {
        match value {
            false => CH23R::EXCLUDED,
            true => CH23R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH23R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH23R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH24R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH24R::EXCLUDED => false,
            CH24R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH24R {
        match value {
            false => CH24R::EXCLUDED,
            true => CH24R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH24R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH24R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH25R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH25R::EXCLUDED => false,
            CH25R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH25R {
        match value {
            false => CH25R::EXCLUDED,
            true => CH25R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH25R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH25R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH26R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH26R::EXCLUDED => false,
            CH26R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH26R {
        match value {
            false => CH26R::EXCLUDED,
            true => CH26R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH26R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH26R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH27R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH27R::EXCLUDED => false,
            CH27R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH27R {
        match value {
            false => CH27R::EXCLUDED,
            true => CH27R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH27R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH27R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH28R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH28R::EXCLUDED => false,
            CH28R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH28R {
        match value {
            false => CH28R::EXCLUDED,
            true => CH28R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH28R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH28R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH29R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH29R::EXCLUDED => false,
            CH29R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH29R {
        match value {
            false => CH29R::EXCLUDED,
            true => CH29R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH29R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH29R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH30R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH30R::EXCLUDED => false,
            CH30R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH30R {
        match value {
            false => CH30R::EXCLUDED,
            true => CH30R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH30R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH30R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH31R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH31R::EXCLUDED => false,
            CH31R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH31R {
        match value {
            false => CH31R::EXCLUDED,
            true => CH31R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH31R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH31R::INCLUDED
    }
}
#[doc = "Values that can be written to the field `CH0`"]
pub enum CH0W {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0W::EXCLUDED => false,
            CH0W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH0W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH0W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1W::EXCLUDED => false,
            CH1W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH1W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH1W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2W::EXCLUDED => false,
            CH2W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH2W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH2W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3W::EXCLUDED => false,
            CH3W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH3W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH3W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4W::EXCLUDED => false,
            CH4W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH4W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH4W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5W::EXCLUDED => false,
            CH5W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH5W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH5W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6W::EXCLUDED => false,
            CH6W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH6W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH6W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7W::EXCLUDED => false,
            CH7W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH7W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH7W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8W::EXCLUDED => false,
            CH8W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH8W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH8W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH9W::EXCLUDED => false,
            CH9W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH9W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH9W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH10W::EXCLUDED => false,
            CH10W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH10W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH10W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH11W::EXCLUDED => false,
            CH11W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH11W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH11W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH12W::EXCLUDED => false,
            CH12W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH12W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH12W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH13W::EXCLUDED => false,
            CH13W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH13W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH13W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH14W::EXCLUDED => false,
            CH14W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH14W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH14W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH15W::EXCLUDED => false,
            CH15W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH15W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH15W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `CH16`"]
pub enum CH16W {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH16W::EXCLUDED => false,
            CH16W::INCLUDED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH16W<'a> {
    w: &'a mut W,
}
impl<'a> _CH16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH16W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH16W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `CH17`"]
pub enum CH17W {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH17W::EXCLUDED => false,
            CH17W::INCLUDED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH17W<'a> {
    w: &'a mut W,
}
impl<'a> _CH17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH17W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH17W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `CH18`"]
pub enum CH18W {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH18W::EXCLUDED => false,
            CH18W::INCLUDED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH18W<'a> {
    w: &'a mut W,
}
impl<'a> _CH18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH18W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH18W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `CH19`"]
pub enum CH19W {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH19W::EXCLUDED => false,
            CH19W::INCLUDED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH19W<'a> {
    w: &'a mut W,
}
impl<'a> _CH19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH19W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH19W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `CH20`"]
pub enum CH20W {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH20W::EXCLUDED => false,
            CH20W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH20W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH20W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH21W::EXCLUDED => false,
            CH21W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH21W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH21W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH22W::EXCLUDED => false,
            CH22W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH22W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH22W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH23W::EXCLUDED => false,
            CH23W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH23W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH23W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH24W::EXCLUDED => false,
            CH24W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH24W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH24W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH25W::EXCLUDED => false,
            CH25W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH25W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH25W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH26W::EXCLUDED => false,
            CH26W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH26W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH26W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH27W::EXCLUDED => false,
            CH27W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH27W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH27W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH28W::EXCLUDED => false,
            CH28W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH28W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH28W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH29W::EXCLUDED => false,
            CH29W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH29W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH29W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH30W::EXCLUDED => false,
            CH30W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH30W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH30W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH31W::EXCLUDED => false,
            CH31W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH31W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH31W::INCLUDED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline]
    pub fn ch0(&self) -> CH0R {
        CH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline]
    pub fn ch1(&self) -> CH1R {
        CH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline]
    pub fn ch2(&self) -> CH2R {
        CH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline]
    pub fn ch3(&self) -> CH3R {
        CH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline]
    pub fn ch4(&self) -> CH4R {
        CH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline]
    pub fn ch5(&self) -> CH5R {
        CH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline]
    pub fn ch6(&self) -> CH6R {
        CH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline]
    pub fn ch7(&self) -> CH7R {
        CH7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline]
    pub fn ch8(&self) -> CH8R {
        CH8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline]
    pub fn ch9(&self) -> CH9R {
        CH9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline]
    pub fn ch10(&self) -> CH10R {
        CH10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline]
    pub fn ch11(&self) -> CH11R {
        CH11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline]
    pub fn ch12(&self) -> CH12R {
        CH12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline]
    pub fn ch13(&self) -> CH13R {
        CH13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline]
    pub fn ch14(&self) -> CH14R {
        CH14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
    #[inline]
    pub fn ch15(&self) -> CH15R {
        CH15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Include or exclude channel 16"]
    #[inline]
    pub fn ch16(&self) -> CH16R {
        CH16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Include or exclude channel 17"]
    #[inline]
    pub fn ch17(&self) -> CH17R {
        CH17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Include or exclude channel 18"]
    #[inline]
    pub fn ch18(&self) -> CH18R {
        CH18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Include or exclude channel 19"]
    #[inline]
    pub fn ch19(&self) -> CH19R {
        CH19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Include or exclude channel 20"]
    #[inline]
    pub fn ch20(&self) -> CH20R {
        CH20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Include or exclude channel 21"]
    #[inline]
    pub fn ch21(&self) -> CH21R {
        CH21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Include or exclude channel 22"]
    #[inline]
    pub fn ch22(&self) -> CH22R {
        CH22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Include or exclude channel 23"]
    #[inline]
    pub fn ch23(&self) -> CH23R {
        CH23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Include or exclude channel 24"]
    #[inline]
    pub fn ch24(&self) -> CH24R {
        CH24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Include or exclude channel 25"]
    #[inline]
    pub fn ch25(&self) -> CH25R {
        CH25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Include or exclude channel 26"]
    #[inline]
    pub fn ch26(&self) -> CH26R {
        CH26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Include or exclude channel 27"]
    #[inline]
    pub fn ch27(&self) -> CH27R {
        CH27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Include or exclude channel 28"]
    #[inline]
    pub fn ch28(&self) -> CH28R {
        CH28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Include or exclude channel 29"]
    #[inline]
    pub fn ch29(&self) -> CH29R {
        CH29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Include or exclude channel 30"]
    #[inline]
    pub fn ch30(&self) -> CH30R {
        CH30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Include or exclude channel 31"]
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
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline]
    pub fn ch0(&mut self) -> _CH0W {
        _CH0W { w: self }
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline]
    pub fn ch1(&mut self) -> _CH1W {
        _CH1W { w: self }
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline]
    pub fn ch2(&mut self) -> _CH2W {
        _CH2W { w: self }
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline]
    pub fn ch3(&mut self) -> _CH3W {
        _CH3W { w: self }
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline]
    pub fn ch4(&mut self) -> _CH4W {
        _CH4W { w: self }
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline]
    pub fn ch5(&mut self) -> _CH5W {
        _CH5W { w: self }
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline]
    pub fn ch6(&mut self) -> _CH6W {
        _CH6W { w: self }
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline]
    pub fn ch7(&mut self) -> _CH7W {
        _CH7W { w: self }
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline]
    pub fn ch8(&mut self) -> _CH8W {
        _CH8W { w: self }
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline]
    pub fn ch9(&mut self) -> _CH9W {
        _CH9W { w: self }
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline]
    pub fn ch10(&mut self) -> _CH10W {
        _CH10W { w: self }
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline]
    pub fn ch11(&mut self) -> _CH11W {
        _CH11W { w: self }
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline]
    pub fn ch12(&mut self) -> _CH12W {
        _CH12W { w: self }
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline]
    pub fn ch13(&mut self) -> _CH13W {
        _CH13W { w: self }
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline]
    pub fn ch14(&mut self) -> _CH14W {
        _CH14W { w: self }
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
    #[inline]
    pub fn ch15(&mut self) -> _CH15W {
        _CH15W { w: self }
    }
    #[doc = "Bit 16 - Include or exclude channel 16"]
    #[inline]
    pub fn ch16(&mut self) -> _CH16W {
        _CH16W { w: self }
    }
    #[doc = "Bit 17 - Include or exclude channel 17"]
    #[inline]
    pub fn ch17(&mut self) -> _CH17W {
        _CH17W { w: self }
    }
    #[doc = "Bit 18 - Include or exclude channel 18"]
    #[inline]
    pub fn ch18(&mut self) -> _CH18W {
        _CH18W { w: self }
    }
    #[doc = "Bit 19 - Include or exclude channel 19"]
    #[inline]
    pub fn ch19(&mut self) -> _CH19W {
        _CH19W { w: self }
    }
    #[doc = "Bit 20 - Include or exclude channel 20"]
    #[inline]
    pub fn ch20(&mut self) -> _CH20W {
        _CH20W { w: self }
    }
    #[doc = "Bit 21 - Include or exclude channel 21"]
    #[inline]
    pub fn ch21(&mut self) -> _CH21W {
        _CH21W { w: self }
    }
    #[doc = "Bit 22 - Include or exclude channel 22"]
    #[inline]
    pub fn ch22(&mut self) -> _CH22W {
        _CH22W { w: self }
    }
    #[doc = "Bit 23 - Include or exclude channel 23"]
    #[inline]
    pub fn ch23(&mut self) -> _CH23W {
        _CH23W { w: self }
    }
    #[doc = "Bit 24 - Include or exclude channel 24"]
    #[inline]
    pub fn ch24(&mut self) -> _CH24W {
        _CH24W { w: self }
    }
    #[doc = "Bit 25 - Include or exclude channel 25"]
    #[inline]
    pub fn ch25(&mut self) -> _CH25W {
        _CH25W { w: self }
    }
    #[doc = "Bit 26 - Include or exclude channel 26"]
    #[inline]
    pub fn ch26(&mut self) -> _CH26W {
        _CH26W { w: self }
    }
    #[doc = "Bit 27 - Include or exclude channel 27"]
    #[inline]
    pub fn ch27(&mut self) -> _CH27W {
        _CH27W { w: self }
    }
    #[doc = "Bit 28 - Include or exclude channel 28"]
    #[inline]
    pub fn ch28(&mut self) -> _CH28W {
        _CH28W { w: self }
    }
    #[doc = "Bit 29 - Include or exclude channel 29"]
    #[inline]
    pub fn ch29(&mut self) -> _CH29W {
        _CH29W { w: self }
    }
    #[doc = "Bit 30 - Include or exclude channel 30"]
    #[inline]
    pub fn ch30(&mut self) -> _CH30W {
        _CH30W { w: self }
    }
    #[doc = "Bit 31 - Include or exclude channel 31"]
    #[inline]
    pub fn ch31(&mut self) -> _CH31W {
        _CH31W { w: self }
    }
}
