#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RREN {
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
#[doc = "Possible values of the field `RR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR0R {
    #[doc = "Disable RR\\[0\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[0\\] register"]
    ENABLED,
}
impl RR0R {
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
            RR0R::DISABLED => false,
            RR0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR0R {
        match value {
            false => RR0R::DISABLED,
            true => RR0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RR0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RR0R::ENABLED
    }
}
#[doc = "Possible values of the field `RR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR1R {
    #[doc = "Disable RR\\[1\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[1\\] register"]
    ENABLED,
}
impl RR1R {
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
            RR1R::DISABLED => false,
            RR1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR1R {
        match value {
            false => RR1R::DISABLED,
            true => RR1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RR1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RR1R::ENABLED
    }
}
#[doc = "Possible values of the field `RR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR2R {
    #[doc = "Disable RR\\[2\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[2\\] register"]
    ENABLED,
}
impl RR2R {
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
            RR2R::DISABLED => false,
            RR2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR2R {
        match value {
            false => RR2R::DISABLED,
            true => RR2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RR2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RR2R::ENABLED
    }
}
#[doc = "Possible values of the field `RR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR3R {
    #[doc = "Disable RR\\[3\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[3\\] register"]
    ENABLED,
}
impl RR3R {
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
            RR3R::DISABLED => false,
            RR3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR3R {
        match value {
            false => RR3R::DISABLED,
            true => RR3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RR3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RR3R::ENABLED
    }
}
#[doc = "Possible values of the field `RR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR4R {
    #[doc = "Disable RR\\[4\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[4\\] register"]
    ENABLED,
}
impl RR4R {
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
            RR4R::DISABLED => false,
            RR4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR4R {
        match value {
            false => RR4R::DISABLED,
            true => RR4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RR4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RR4R::ENABLED
    }
}
#[doc = "Possible values of the field `RR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR5R {
    #[doc = "Disable RR\\[5\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[5\\] register"]
    ENABLED,
}
impl RR5R {
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
            RR5R::DISABLED => false,
            RR5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR5R {
        match value {
            false => RR5R::DISABLED,
            true => RR5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RR5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RR5R::ENABLED
    }
}
#[doc = "Possible values of the field `RR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR6R {
    #[doc = "Disable RR\\[6\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[6\\] register"]
    ENABLED,
}
impl RR6R {
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
            RR6R::DISABLED => false,
            RR6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR6R {
        match value {
            false => RR6R::DISABLED,
            true => RR6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RR6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RR6R::ENABLED
    }
}
#[doc = "Possible values of the field `RR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR7R {
    #[doc = "Disable RR\\[7\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[7\\] register"]
    ENABLED,
}
impl RR7R {
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
            RR7R::DISABLED => false,
            RR7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR7R {
        match value {
            false => RR7R::DISABLED,
            true => RR7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RR7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RR7R::ENABLED
    }
}
#[doc = "Values that can be written to the field `RR0`"]
pub enum RR0W {
    #[doc = "Disable RR\\[0\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[0\\] register"]
    ENABLED,
}
impl RR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RR0W::DISABLED => false,
            RR0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RR0W<'a> {
    w: &'a mut W,
}
impl<'a> _RR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RR\\[0\\] register"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR0W::DISABLED)
    }
    #[doc = "Enable RR\\[0\\] register"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR0W::ENABLED)
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
#[doc = "Values that can be written to the field `RR1`"]
pub enum RR1W {
    #[doc = "Disable RR\\[1\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[1\\] register"]
    ENABLED,
}
impl RR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RR1W::DISABLED => false,
            RR1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RR1W<'a> {
    w: &'a mut W,
}
impl<'a> _RR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RR\\[1\\] register"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR1W::DISABLED)
    }
    #[doc = "Enable RR\\[1\\] register"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR1W::ENABLED)
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
#[doc = "Values that can be written to the field `RR2`"]
pub enum RR2W {
    #[doc = "Disable RR\\[2\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[2\\] register"]
    ENABLED,
}
impl RR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RR2W::DISABLED => false,
            RR2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RR2W<'a> {
    w: &'a mut W,
}
impl<'a> _RR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RR\\[2\\] register"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR2W::DISABLED)
    }
    #[doc = "Enable RR\\[2\\] register"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR2W::ENABLED)
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
#[doc = "Values that can be written to the field `RR3`"]
pub enum RR3W {
    #[doc = "Disable RR\\[3\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[3\\] register"]
    ENABLED,
}
impl RR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RR3W::DISABLED => false,
            RR3W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RR3W<'a> {
    w: &'a mut W,
}
impl<'a> _RR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RR\\[3\\] register"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR3W::DISABLED)
    }
    #[doc = "Enable RR\\[3\\] register"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR3W::ENABLED)
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
#[doc = "Values that can be written to the field `RR4`"]
pub enum RR4W {
    #[doc = "Disable RR\\[4\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[4\\] register"]
    ENABLED,
}
impl RR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RR4W::DISABLED => false,
            RR4W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RR4W<'a> {
    w: &'a mut W,
}
impl<'a> _RR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RR\\[4\\] register"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR4W::DISABLED)
    }
    #[doc = "Enable RR\\[4\\] register"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR4W::ENABLED)
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
#[doc = "Values that can be written to the field `RR5`"]
pub enum RR5W {
    #[doc = "Disable RR\\[5\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[5\\] register"]
    ENABLED,
}
impl RR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RR5W::DISABLED => false,
            RR5W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RR5W<'a> {
    w: &'a mut W,
}
impl<'a> _RR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RR\\[5\\] register"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR5W::DISABLED)
    }
    #[doc = "Enable RR\\[5\\] register"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR5W::ENABLED)
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
#[doc = "Values that can be written to the field `RR6`"]
pub enum RR6W {
    #[doc = "Disable RR\\[6\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[6\\] register"]
    ENABLED,
}
impl RR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RR6W::DISABLED => false,
            RR6W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RR6W<'a> {
    w: &'a mut W,
}
impl<'a> _RR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RR\\[6\\] register"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR6W::DISABLED)
    }
    #[doc = "Enable RR\\[6\\] register"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR6W::ENABLED)
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
#[doc = "Values that can be written to the field `RR7`"]
pub enum RR7W {
    #[doc = "Disable RR\\[7\\] register"]
    DISABLED,
    #[doc = "Enable RR\\[7\\] register"]
    ENABLED,
}
impl RR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RR7W::DISABLED => false,
            RR7W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RR7W<'a> {
    w: &'a mut W,
}
impl<'a> _RR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RR7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RR\\[7\\] register"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR7W::DISABLED)
    }
    #[doc = "Enable RR\\[7\\] register"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR7W::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable or disable RR\\[0\\] register"]
    #[inline]
    pub fn rr0(&self) -> RR0R {
        RR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable or disable RR\\[1\\] register"]
    #[inline]
    pub fn rr1(&self) -> RR1R {
        RR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable or disable RR\\[2\\] register"]
    #[inline]
    pub fn rr2(&self) -> RR2R {
        RR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable or disable RR\\[3\\] register"]
    #[inline]
    pub fn rr3(&self) -> RR3R {
        RR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable or disable RR\\[4\\] register"]
    #[inline]
    pub fn rr4(&self) -> RR4R {
        RR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable or disable RR\\[5\\] register"]
    #[inline]
    pub fn rr5(&self) -> RR5R {
        RR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable or disable RR\\[6\\] register"]
    #[inline]
    pub fn rr6(&self) -> RR6R {
        RR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable or disable RR\\[7\\] register"]
    #[inline]
    pub fn rr7(&self) -> RR7R {
        RR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Enable or disable RR\\[0\\] register"]
    #[inline]
    pub fn rr0(&mut self) -> _RR0W {
        _RR0W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable RR\\[1\\] register"]
    #[inline]
    pub fn rr1(&mut self) -> _RR1W {
        _RR1W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable RR\\[2\\] register"]
    #[inline]
    pub fn rr2(&mut self) -> _RR2W {
        _RR2W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable RR\\[3\\] register"]
    #[inline]
    pub fn rr3(&mut self) -> _RR3W {
        _RR3W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable RR\\[4\\] register"]
    #[inline]
    pub fn rr4(&mut self) -> _RR4W {
        _RR4W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable RR\\[5\\] register"]
    #[inline]
    pub fn rr5(&mut self) -> _RR5W {
        _RR5W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable RR\\[6\\] register"]
    #[inline]
    pub fn rr6(&mut self) -> _RR6W {
        _RR6W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable RR\\[7\\] register"]
    #[inline]
    pub fn rr7(&mut self) -> _RR7W {
        _RR7W { w: self }
    }
}
