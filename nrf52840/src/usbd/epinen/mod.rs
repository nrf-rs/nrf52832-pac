#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPINEN {
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
#[doc = "Possible values of the field `IN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0R {
    #[doc = "Disable endpoint IN 0 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 0 (response to IN tokens)"]
    ENABLE,
}
impl IN0R {
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
            IN0R::DISABLE => false,
            IN0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN0R {
        match value {
            false => IN0R::DISABLE,
            true => IN0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IN0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IN0R::ENABLE
    }
}
#[doc = "Possible values of the field `IN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1R {
    #[doc = "Disable endpoint IN 1 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 1 (response to IN tokens)"]
    ENABLE,
}
impl IN1R {
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
            IN1R::DISABLE => false,
            IN1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN1R {
        match value {
            false => IN1R::DISABLE,
            true => IN1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IN1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IN1R::ENABLE
    }
}
#[doc = "Possible values of the field `IN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2R {
    #[doc = "Disable endpoint IN 2 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 2 (response to IN tokens)"]
    ENABLE,
}
impl IN2R {
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
            IN2R::DISABLE => false,
            IN2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN2R {
        match value {
            false => IN2R::DISABLE,
            true => IN2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IN2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IN2R::ENABLE
    }
}
#[doc = "Possible values of the field `IN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3R {
    #[doc = "Disable endpoint IN 3 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 3 (response to IN tokens)"]
    ENABLE,
}
impl IN3R {
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
            IN3R::DISABLE => false,
            IN3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN3R {
        match value {
            false => IN3R::DISABLE,
            true => IN3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IN3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IN3R::ENABLE
    }
}
#[doc = "Possible values of the field `IN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4R {
    #[doc = "Disable endpoint IN 4 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 4 (response to IN tokens)"]
    ENABLE,
}
impl IN4R {
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
            IN4R::DISABLE => false,
            IN4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN4R {
        match value {
            false => IN4R::DISABLE,
            true => IN4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IN4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IN4R::ENABLE
    }
}
#[doc = "Possible values of the field `IN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5R {
    #[doc = "Disable endpoint IN 5 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 5 (response to IN tokens)"]
    ENABLE,
}
impl IN5R {
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
            IN5R::DISABLE => false,
            IN5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN5R {
        match value {
            false => IN5R::DISABLE,
            true => IN5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IN5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IN5R::ENABLE
    }
}
#[doc = "Possible values of the field `IN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6R {
    #[doc = "Disable endpoint IN 6 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 6 (response to IN tokens)"]
    ENABLE,
}
impl IN6R {
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
            IN6R::DISABLE => false,
            IN6R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN6R {
        match value {
            false => IN6R::DISABLE,
            true => IN6R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IN6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IN6R::ENABLE
    }
}
#[doc = "Possible values of the field `IN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7R {
    #[doc = "Disable endpoint IN 7 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 7 (response to IN tokens)"]
    ENABLE,
}
impl IN7R {
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
            IN7R::DISABLE => false,
            IN7R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IN7R {
        match value {
            false => IN7R::DISABLE,
            true => IN7R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IN7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IN7R::ENABLE
    }
}
#[doc = "Possible values of the field `ISOIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOINR {
    #[doc = "Disable ISO IN endpoint 8"]
    DISABLE,
    #[doc = "Enable ISO IN endpoint 8"]
    ENABLE,
}
impl ISOINR {
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
            ISOINR::DISABLE => false,
            ISOINR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISOINR {
        match value {
            false => ISOINR::DISABLE,
            true => ISOINR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ISOINR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ISOINR::ENABLE
    }
}
#[doc = "Values that can be written to the field `IN0`"]
pub enum IN0W {
    #[doc = "Disable endpoint IN 0 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 0 (response to IN tokens)"]
    ENABLE,
}
impl IN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN0W::DISABLE => false,
            IN0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN0W<'a> {
    w: &'a mut W,
}
impl<'a> _IN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint IN 0 (no response to IN tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN0W::DISABLE)
    }
    #[doc = "Enable endpoint IN 0 (response to IN tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN0W::ENABLE)
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
#[doc = "Values that can be written to the field `IN1`"]
pub enum IN1W {
    #[doc = "Disable endpoint IN 1 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 1 (response to IN tokens)"]
    ENABLE,
}
impl IN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN1W::DISABLE => false,
            IN1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN1W<'a> {
    w: &'a mut W,
}
impl<'a> _IN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint IN 1 (no response to IN tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN1W::DISABLE)
    }
    #[doc = "Enable endpoint IN 1 (response to IN tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN1W::ENABLE)
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
#[doc = "Values that can be written to the field `IN2`"]
pub enum IN2W {
    #[doc = "Disable endpoint IN 2 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 2 (response to IN tokens)"]
    ENABLE,
}
impl IN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN2W::DISABLE => false,
            IN2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN2W<'a> {
    w: &'a mut W,
}
impl<'a> _IN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint IN 2 (no response to IN tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN2W::DISABLE)
    }
    #[doc = "Enable endpoint IN 2 (response to IN tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN2W::ENABLE)
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
#[doc = "Values that can be written to the field `IN3`"]
pub enum IN3W {
    #[doc = "Disable endpoint IN 3 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 3 (response to IN tokens)"]
    ENABLE,
}
impl IN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN3W::DISABLE => false,
            IN3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN3W<'a> {
    w: &'a mut W,
}
impl<'a> _IN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint IN 3 (no response to IN tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN3W::DISABLE)
    }
    #[doc = "Enable endpoint IN 3 (response to IN tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN3W::ENABLE)
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
#[doc = "Values that can be written to the field `IN4`"]
pub enum IN4W {
    #[doc = "Disable endpoint IN 4 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 4 (response to IN tokens)"]
    ENABLE,
}
impl IN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN4W::DISABLE => false,
            IN4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN4W<'a> {
    w: &'a mut W,
}
impl<'a> _IN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint IN 4 (no response to IN tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN4W::DISABLE)
    }
    #[doc = "Enable endpoint IN 4 (response to IN tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN4W::ENABLE)
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
#[doc = "Values that can be written to the field `IN5`"]
pub enum IN5W {
    #[doc = "Disable endpoint IN 5 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 5 (response to IN tokens)"]
    ENABLE,
}
impl IN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN5W::DISABLE => false,
            IN5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN5W<'a> {
    w: &'a mut W,
}
impl<'a> _IN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint IN 5 (no response to IN tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN5W::DISABLE)
    }
    #[doc = "Enable endpoint IN 5 (response to IN tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN5W::ENABLE)
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
#[doc = "Values that can be written to the field `IN6`"]
pub enum IN6W {
    #[doc = "Disable endpoint IN 6 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 6 (response to IN tokens)"]
    ENABLE,
}
impl IN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN6W::DISABLE => false,
            IN6W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN6W<'a> {
    w: &'a mut W,
}
impl<'a> _IN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint IN 6 (no response to IN tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN6W::DISABLE)
    }
    #[doc = "Enable endpoint IN 6 (response to IN tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN6W::ENABLE)
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
#[doc = "Values that can be written to the field `IN7`"]
pub enum IN7W {
    #[doc = "Disable endpoint IN 7 (no response to IN tokens)"]
    DISABLE,
    #[doc = "Enable endpoint IN 7 (response to IN tokens)"]
    ENABLE,
}
impl IN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IN7W::DISABLE => false,
            IN7W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IN7W<'a> {
    w: &'a mut W,
}
impl<'a> _IN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable endpoint IN 7 (no response to IN tokens)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN7W::DISABLE)
    }
    #[doc = "Enable endpoint IN 7 (response to IN tokens)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN7W::ENABLE)
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
#[doc = "Values that can be written to the field `ISOIN`"]
pub enum ISOINW {
    #[doc = "Disable ISO IN endpoint 8"]
    DISABLE,
    #[doc = "Enable ISO IN endpoint 8"]
    ENABLE,
}
impl ISOINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISOINW::DISABLE => false,
            ISOINW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISOINW<'a> {
    w: &'a mut W,
}
impl<'a> _ISOINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISOINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable ISO IN endpoint 8"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISOINW::DISABLE)
    }
    #[doc = "Enable ISO IN endpoint 8"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISOINW::ENABLE)
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
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline]
    pub fn in0(&self) -> IN0R {
        IN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline]
    pub fn in1(&self) -> IN1R {
        IN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline]
    pub fn in2(&self) -> IN2R {
        IN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline]
    pub fn in3(&self) -> IN3R {
        IN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline]
    pub fn in4(&self) -> IN4R {
        IN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline]
    pub fn in5(&self) -> IN5R {
        IN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline]
    pub fn in6(&self) -> IN6R {
        IN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline]
    pub fn in7(&self) -> IN7R {
        IN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline]
    pub fn isoin(&self) -> ISOINR {
        ISOINR::_from({
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
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline]
    pub fn in0(&mut self) -> _IN0W {
        _IN0W { w: self }
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline]
    pub fn in1(&mut self) -> _IN1W {
        _IN1W { w: self }
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline]
    pub fn in2(&mut self) -> _IN2W {
        _IN2W { w: self }
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline]
    pub fn in3(&mut self) -> _IN3W {
        _IN3W { w: self }
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline]
    pub fn in4(&mut self) -> _IN4W {
        _IN4W { w: self }
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline]
    pub fn in5(&mut self) -> _IN5W {
        _IN5W { w: self }
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline]
    pub fn in6(&mut self) -> _IN6W {
        _IN6W { w: self }
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline]
    pub fn in7(&mut self) -> _IN7W {
        _IN7W { w: self }
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline]
    pub fn isoin(&mut self) -> _ISOINW {
        _ISOINW { w: self }
    }
}
