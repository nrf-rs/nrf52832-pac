#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPSTATUS {
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
#[doc = "Possible values of the field `EPIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN0R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN0R::NODATA => false,
            EPIN0R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN0R {
        match value {
            false => EPIN0R::NODATA,
            true => EPIN0R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN0R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN0R::DATADONE
    }
}
#[doc = "Possible values of the field `EPIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN1R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN1R::NODATA => false,
            EPIN1R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN1R {
        match value {
            false => EPIN1R::NODATA,
            true => EPIN1R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN1R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN1R::DATADONE
    }
}
#[doc = "Possible values of the field `EPIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN2R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN2R::NODATA => false,
            EPIN2R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN2R {
        match value {
            false => EPIN2R::NODATA,
            true => EPIN2R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN2R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN2R::DATADONE
    }
}
#[doc = "Possible values of the field `EPIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN3R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN3R::NODATA => false,
            EPIN3R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN3R {
        match value {
            false => EPIN3R::NODATA,
            true => EPIN3R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN3R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN3R::DATADONE
    }
}
#[doc = "Possible values of the field `EPIN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN4R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN4R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN4R::NODATA => false,
            EPIN4R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN4R {
        match value {
            false => EPIN4R::NODATA,
            true => EPIN4R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN4R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN4R::DATADONE
    }
}
#[doc = "Possible values of the field `EPIN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN5R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN5R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN5R::NODATA => false,
            EPIN5R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN5R {
        match value {
            false => EPIN5R::NODATA,
            true => EPIN5R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN5R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN5R::DATADONE
    }
}
#[doc = "Possible values of the field `EPIN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN6R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN6R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN6R::NODATA => false,
            EPIN6R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN6R {
        match value {
            false => EPIN6R::NODATA,
            true => EPIN6R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN6R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN6R::DATADONE
    }
}
#[doc = "Possible values of the field `EPIN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN7R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN7R::NODATA => false,
            EPIN7R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN7R {
        match value {
            false => EPIN7R::NODATA,
            true => EPIN7R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN7R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN7R::DATADONE
    }
}
#[doc = "Possible values of the field `EPIN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN8R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN8R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPIN8R::NODATA => false,
            EPIN8R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN8R {
        match value {
            false => EPIN8R::NODATA,
            true => EPIN8R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN8R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN8R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT0R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT0R::NODATA => false,
            EPOUT0R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT0R {
        match value {
            false => EPOUT0R::NODATA,
            true => EPOUT0R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT0R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT0R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT1R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT1R::NODATA => false,
            EPOUT1R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT1R {
        match value {
            false => EPOUT1R::NODATA,
            true => EPOUT1R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT1R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT1R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT2R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT2R::NODATA => false,
            EPOUT2R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT2R {
        match value {
            false => EPOUT2R::NODATA,
            true => EPOUT2R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT2R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT2R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT3R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT3R::NODATA => false,
            EPOUT3R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT3R {
        match value {
            false => EPOUT3R::NODATA,
            true => EPOUT3R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT3R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT3R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT4R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT4R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT4R::NODATA => false,
            EPOUT4R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT4R {
        match value {
            false => EPOUT4R::NODATA,
            true => EPOUT4R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT4R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT4R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT5R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT5R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT5R::NODATA => false,
            EPOUT5R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT5R {
        match value {
            false => EPOUT5R::NODATA,
            true => EPOUT5R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT5R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT5R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT6R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT6R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT6R::NODATA => false,
            EPOUT6R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT6R {
        match value {
            false => EPOUT6R::NODATA,
            true => EPOUT6R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT6R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT6R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT7R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT7R::NODATA => false,
            EPOUT7R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT7R {
        match value {
            false => EPOUT7R::NODATA,
            true => EPOUT7R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT7R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT7R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT8R {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT8R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EPOUT8R::NODATA => false,
            EPOUT8R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT8R {
        match value {
            false => EPOUT8R::NODATA,
            true => EPOUT8R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT8R::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT8R::DATADONE
    }
}
#[doc = "Values that can be written to the field `EPIN0`"]
pub enum EPIN0W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN0W::NODATA => false,
            EPIN0W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN0W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN0W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPIN1`"]
pub enum EPIN1W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN1W::NODATA => false,
            EPIN1W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN1W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN1W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPIN2`"]
pub enum EPIN2W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN2W::NODATA => false,
            EPIN2W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN2W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN2W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPIN3`"]
pub enum EPIN3W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN3W::NODATA => false,
            EPIN3W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN3W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN3W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPIN4`"]
pub enum EPIN4W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN4W::NODATA => false,
            EPIN4W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN4W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN4W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPIN5`"]
pub enum EPIN5W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN5W::NODATA => false,
            EPIN5W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN5W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN5W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPIN6`"]
pub enum EPIN6W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN6W::NODATA => false,
            EPIN6W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN6W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN6W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPIN7`"]
pub enum EPIN7W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN7W::NODATA => false,
            EPIN7W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN7W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN7W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPIN8`"]
pub enum EPIN8W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPIN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN8W::NODATA => false,
            EPIN8W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPIN8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPIN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPIN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN8W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN8W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT0`"]
pub enum EPOUT0W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT0W::NODATA => false,
            EPOUT0W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT0W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT0W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT1`"]
pub enum EPOUT1W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT1W::NODATA => false,
            EPOUT1W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT1W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT1W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT2`"]
pub enum EPOUT2W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT2W::NODATA => false,
            EPOUT2W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT2W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT2W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT3`"]
pub enum EPOUT3W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT3W::NODATA => false,
            EPOUT3W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT3W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT3W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT4`"]
pub enum EPOUT4W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT4W::NODATA => false,
            EPOUT4W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT4W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT4W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT5`"]
pub enum EPOUT5W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT5W::NODATA => false,
            EPOUT5W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT5W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT5W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT6`"]
pub enum EPOUT6W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT6W::NODATA => false,
            EPOUT6W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT6W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT6W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT7`"]
pub enum EPOUT7W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT7W::NODATA => false,
            EPOUT7W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT7W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT7W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `EPOUT8`"]
pub enum EPOUT8W {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    NODATA,
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    DATADONE,
}
impl EPOUT8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT8W::NODATA => false,
            EPOUT8W::DATADONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPOUT8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPOUT8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPOUT8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT8W::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT8W::DATADONE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin0(&self) -> EPIN0R {
        EPIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin1(&self) -> EPIN1R {
        EPIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin2(&self) -> EPIN2R {
        EPIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin3(&self) -> EPIN3R {
        EPIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin4(&self) -> EPIN4R {
        EPIN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin5(&self) -> EPIN5R {
        EPIN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin6(&self) -> EPIN6R {
        EPIN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin7(&self) -> EPIN7R {
        EPIN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin8(&self) -> EPIN8R {
        EPIN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout0(&self) -> EPOUT0R {
        EPOUT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout1(&self) -> EPOUT1R {
        EPOUT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout2(&self) -> EPOUT2R {
        EPOUT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout3(&self) -> EPOUT3R {
        EPOUT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout4(&self) -> EPOUT4R {
        EPOUT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout5(&self) -> EPOUT5R {
        EPOUT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout6(&self) -> EPOUT6R {
        EPOUT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout7(&self) -> EPOUT7R {
        EPOUT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout8(&self) -> EPOUT8R {
        EPOUT8R::_from({
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
    #[doc = "Bit 0 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin0(&mut self) -> _EPIN0W {
        _EPIN0W { w: self }
    }
    #[doc = "Bit 1 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin1(&mut self) -> _EPIN1W {
        _EPIN1W { w: self }
    }
    #[doc = "Bit 2 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin2(&mut self) -> _EPIN2W {
        _EPIN2W { w: self }
    }
    #[doc = "Bit 3 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin3(&mut self) -> _EPIN3W {
        _EPIN3W { w: self }
    }
    #[doc = "Bit 4 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin4(&mut self) -> _EPIN4W {
        _EPIN4W { w: self }
    }
    #[doc = "Bit 5 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin5(&mut self) -> _EPIN5W {
        _EPIN5W { w: self }
    }
    #[doc = "Bit 6 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin6(&mut self) -> _EPIN6W {
        _EPIN6W { w: self }
    }
    #[doc = "Bit 7 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin7(&mut self) -> _EPIN7W {
        _EPIN7W { w: self }
    }
    #[doc = "Bit 8 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epin8(&mut self) -> _EPIN8W {
        _EPIN8W { w: self }
    }
    #[doc = "Bit 16 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout0(&mut self) -> _EPOUT0W {
        _EPOUT0W { w: self }
    }
    #[doc = "Bit 17 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout1(&mut self) -> _EPOUT1W {
        _EPOUT1W { w: self }
    }
    #[doc = "Bit 18 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout2(&mut self) -> _EPOUT2W {
        _EPOUT2W { w: self }
    }
    #[doc = "Bit 19 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout3(&mut self) -> _EPOUT3W {
        _EPOUT3W { w: self }
    }
    #[doc = "Bit 20 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout4(&mut self) -> _EPOUT4W {
        _EPOUT4W { w: self }
    }
    #[doc = "Bit 21 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout5(&mut self) -> _EPOUT5W {
        _EPOUT5W { w: self }
    }
    #[doc = "Bit 22 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout6(&mut self) -> _EPOUT6W {
        _EPOUT6W { w: self }
    }
    #[doc = "Bit 23 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout7(&mut self) -> _EPOUT7W {
        _EPOUT7W { w: self }
    }
    #[doc = "Bit 24 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline]
    pub fn epout8(&mut self) -> _EPOUT8W {
        _EPOUT8W { w: self }
    }
}
