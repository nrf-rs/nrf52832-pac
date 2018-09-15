#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS {
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
#[doc = "Possible values of the field `COMPARE0_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_CLEARR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE0_CLEARR {
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
            COMPARE0_CLEARR::DISABLED => false,
            COMPARE0_CLEARR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE0_CLEARR {
        match value {
            false => COMPARE0_CLEARR::DISABLED,
            true => COMPARE0_CLEARR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_CLEARR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_CLEARR::ENABLED
    }
}
#[doc = "Possible values of the field `COMPARE1_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_CLEARR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE1_CLEARR {
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
            COMPARE1_CLEARR::DISABLED => false,
            COMPARE1_CLEARR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE1_CLEARR {
        match value {
            false => COMPARE1_CLEARR::DISABLED,
            true => COMPARE1_CLEARR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_CLEARR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_CLEARR::ENABLED
    }
}
#[doc = "Possible values of the field `COMPARE2_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_CLEARR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE2_CLEARR {
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
            COMPARE2_CLEARR::DISABLED => false,
            COMPARE2_CLEARR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE2_CLEARR {
        match value {
            false => COMPARE2_CLEARR::DISABLED,
            true => COMPARE2_CLEARR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_CLEARR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_CLEARR::ENABLED
    }
}
#[doc = "Possible values of the field `COMPARE3_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_CLEARR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE3_CLEARR {
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
            COMPARE3_CLEARR::DISABLED => false,
            COMPARE3_CLEARR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE3_CLEARR {
        match value {
            false => COMPARE3_CLEARR::DISABLED,
            true => COMPARE3_CLEARR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_CLEARR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_CLEARR::ENABLED
    }
}
#[doc = "Possible values of the field `COMPARE0_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE0_STOPR {
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
            COMPARE0_STOPR::DISABLED => false,
            COMPARE0_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE0_STOPR {
        match value {
            false => COMPARE0_STOPR::DISABLED,
            true => COMPARE0_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_STOPR::ENABLED
    }
}
#[doc = "Possible values of the field `COMPARE1_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE1_STOPR {
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
            COMPARE1_STOPR::DISABLED => false,
            COMPARE1_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE1_STOPR {
        match value {
            false => COMPARE1_STOPR::DISABLED,
            true => COMPARE1_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_STOPR::ENABLED
    }
}
#[doc = "Possible values of the field `COMPARE2_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE2_STOPR {
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
            COMPARE2_STOPR::DISABLED => false,
            COMPARE2_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE2_STOPR {
        match value {
            false => COMPARE2_STOPR::DISABLED,
            true => COMPARE2_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_STOPR::ENABLED
    }
}
#[doc = "Possible values of the field `COMPARE3_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE3_STOPR {
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
            COMPARE3_STOPR::DISABLED => false,
            COMPARE3_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE3_STOPR {
        match value {
            false => COMPARE3_STOPR::DISABLED,
            true => COMPARE3_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_STOPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `COMPARE0_CLEAR`"]
pub enum COMPARE0_CLEARW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE0_CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE0_CLEARW::DISABLED => false,
            COMPARE0_CLEARW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE0_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE0_CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE0_CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEARW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEARW::ENABLED)
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
#[doc = "Values that can be written to the field `COMPARE1_CLEAR`"]
pub enum COMPARE1_CLEARW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE1_CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE1_CLEARW::DISABLED => false,
            COMPARE1_CLEARW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE1_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE1_CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE1_CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEARW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEARW::ENABLED)
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
#[doc = "Values that can be written to the field `COMPARE2_CLEAR`"]
pub enum COMPARE2_CLEARW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE2_CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE2_CLEARW::DISABLED => false,
            COMPARE2_CLEARW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE2_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE2_CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE2_CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEARW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEARW::ENABLED)
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
#[doc = "Values that can be written to the field `COMPARE3_CLEAR`"]
pub enum COMPARE3_CLEARW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE3_CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE3_CLEARW::DISABLED => false,
            COMPARE3_CLEARW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE3_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE3_CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE3_CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEARW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEARW::ENABLED)
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
#[doc = "Values that can be written to the field `COMPARE0_STOP`"]
pub enum COMPARE0_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE0_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE0_STOPW::DISABLED => false,
            COMPARE0_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE0_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE0_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE0_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOPW::ENABLED)
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
#[doc = "Values that can be written to the field `COMPARE1_STOP`"]
pub enum COMPARE1_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE1_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE1_STOPW::DISABLED => false,
            COMPARE1_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE1_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE1_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE1_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOPW::ENABLED)
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
#[doc = "Values that can be written to the field `COMPARE2_STOP`"]
pub enum COMPARE2_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE2_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE2_STOPW::DISABLED => false,
            COMPARE2_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE2_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE2_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE2_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOPW::ENABLED)
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
#[doc = "Values that can be written to the field `COMPARE3_STOP`"]
pub enum COMPARE3_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl COMPARE3_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE3_STOPW::DISABLED => false,
            COMPARE3_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE3_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE3_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE3_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOPW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Shortcut between CC[0] event and the CLEAR task."]
    #[inline]
    pub fn compare0_clear(&self) -> COMPARE0_CLEARR {
        COMPARE0_CLEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Shortcut between CC[1] event and the CLEAR task."]
    #[inline]
    pub fn compare1_clear(&self) -> COMPARE1_CLEARR {
        COMPARE1_CLEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Shortcut between CC[2] event and the CLEAR task."]
    #[inline]
    pub fn compare2_clear(&self) -> COMPARE2_CLEARR {
        COMPARE2_CLEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Shortcut between CC[3] event and the CLEAR task."]
    #[inline]
    pub fn compare3_clear(&self) -> COMPARE3_CLEARR {
        COMPARE3_CLEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Shortcut between CC[0] event and the STOP task."]
    #[inline]
    pub fn compare0_stop(&self) -> COMPARE0_STOPR {
        COMPARE0_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Shortcut between CC[1] event and the STOP task."]
    #[inline]
    pub fn compare1_stop(&self) -> COMPARE1_STOPR {
        COMPARE1_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Shortcut between CC[2] event and the STOP task."]
    #[inline]
    pub fn compare2_stop(&self) -> COMPARE2_STOPR {
        COMPARE2_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Shortcut between CC[3] event and the STOP task."]
    #[inline]
    pub fn compare3_stop(&self) -> COMPARE3_STOPR {
        COMPARE3_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Shortcut between CC[0] event and the CLEAR task."]
    #[inline]
    pub fn compare0_clear(&mut self) -> _COMPARE0_CLEARW {
        _COMPARE0_CLEARW { w: self }
    }
    #[doc = "Bit 1 - Shortcut between CC[1] event and the CLEAR task."]
    #[inline]
    pub fn compare1_clear(&mut self) -> _COMPARE1_CLEARW {
        _COMPARE1_CLEARW { w: self }
    }
    #[doc = "Bit 2 - Shortcut between CC[2] event and the CLEAR task."]
    #[inline]
    pub fn compare2_clear(&mut self) -> _COMPARE2_CLEARW {
        _COMPARE2_CLEARW { w: self }
    }
    #[doc = "Bit 3 - Shortcut between CC[3] event and the CLEAR task."]
    #[inline]
    pub fn compare3_clear(&mut self) -> _COMPARE3_CLEARW {
        _COMPARE3_CLEARW { w: self }
    }
    #[doc = "Bit 8 - Shortcut between CC[0] event and the STOP task."]
    #[inline]
    pub fn compare0_stop(&mut self) -> _COMPARE0_STOPW {
        _COMPARE0_STOPW { w: self }
    }
    #[doc = "Bit 9 - Shortcut between CC[1] event and the STOP task."]
    #[inline]
    pub fn compare1_stop(&mut self) -> _COMPARE1_STOPW {
        _COMPARE1_STOPW { w: self }
    }
    #[doc = "Bit 10 - Shortcut between CC[2] event and the STOP task."]
    #[inline]
    pub fn compare2_stop(&mut self) -> _COMPARE2_STOPW {
        _COMPARE2_STOPW { w: self }
    }
    #[doc = "Bit 11 - Shortcut between CC[3] event and the STOP task."]
    #[inline]
    pub fn compare3_stop(&mut self) -> _COMPARE3_STOPW {
        _COMPARE3_STOPW { w: self }
    }
}
