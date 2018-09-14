#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPOUTEN {
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
#[doc = "Possible values of the field `OUT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT0R {
    #[doc = "Disable endpoint OUT 0 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 0 (response to OUT tokens)"]
    ENABLE,
}
impl OUT0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OUT0R::DISABLE => false,
            OUT0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUT0R {
        match value {
            false => OUT0R::DISABLE,
            true => OUT0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUT0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OUT0R::ENABLE
    }
}
#[doc = "Possible values of the field `OUT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT1R {
    #[doc = "Disable endpoint OUT 1 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 1 (response to OUT tokens)"]
    ENABLE,
}
impl OUT1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OUT1R::DISABLE => false,
            OUT1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUT1R {
        match value {
            false => OUT1R::DISABLE,
            true => OUT1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUT1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OUT1R::ENABLE
    }
}
#[doc = "Possible values of the field `OUT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT2R {
    #[doc = "Disable endpoint OUT 2 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 2 (response to OUT tokens)"]
    ENABLE,
}
impl OUT2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OUT2R::DISABLE => false,
            OUT2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUT2R {
        match value {
            false => OUT2R::DISABLE,
            true => OUT2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUT2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OUT2R::ENABLE
    }
}
#[doc = "Possible values of the field `OUT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT3R {
    #[doc = "Disable endpoint OUT 3 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 3 (response to OUT tokens)"]
    ENABLE,
}
impl OUT3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OUT3R::DISABLE => false,
            OUT3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUT3R {
        match value {
            false => OUT3R::DISABLE,
            true => OUT3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUT3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OUT3R::ENABLE
    }
}
#[doc = "Possible values of the field `OUT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT4R {
    #[doc = "Disable endpoint OUT 4 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 4 (response to OUT tokens)"]
    ENABLE,
}
impl OUT4R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OUT4R::DISABLE => false,
            OUT4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUT4R {
        match value {
            false => OUT4R::DISABLE,
            true => OUT4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUT4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OUT4R::ENABLE
    }
}
#[doc = "Possible values of the field `OUT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT5R {
    #[doc = "Disable endpoint OUT 5 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 5 (response to OUT tokens)"]
    ENABLE,
}
impl OUT5R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OUT5R::DISABLE => false,
            OUT5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUT5R {
        match value {
            false => OUT5R::DISABLE,
            true => OUT5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUT5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OUT5R::ENABLE
    }
}
#[doc = "Possible values of the field `OUT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT6R {
    #[doc = "Disable endpoint OUT 6 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 6 (response to OUT tokens)"]
    ENABLE,
}
impl OUT6R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OUT6R::DISABLE => false,
            OUT6R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUT6R {
        match value {
            false => OUT6R::DISABLE,
            true => OUT6R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUT6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OUT6R::ENABLE
    }
}
#[doc = "Possible values of the field `OUT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT7R {
    #[doc = "Disable endpoint OUT 7 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 7 (response to OUT tokens)"]
    ENABLE,
}
impl OUT7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OUT7R::DISABLE => false,
            OUT7R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUT7R {
        match value {
            false => OUT7R::DISABLE,
            true => OUT7R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUT7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OUT7R::ENABLE
    }
}
#[doc = "Possible values of the field `ISOOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOOUTR {
    #[doc = "Disable ISO OUT endpoint 8"]
    DISABLE,
    #[doc = "Enable ISO OUT endpoint 8"]
    ENABLE,
}
impl ISOOUTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ISOOUTR::DISABLE => false,
            ISOOUTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISOOUTR {
        match value {
            false => ISOOUTR::DISABLE,
            true => ISOOUTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ISOOUTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ISOOUTR::ENABLE
    }
}
#[doc = "Values that can be written to the field `OUT0`"]
pub enum OUT0W {
    #[doc = "Disable endpoint OUT 0 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 0 (response to OUT tokens)"]
    ENABLE,
}
impl OUT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUT0W::DISABLE => false,
            OUT0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT0W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint OUT 0 (no response to OUT tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT0W::DISABLE)
    }
    #[doc = "Enable endpoint OUT 0 (response to OUT tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT0W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `OUT1`"]
pub enum OUT1W {
    #[doc = "Disable endpoint OUT 1 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 1 (response to OUT tokens)"]
    ENABLE,
}
impl OUT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUT1W::DISABLE => false,
            OUT1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT1W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint OUT 1 (no response to OUT tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT1W::DISABLE)
    }
    #[doc = "Enable endpoint OUT 1 (response to OUT tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT1W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `OUT2`"]
pub enum OUT2W {
    #[doc = "Disable endpoint OUT 2 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 2 (response to OUT tokens)"]
    ENABLE,
}
impl OUT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUT2W::DISABLE => false,
            OUT2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT2W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint OUT 2 (no response to OUT tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT2W::DISABLE)
    }
    #[doc = "Enable endpoint OUT 2 (response to OUT tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT2W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `OUT3`"]
pub enum OUT3W {
    #[doc = "Disable endpoint OUT 3 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 3 (response to OUT tokens)"]
    ENABLE,
}
impl OUT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUT3W::DISABLE => false,
            OUT3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT3W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint OUT 3 (no response to OUT tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT3W::DISABLE)
    }
    #[doc = "Enable endpoint OUT 3 (response to OUT tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT3W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `OUT4`"]
pub enum OUT4W {
    #[doc = "Disable endpoint OUT 4 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 4 (response to OUT tokens)"]
    ENABLE,
}
impl OUT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUT4W::DISABLE => false,
            OUT4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT4W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint OUT 4 (no response to OUT tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT4W::DISABLE)
    }
    #[doc = "Enable endpoint OUT 4 (response to OUT tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT4W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `OUT5`"]
pub enum OUT5W {
    #[doc = "Disable endpoint OUT 5 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 5 (response to OUT tokens)"]
    ENABLE,
}
impl OUT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUT5W::DISABLE => false,
            OUT5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT5W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint OUT 5 (no response to OUT tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT5W::DISABLE)
    }
    #[doc = "Enable endpoint OUT 5 (response to OUT tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT5W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `OUT6`"]
pub enum OUT6W {
    #[doc = "Disable endpoint OUT 6 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 6 (response to OUT tokens)"]
    ENABLE,
}
impl OUT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUT6W::DISABLE => false,
            OUT6W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT6W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint OUT 6 (no response to OUT tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT6W::DISABLE)
    }
    #[doc = "Enable endpoint OUT 6 (response to OUT tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT6W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `OUT7`"]
pub enum OUT7W {
    #[doc = "Disable endpoint OUT 7 (no response to OUT tokens)"]
    DISABLE,
    #[doc = "Enable endpoint OUT 7 (response to OUT tokens)"]
    ENABLE,
}
impl OUT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUT7W::DISABLE => false,
            OUT7W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUT7W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint OUT 7 (no response to OUT tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUT7W::DISABLE)
    }
    #[doc = "Enable endpoint OUT 7 (response to OUT tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUT7W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `ISOOUT`"]
pub enum ISOOUTW {
    #[doc = "Disable ISO OUT endpoint 8"]
    DISABLE,
    #[doc = "Enable ISO OUT endpoint 8"]
    ENABLE,
}
impl ISOOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISOOUTW::DISABLE => false,
            ISOOUTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISOOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _ISOOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISOOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable ISO OUT endpoint 8"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISOOUTW::DISABLE)
    }
    #[doc = "Enable ISO OUT endpoint 8"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISOOUTW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable OUT endpoint 0"]
    #[inline]
    pub fn out0(&self) -> OUT0R {
        OUT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable OUT endpoint 1"]
    #[inline]
    pub fn out1(&self) -> OUT1R {
        OUT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable OUT endpoint 2"]
    #[inline]
    pub fn out2(&self) -> OUT2R {
        OUT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable OUT endpoint 3"]
    #[inline]
    pub fn out3(&self) -> OUT3R {
        OUT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable OUT endpoint 4"]
    #[inline]
    pub fn out4(&self) -> OUT4R {
        OUT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable OUT endpoint 5"]
    #[inline]
    pub fn out5(&self) -> OUT5R {
        OUT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable OUT endpoint 6"]
    #[inline]
    pub fn out6(&self) -> OUT6R {
        OUT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable OUT endpoint 7"]
    #[inline]
    pub fn out7(&self) -> OUT7R {
        OUT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable ISO OUT endpoint 8"]
    #[inline]
    pub fn isoout(&self) -> ISOOUTR {
        ISOOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable OUT endpoint 0"]
    #[inline]
    pub fn out0(&mut self) -> _OUT0W {
        _OUT0W { w: self }
    }
    #[doc = "Bit 1 - Enable OUT endpoint 1"]
    #[inline]
    pub fn out1(&mut self) -> _OUT1W {
        _OUT1W { w: self }
    }
    #[doc = "Bit 2 - Enable OUT endpoint 2"]
    #[inline]
    pub fn out2(&mut self) -> _OUT2W {
        _OUT2W { w: self }
    }
    #[doc = "Bit 3 - Enable OUT endpoint 3"]
    #[inline]
    pub fn out3(&mut self) -> _OUT3W {
        _OUT3W { w: self }
    }
    #[doc = "Bit 4 - Enable OUT endpoint 4"]
    #[inline]
    pub fn out4(&mut self) -> _OUT4W {
        _OUT4W { w: self }
    }
    #[doc = "Bit 5 - Enable OUT endpoint 5"]
    #[inline]
    pub fn out5(&mut self) -> _OUT5W {
        _OUT5W { w: self }
    }
    #[doc = "Bit 6 - Enable OUT endpoint 6"]
    #[inline]
    pub fn out6(&mut self) -> _OUT6W {
        _OUT6W { w: self }
    }
    #[doc = "Bit 7 - Enable OUT endpoint 7"]
    #[inline]
    pub fn out7(&mut self) -> _OUT7W {
        _OUT7W { w: self }
    }
    #[doc = "Bit 8 - Enable ISO OUT endpoint 8"]
    #[inline]
    pub fn isoout(&mut self) -> _ISOOUTW {
        _ISOOUTW { w: self }
    }
}
