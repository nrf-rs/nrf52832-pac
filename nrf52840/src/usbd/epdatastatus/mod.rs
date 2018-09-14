#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPDATASTATUS {
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
#[doc = "Possible values of the field `EPIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN1R {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
            EPIN1R::NOTDONE => false,
            EPIN1R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN1R {
        match value {
            false => EPIN1R::NOTDONE,
            true => EPIN1R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN1R::NOTDONE
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
            EPIN2R::NOTDONE => false,
            EPIN2R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN2R {
        match value {
            false => EPIN2R::NOTDONE,
            true => EPIN2R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN2R::NOTDONE
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
            EPIN3R::NOTDONE => false,
            EPIN3R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN3R {
        match value {
            false => EPIN3R::NOTDONE,
            true => EPIN3R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN3R::NOTDONE
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
            EPIN4R::NOTDONE => false,
            EPIN4R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN4R {
        match value {
            false => EPIN4R::NOTDONE,
            true => EPIN4R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN4R::NOTDONE
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
            EPIN5R::NOTDONE => false,
            EPIN5R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN5R {
        match value {
            false => EPIN5R::NOTDONE,
            true => EPIN5R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN5R::NOTDONE
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
            EPIN6R::NOTDONE => false,
            EPIN6R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN6R {
        match value {
            false => EPIN6R::NOTDONE,
            true => EPIN6R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN6R::NOTDONE
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
            EPIN7R::NOTDONE => false,
            EPIN7R::DATADONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIN7R {
        match value {
            false => EPIN7R::NOTDONE,
            true => EPIN7R::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN7R::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN7R::DATADONE
    }
}
#[doc = "Possible values of the field `EPOUT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT1R {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
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
            EPOUT1R::NOTSTARTED => false,
            EPOUT1R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT1R {
        match value {
            false => EPOUT1R::NOTSTARTED,
            true => EPOUT1R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT1R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == EPOUT1R::STARTED
    }
}
#[doc = "Possible values of the field `EPOUT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT2R {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
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
            EPOUT2R::NOTSTARTED => false,
            EPOUT2R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT2R {
        match value {
            false => EPOUT2R::NOTSTARTED,
            true => EPOUT2R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT2R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == EPOUT2R::STARTED
    }
}
#[doc = "Possible values of the field `EPOUT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT3R {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
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
            EPOUT3R::NOTSTARTED => false,
            EPOUT3R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT3R {
        match value {
            false => EPOUT3R::NOTSTARTED,
            true => EPOUT3R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT3R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == EPOUT3R::STARTED
    }
}
#[doc = "Possible values of the field `EPOUT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT4R {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
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
            EPOUT4R::NOTSTARTED => false,
            EPOUT4R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT4R {
        match value {
            false => EPOUT4R::NOTSTARTED,
            true => EPOUT4R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT4R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == EPOUT4R::STARTED
    }
}
#[doc = "Possible values of the field `EPOUT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT5R {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
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
            EPOUT5R::NOTSTARTED => false,
            EPOUT5R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT5R {
        match value {
            false => EPOUT5R::NOTSTARTED,
            true => EPOUT5R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT5R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == EPOUT5R::STARTED
    }
}
#[doc = "Possible values of the field `EPOUT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT6R {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
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
            EPOUT6R::NOTSTARTED => false,
            EPOUT6R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT6R {
        match value {
            false => EPOUT6R::NOTSTARTED,
            true => EPOUT6R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT6R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == EPOUT6R::STARTED
    }
}
#[doc = "Possible values of the field `EPOUT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT7R {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
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
            EPOUT7R::NOTSTARTED => false,
            EPOUT7R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPOUT7R {
        match value {
            false => EPOUT7R::NOTSTARTED,
            true => EPOUT7R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT7R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == EPOUT7R::STARTED
    }
}
#[doc = "Values that can be written to the field `EPIN1`"]
pub enum EPIN1W {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    DATADONE,
}
impl EPIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN1W::NOTDONE => false,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN1W::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    DATADONE,
}
impl EPIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN2W::NOTDONE => false,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN2W::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    DATADONE,
}
impl EPIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN3W::NOTDONE => false,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN3W::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    DATADONE,
}
impl EPIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN4W::NOTDONE => false,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN4W::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    DATADONE,
}
impl EPIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN5W::NOTDONE => false,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN5W::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    DATADONE,
}
impl EPIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN6W::NOTDONE => false,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN6W::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTDONE,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    DATADONE,
}
impl EPIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPIN7W::NOTDONE => false,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN7W::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
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
#[doc = "Values that can be written to the field `EPOUT1`"]
pub enum EPOUT1W {
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
}
impl EPOUT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT1W::NOTSTARTED => false,
            EPOUT1W::STARTED => true,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT1W::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT1W::STARTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
}
impl EPOUT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT2W::NOTSTARTED => false,
            EPOUT2W::STARTED => true,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT2W::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT2W::STARTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
}
impl EPOUT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT3W::NOTSTARTED => false,
            EPOUT3W::STARTED => true,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT3W::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT3W::STARTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
}
impl EPOUT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT4W::NOTSTARTED => false,
            EPOUT4W::STARTED => true,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT4W::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT4W::STARTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
}
impl EPOUT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT5W::NOTSTARTED => false,
            EPOUT5W::STARTED => true,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT5W::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT5W::STARTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
}
impl EPOUT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT6W::NOTSTARTED => false,
            EPOUT6W::STARTED => true,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT6W::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT6W::STARTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    NOTSTARTED,
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    STARTED,
}
impl EPOUT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPOUT7W::NOTSTARTED => false,
            EPOUT7W::STARTED => true,
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
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT7W::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT7W::STARTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin1(&self) -> EPIN1R {
        EPIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin2(&self) -> EPIN2R {
        EPIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin3(&self) -> EPIN3R {
        EPIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin4(&self) -> EPIN4R {
        EPIN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin5(&self) -> EPIN5R {
        EPIN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin6(&self) -> EPIN6R {
        EPIN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin7(&self) -> EPIN7R {
        EPIN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout1(&self) -> EPOUT1R {
        EPOUT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout2(&self) -> EPOUT2R {
        EPOUT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout3(&self) -> EPOUT3R {
        EPOUT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout4(&self) -> EPOUT4R {
        EPOUT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout5(&self) -> EPOUT5R {
        EPOUT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout6(&self) -> EPOUT6R {
        EPOUT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout7(&self) -> EPOUT7R {
        EPOUT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 1 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin1(&mut self) -> _EPIN1W {
        _EPIN1W { w: self }
    }
    #[doc = "Bit 2 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin2(&mut self) -> _EPIN2W {
        _EPIN2W { w: self }
    }
    #[doc = "Bit 3 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin3(&mut self) -> _EPIN3W {
        _EPIN3W { w: self }
    }
    #[doc = "Bit 4 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin4(&mut self) -> _EPIN4W {
        _EPIN4W { w: self }
    }
    #[doc = "Bit 5 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin5(&mut self) -> _EPIN5W {
        _EPIN5W { w: self }
    }
    #[doc = "Bit 6 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin6(&mut self) -> _EPIN6W {
        _EPIN6W { w: self }
    }
    #[doc = "Bit 7 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline]
    pub fn epin7(&mut self) -> _EPIN7W {
        _EPIN7W { w: self }
    }
    #[doc = "Bit 17 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout1(&mut self) -> _EPOUT1W {
        _EPOUT1W { w: self }
    }
    #[doc = "Bit 18 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout2(&mut self) -> _EPOUT2W {
        _EPOUT2W { w: self }
    }
    #[doc = "Bit 19 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout3(&mut self) -> _EPOUT3W {
        _EPOUT3W { w: self }
    }
    #[doc = "Bit 20 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout4(&mut self) -> _EPOUT4W {
        _EPOUT4W { w: self }
    }
    #[doc = "Bit 21 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout5(&mut self) -> _EPOUT5W {
        _EPOUT5W { w: self }
    }
    #[doc = "Bit 22 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout6(&mut self) -> _EPOUT6W {
        _EPOUT6W { w: self }
    }
    #[doc = "Bit 23 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline]
    pub fn epout7(&mut self) -> _EPOUT7W {
        _EPOUT7W { w: self }
    }
}
