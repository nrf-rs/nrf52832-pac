#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUBSTATRA {
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR0R::NOACCESS => false,
            SR0R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR0R {
        match value {
            false => SR0R::NOACCESS,
            true => SR0R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR0R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR0R::ACCESS
    }
}
#[doc = "Possible values of the field `SR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR1R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR1R::NOACCESS => false,
            SR1R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR1R {
        match value {
            false => SR1R::NOACCESS,
            true => SR1R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR1R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR1R::ACCESS
    }
}
#[doc = "Possible values of the field `SR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR2R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR2R::NOACCESS => false,
            SR2R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR2R {
        match value {
            false => SR2R::NOACCESS,
            true => SR2R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR2R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR2R::ACCESS
    }
}
#[doc = "Possible values of the field `SR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR3R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR3R::NOACCESS => false,
            SR3R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR3R {
        match value {
            false => SR3R::NOACCESS,
            true => SR3R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR3R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR3R::ACCESS
    }
}
#[doc = "Possible values of the field `SR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR4R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR4R::NOACCESS => false,
            SR4R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR4R {
        match value {
            false => SR4R::NOACCESS,
            true => SR4R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR4R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR4R::ACCESS
    }
}
#[doc = "Possible values of the field `SR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR5R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR5R::NOACCESS => false,
            SR5R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR5R {
        match value {
            false => SR5R::NOACCESS,
            true => SR5R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR5R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR5R::ACCESS
    }
}
#[doc = "Possible values of the field `SR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR6R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR6R::NOACCESS => false,
            SR6R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR6R {
        match value {
            false => SR6R::NOACCESS,
            true => SR6R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR6R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR6R::ACCESS
    }
}
#[doc = "Possible values of the field `SR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR7R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR7R::NOACCESS => false,
            SR7R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR7R {
        match value {
            false => SR7R::NOACCESS,
            true => SR7R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR7R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR7R::ACCESS
    }
}
#[doc = "Possible values of the field `SR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR8R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR8R::NOACCESS => false,
            SR8R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR8R {
        match value {
            false => SR8R::NOACCESS,
            true => SR8R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR8R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR8R::ACCESS
    }
}
#[doc = "Possible values of the field `SR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR9R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR9R::NOACCESS => false,
            SR9R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR9R {
        match value {
            false => SR9R::NOACCESS,
            true => SR9R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR9R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR9R::ACCESS
    }
}
#[doc = "Possible values of the field `SR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR10R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR10R::NOACCESS => false,
            SR10R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR10R {
        match value {
            false => SR10R::NOACCESS,
            true => SR10R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR10R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR10R::ACCESS
    }
}
#[doc = "Possible values of the field `SR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR11R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR11R::NOACCESS => false,
            SR11R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR11R {
        match value {
            false => SR11R::NOACCESS,
            true => SR11R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR11R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR11R::ACCESS
    }
}
#[doc = "Possible values of the field `SR12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR12R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR12R::NOACCESS => false,
            SR12R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR12R {
        match value {
            false => SR12R::NOACCESS,
            true => SR12R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR12R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR12R::ACCESS
    }
}
#[doc = "Possible values of the field `SR13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR13R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR13R::NOACCESS => false,
            SR13R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR13R {
        match value {
            false => SR13R::NOACCESS,
            true => SR13R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR13R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR13R::ACCESS
    }
}
#[doc = "Possible values of the field `SR14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR14R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR14R::NOACCESS => false,
            SR14R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR14R {
        match value {
            false => SR14R::NOACCESS,
            true => SR14R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR14R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR14R::ACCESS
    }
}
#[doc = "Possible values of the field `SR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR15R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR15R::NOACCESS => false,
            SR15R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR15R {
        match value {
            false => SR15R::NOACCESS,
            true => SR15R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR15R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR15R::ACCESS
    }
}
#[doc = "Possible values of the field `SR16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR16R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR16R::NOACCESS => false,
            SR16R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR16R {
        match value {
            false => SR16R::NOACCESS,
            true => SR16R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR16R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR16R::ACCESS
    }
}
#[doc = "Possible values of the field `SR17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR17R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR17R::NOACCESS => false,
            SR17R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR17R {
        match value {
            false => SR17R::NOACCESS,
            true => SR17R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR17R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR17R::ACCESS
    }
}
#[doc = "Possible values of the field `SR18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR18R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR18R::NOACCESS => false,
            SR18R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR18R {
        match value {
            false => SR18R::NOACCESS,
            true => SR18R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR18R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR18R::ACCESS
    }
}
#[doc = "Possible values of the field `SR19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR19R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR19R::NOACCESS => false,
            SR19R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR19R {
        match value {
            false => SR19R::NOACCESS,
            true => SR19R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR19R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR19R::ACCESS
    }
}
#[doc = "Possible values of the field `SR20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR20R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR20R::NOACCESS => false,
            SR20R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR20R {
        match value {
            false => SR20R::NOACCESS,
            true => SR20R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR20R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR20R::ACCESS
    }
}
#[doc = "Possible values of the field `SR21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR21R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR21R::NOACCESS => false,
            SR21R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR21R {
        match value {
            false => SR21R::NOACCESS,
            true => SR21R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR21R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR21R::ACCESS
    }
}
#[doc = "Possible values of the field `SR22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR22R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR22R::NOACCESS => false,
            SR22R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR22R {
        match value {
            false => SR22R::NOACCESS,
            true => SR22R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR22R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR22R::ACCESS
    }
}
#[doc = "Possible values of the field `SR23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR23R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR23R::NOACCESS => false,
            SR23R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR23R {
        match value {
            false => SR23R::NOACCESS,
            true => SR23R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR23R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR23R::ACCESS
    }
}
#[doc = "Possible values of the field `SR24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR24R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR24R::NOACCESS => false,
            SR24R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR24R {
        match value {
            false => SR24R::NOACCESS,
            true => SR24R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR24R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR24R::ACCESS
    }
}
#[doc = "Possible values of the field `SR25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR25R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR25R::NOACCESS => false,
            SR25R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR25R {
        match value {
            false => SR25R::NOACCESS,
            true => SR25R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR25R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR25R::ACCESS
    }
}
#[doc = "Possible values of the field `SR26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR26R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR26R::NOACCESS => false,
            SR26R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR26R {
        match value {
            false => SR26R::NOACCESS,
            true => SR26R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR26R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR26R::ACCESS
    }
}
#[doc = "Possible values of the field `SR27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR27R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR27R::NOACCESS => false,
            SR27R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR27R {
        match value {
            false => SR27R::NOACCESS,
            true => SR27R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR27R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR27R::ACCESS
    }
}
#[doc = "Possible values of the field `SR28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR28R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR28R::NOACCESS => false,
            SR28R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR28R {
        match value {
            false => SR28R::NOACCESS,
            true => SR28R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR28R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR28R::ACCESS
    }
}
#[doc = "Possible values of the field `SR29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR29R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR29R::NOACCESS => false,
            SR29R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR29R {
        match value {
            false => SR29R::NOACCESS,
            true => SR29R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR29R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR29R::ACCESS
    }
}
#[doc = "Possible values of the field `SR30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR30R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR30R::NOACCESS => false,
            SR30R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR30R {
        match value {
            false => SR30R::NOACCESS,
            true => SR30R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR30R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR30R::ACCESS
    }
}
#[doc = "Possible values of the field `SR31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR31R {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
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
            SR31R::NOACCESS => false,
            SR31R::ACCESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR31R {
        match value {
            false => SR31R::NOACCESS,
            true => SR31R::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline]
    pub fn is_no_access(&self) -> bool {
        *self == SR31R::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline]
    pub fn is_access(&self) -> bool {
        *self == SR31R::ACCESS
    }
}
#[doc = "Values that can be written to the field `SR0`"]
pub enum SR0W {
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR0W::NOACCESS => false,
            SR0W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR0W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR0W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR1W::NOACCESS => false,
            SR1W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR1W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR1W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR2W::NOACCESS => false,
            SR2W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR2W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR2W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR3W::NOACCESS => false,
            SR3W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR3W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR3W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR4W::NOACCESS => false,
            SR4W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR4W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR4W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR5W::NOACCESS => false,
            SR5W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR5W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR5W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR6W::NOACCESS => false,
            SR6W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR6W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR6W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR7W::NOACCESS => false,
            SR7W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR7W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR7W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR8W::NOACCESS => false,
            SR8W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR8W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR8W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR9W::NOACCESS => false,
            SR9W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR9W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR9W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR10W::NOACCESS => false,
            SR10W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR10W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR10W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR11W::NOACCESS => false,
            SR11W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR11W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR11W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR12W::NOACCESS => false,
            SR12W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR12W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR12W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR13W::NOACCESS => false,
            SR13W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR13W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR13W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR14W::NOACCESS => false,
            SR14W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR14W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR14W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR15W::NOACCESS => false,
            SR15W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR15W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR15W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR16W::NOACCESS => false,
            SR16W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR16W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR16W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR17W::NOACCESS => false,
            SR17W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR17W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR17W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR18W::NOACCESS => false,
            SR18W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR18W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR18W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR19W::NOACCESS => false,
            SR19W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR19W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR19W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR20W::NOACCESS => false,
            SR20W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR20W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR20W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR21W::NOACCESS => false,
            SR21W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR21W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR21W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR22W::NOACCESS => false,
            SR22W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR22W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR22W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR23W::NOACCESS => false,
            SR23W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR23W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR23W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR24W::NOACCESS => false,
            SR24W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR24W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR24W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR25W::NOACCESS => false,
            SR25W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR25W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR25W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR26W::NOACCESS => false,
            SR26W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR26W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR26W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR27W::NOACCESS => false,
            SR27W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR27W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR27W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR28W::NOACCESS => false,
            SR28W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR28W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR28W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR29W::NOACCESS => false,
            SR29W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR29W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR29W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR30W::NOACCESS => false,
            SR30W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR30W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR30W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No read access occurred in this subregion"]
    NOACCESS,
    #[doc = "Read access(es) occurred in this subregion"]
    ACCESS,
}
impl SR31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SR31W::NOACCESS => false,
            SR31W::ACCESS => true,
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
    #[doc = "No read access occurred in this subregion"]
    #[inline]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR31W::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline]
    pub fn access(self) -> &'a mut W {
        self.variant(SR31W::ACCESS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Subregion 0 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr0(&self) -> SR0R {
        SR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Subregion 1 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr1(&self) -> SR1R {
        SR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Subregion 2 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr2(&self) -> SR2R {
        SR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Subregion 3 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr3(&self) -> SR3R {
        SR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Subregion 4 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr4(&self) -> SR4R {
        SR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Subregion 5 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr5(&self) -> SR5R {
        SR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Subregion 6 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr6(&self) -> SR6R {
        SR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Subregion 7 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr7(&self) -> SR7R {
        SR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Subregion 8 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr8(&self) -> SR8R {
        SR8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Subregion 9 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr9(&self) -> SR9R {
        SR9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Subregion 10 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr10(&self) -> SR10R {
        SR10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Subregion 11 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr11(&self) -> SR11R {
        SR11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Subregion 12 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr12(&self) -> SR12R {
        SR12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Subregion 13 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr13(&self) -> SR13R {
        SR13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Subregion 14 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr14(&self) -> SR14R {
        SR14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Subregion 15 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr15(&self) -> SR15R {
        SR15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Subregion 16 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr16(&self) -> SR16R {
        SR16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Subregion 17 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr17(&self) -> SR17R {
        SR17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Subregion 18 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr18(&self) -> SR18R {
        SR18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Subregion 19 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr19(&self) -> SR19R {
        SR19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Subregion 20 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr20(&self) -> SR20R {
        SR20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Subregion 21 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr21(&self) -> SR21R {
        SR21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Subregion 22 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr22(&self) -> SR22R {
        SR22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Subregion 23 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr23(&self) -> SR23R {
        SR23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Subregion 24 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr24(&self) -> SR24R {
        SR24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Subregion 25 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr25(&self) -> SR25R {
        SR25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Subregion 26 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr26(&self) -> SR26R {
        SR26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Subregion 27 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr27(&self) -> SR27R {
        SR27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Subregion 28 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr28(&self) -> SR28R {
        SR28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Subregion 29 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr29(&self) -> SR29R {
        SR29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Subregion 30 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr30(&self) -> SR30R {
        SR30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Subregion 31 in region n (write '1' to clear)"]
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
    #[doc = "Bit 0 - Subregion 0 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr0(&mut self) -> _SR0W {
        _SR0W { w: self }
    }
    #[doc = "Bit 1 - Subregion 1 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr1(&mut self) -> _SR1W {
        _SR1W { w: self }
    }
    #[doc = "Bit 2 - Subregion 2 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr2(&mut self) -> _SR2W {
        _SR2W { w: self }
    }
    #[doc = "Bit 3 - Subregion 3 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr3(&mut self) -> _SR3W {
        _SR3W { w: self }
    }
    #[doc = "Bit 4 - Subregion 4 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr4(&mut self) -> _SR4W {
        _SR4W { w: self }
    }
    #[doc = "Bit 5 - Subregion 5 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr5(&mut self) -> _SR5W {
        _SR5W { w: self }
    }
    #[doc = "Bit 6 - Subregion 6 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr6(&mut self) -> _SR6W {
        _SR6W { w: self }
    }
    #[doc = "Bit 7 - Subregion 7 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr7(&mut self) -> _SR7W {
        _SR7W { w: self }
    }
    #[doc = "Bit 8 - Subregion 8 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr8(&mut self) -> _SR8W {
        _SR8W { w: self }
    }
    #[doc = "Bit 9 - Subregion 9 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr9(&mut self) -> _SR9W {
        _SR9W { w: self }
    }
    #[doc = "Bit 10 - Subregion 10 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr10(&mut self) -> _SR10W {
        _SR10W { w: self }
    }
    #[doc = "Bit 11 - Subregion 11 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr11(&mut self) -> _SR11W {
        _SR11W { w: self }
    }
    #[doc = "Bit 12 - Subregion 12 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr12(&mut self) -> _SR12W {
        _SR12W { w: self }
    }
    #[doc = "Bit 13 - Subregion 13 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr13(&mut self) -> _SR13W {
        _SR13W { w: self }
    }
    #[doc = "Bit 14 - Subregion 14 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr14(&mut self) -> _SR14W {
        _SR14W { w: self }
    }
    #[doc = "Bit 15 - Subregion 15 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr15(&mut self) -> _SR15W {
        _SR15W { w: self }
    }
    #[doc = "Bit 16 - Subregion 16 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr16(&mut self) -> _SR16W {
        _SR16W { w: self }
    }
    #[doc = "Bit 17 - Subregion 17 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr17(&mut self) -> _SR17W {
        _SR17W { w: self }
    }
    #[doc = "Bit 18 - Subregion 18 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr18(&mut self) -> _SR18W {
        _SR18W { w: self }
    }
    #[doc = "Bit 19 - Subregion 19 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr19(&mut self) -> _SR19W {
        _SR19W { w: self }
    }
    #[doc = "Bit 20 - Subregion 20 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr20(&mut self) -> _SR20W {
        _SR20W { w: self }
    }
    #[doc = "Bit 21 - Subregion 21 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr21(&mut self) -> _SR21W {
        _SR21W { w: self }
    }
    #[doc = "Bit 22 - Subregion 22 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr22(&mut self) -> _SR22W {
        _SR22W { w: self }
    }
    #[doc = "Bit 23 - Subregion 23 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr23(&mut self) -> _SR23W {
        _SR23W { w: self }
    }
    #[doc = "Bit 24 - Subregion 24 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr24(&mut self) -> _SR24W {
        _SR24W { w: self }
    }
    #[doc = "Bit 25 - Subregion 25 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr25(&mut self) -> _SR25W {
        _SR25W { w: self }
    }
    #[doc = "Bit 26 - Subregion 26 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr26(&mut self) -> _SR26W {
        _SR26W { w: self }
    }
    #[doc = "Bit 27 - Subregion 27 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr27(&mut self) -> _SR27W {
        _SR27W { w: self }
    }
    #[doc = "Bit 28 - Subregion 28 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr28(&mut self) -> _SR28W {
        _SR28W { w: self }
    }
    #[doc = "Bit 29 - Subregion 29 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr29(&mut self) -> _SR29W {
        _SR29W { w: self }
    }
    #[doc = "Bit 30 - Subregion 30 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr30(&mut self) -> _SR30W {
        _SR30W { w: self }
    }
    #[doc = "Bit 31 - Subregion 31 in region n (write '1' to clear)"]
    #[inline]
    pub fn sr31(&mut self) -> _SR31W {
        _SR31W { w: self }
    }
}
