#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENCLR {
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
#[doc = "Possible values of the field `READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READYR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl READYR {
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
            READYR::DISABLED => false,
            READYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READYR {
        match value {
            false => READYR::DISABLED,
            true => READYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == READYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == READYR::ENABLED
    }
}
#[doc = "Possible values of the field `FIELDDETECTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDDETECTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl FIELDDETECTEDR {
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
            FIELDDETECTEDR::DISABLED => false,
            FIELDDETECTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIELDDETECTEDR {
        match value {
            false => FIELDDETECTEDR::DISABLED,
            true => FIELDDETECTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDDETECTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDDETECTEDR::ENABLED
    }
}
#[doc = "Possible values of the field `FIELDLOST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDLOSTR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl FIELDLOSTR {
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
            FIELDLOSTR::DISABLED => false,
            FIELDLOSTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIELDLOSTR {
        match value {
            false => FIELDLOSTR::DISABLED,
            true => FIELDLOSTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDLOSTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDLOSTR::ENABLED
    }
}
#[doc = "Possible values of the field `TXFRAMESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMESTARTR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TXFRAMESTARTR {
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
            TXFRAMESTARTR::DISABLED => false,
            TXFRAMESTARTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFRAMESTARTR {
        match value {
            false => TXFRAMESTARTR::DISABLED,
            true => TXFRAMESTARTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXFRAMESTARTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMESTARTR::ENABLED
    }
}
#[doc = "Possible values of the field `TXFRAMEEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMEENDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TXFRAMEENDR {
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
            TXFRAMEENDR::DISABLED => false,
            TXFRAMEENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFRAMEENDR {
        match value {
            false => TXFRAMEENDR::DISABLED,
            true => TXFRAMEENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXFRAMEENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMEENDR::ENABLED
    }
}
#[doc = "Possible values of the field `RXFRAMESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRAMESTARTR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RXFRAMESTARTR {
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
            RXFRAMESTARTR::DISABLED => false,
            RXFRAMESTARTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFRAMESTARTR {
        match value {
            false => RXFRAMESTARTR::DISABLED,
            true => RXFRAMESTARTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXFRAMESTARTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXFRAMESTARTR::ENABLED
    }
}
#[doc = "Possible values of the field `RXFRAMEEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRAMEENDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RXFRAMEENDR {
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
            RXFRAMEENDR::DISABLED => false,
            RXFRAMEENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFRAMEENDR {
        match value {
            false => RXFRAMEENDR::DISABLED,
            true => RXFRAMEENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXFRAMEENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXFRAMEENDR::ENABLED
    }
}
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ERRORR {
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
            ERRORR::DISABLED => false,
            ERRORR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRORR {
        match value {
            false => ERRORR::DISABLED,
            true => ERRORR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ERRORR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ERRORR::ENABLED
    }
}
#[doc = "Possible values of the field `RXERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERRORR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RXERRORR {
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
            RXERRORR::DISABLED => false,
            RXERRORR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXERRORR {
        match value {
            false => RXERRORR::DISABLED,
            true => RXERRORR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXERRORR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXERRORR::ENABLED
    }
}
#[doc = "Possible values of the field `ENDRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRXR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDRXR {
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
            ENDRXR::DISABLED => false,
            ENDRXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDRXR {
        match value {
            false => ENDRXR::DISABLED,
            true => ENDRXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRXR::ENABLED
    }
}
#[doc = "Possible values of the field `ENDTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTXR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDTXR {
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
            ENDTXR::DISABLED => false,
            ENDTXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDTXR {
        match value {
            false => ENDTXR::DISABLED,
            true => ENDTXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDTXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDTXR::ENABLED
    }
}
#[doc = "Possible values of the field `AUTOCOLRESSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOCOLRESSTARTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl AUTOCOLRESSTARTEDR {
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
            AUTOCOLRESSTARTEDR::DISABLED => false,
            AUTOCOLRESSTARTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOCOLRESSTARTEDR {
        match value {
            false => AUTOCOLRESSTARTEDR::DISABLED,
            true => AUTOCOLRESSTARTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOCOLRESSTARTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOCOLRESSTARTEDR::ENABLED
    }
}
#[doc = "Possible values of the field `COLLISION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLLISIONR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl COLLISIONR {
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
            COLLISIONR::DISABLED => false,
            COLLISIONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COLLISIONR {
        match value {
            false => COLLISIONR::DISABLED,
            true => COLLISIONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COLLISIONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COLLISIONR::ENABLED
    }
}
#[doc = "Possible values of the field `SELECTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SELECTEDR {
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
            SELECTEDR::DISABLED => false,
            SELECTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELECTEDR {
        match value {
            false => SELECTEDR::DISABLED,
            true => SELECTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SELECTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SELECTEDR::ENABLED
    }
}
#[doc = "Possible values of the field `STARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl STARTEDR {
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
            STARTEDR::DISABLED => false,
            STARTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTEDR {
        match value {
            false => STARTEDR::DISABLED,
            true => STARTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STARTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == STARTEDR::ENABLED
    }
}
#[doc = "Values that can be written to the field `READY`"]
pub enum READYW {
    #[doc = "Disable"]
    CLEAR,
}
impl READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READYW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READYW<'a> {
    w: &'a mut W,
}
impl<'a> _READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(READYW::CLEAR)
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
#[doc = "Values that can be written to the field `FIELDDETECTED`"]
pub enum FIELDDETECTEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl FIELDDETECTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIELDDETECTEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIELDDETECTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _FIELDDETECTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIELDDETECTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(FIELDDETECTEDW::CLEAR)
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
#[doc = "Values that can be written to the field `FIELDLOST`"]
pub enum FIELDLOSTW {
    #[doc = "Disable"]
    CLEAR,
}
impl FIELDLOSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIELDLOSTW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIELDLOSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FIELDLOSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIELDLOSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(FIELDLOSTW::CLEAR)
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
#[doc = "Values that can be written to the field `TXFRAMESTART`"]
pub enum TXFRAMESTARTW {
    #[doc = "Disable"]
    CLEAR,
}
impl TXFRAMESTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFRAMESTARTW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFRAMESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFRAMESTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFRAMESTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFRAMESTARTW::CLEAR)
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
#[doc = "Values that can be written to the field `TXFRAMEEND`"]
pub enum TXFRAMEENDW {
    #[doc = "Disable"]
    CLEAR,
}
impl TXFRAMEENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFRAMEENDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFRAMEENDW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFRAMEENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFRAMEENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFRAMEENDW::CLEAR)
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
#[doc = "Values that can be written to the field `RXFRAMESTART`"]
pub enum RXFRAMESTARTW {
    #[doc = "Disable"]
    CLEAR,
}
impl RXFRAMESTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFRAMESTARTW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFRAMESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFRAMESTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFRAMESTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFRAMESTARTW::CLEAR)
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
#[doc = "Values that can be written to the field `RXFRAMEEND`"]
pub enum RXFRAMEENDW {
    #[doc = "Disable"]
    CLEAR,
}
impl RXFRAMEENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFRAMEENDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFRAMEENDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFRAMEENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFRAMEENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFRAMEENDW::CLEAR)
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
#[doc = "Values that can be written to the field `ERROR`"]
pub enum ERRORW {
    #[doc = "Disable"]
    CLEAR,
}
impl ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRORW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRORW::CLEAR)
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
#[doc = "Values that can be written to the field `RXERROR`"]
pub enum RXERRORW {
    #[doc = "Disable"]
    CLEAR,
}
impl RXERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXERRORW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXERRORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXERRORW::CLEAR)
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
#[doc = "Values that can be written to the field `ENDRX`"]
pub enum ENDRXW {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDRXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDRXW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDRXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDRXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDRXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDRXW::CLEAR)
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
#[doc = "Values that can be written to the field `ENDTX`"]
pub enum ENDTXW {
    #[doc = "Disable"]
    CLEAR,
}
impl ENDTXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDTXW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDTXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDTXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDTXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDTXW::CLEAR)
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
#[doc = "Values that can be written to the field `AUTOCOLRESSTARTED`"]
pub enum AUTOCOLRESSTARTEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl AUTOCOLRESSTARTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOCOLRESSTARTEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOCOLRESSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOCOLRESSTARTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOCOLRESSTARTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(AUTOCOLRESSTARTEDW::CLEAR)
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
#[doc = "Values that can be written to the field `COLLISION`"]
pub enum COLLISIONW {
    #[doc = "Disable"]
    CLEAR,
}
impl COLLISIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COLLISIONW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COLLISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _COLLISIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COLLISIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(COLLISIONW::CLEAR)
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
#[doc = "Values that can be written to the field `SELECTED`"]
pub enum SELECTEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl SELECTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELECTEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELECTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SELECTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELECTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SELECTEDW::CLEAR)
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
#[doc = "Values that can be written to the field `STARTED`"]
pub enum STARTEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl STARTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(STARTEDW::CLEAR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write '1' to Disable interrupt for READY event"]
    #[inline]
    pub fn ready(&self) -> READYR {
        READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for FIELDDETECTED event"]
    #[inline]
    pub fn fielddetected(&self) -> FIELDDETECTEDR {
        FIELDDETECTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for FIELDLOST event"]
    #[inline]
    pub fn fieldlost(&self) -> FIELDLOSTR {
        FIELDLOSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for TXFRAMESTART event"]
    #[inline]
    pub fn txframestart(&self) -> TXFRAMESTARTR {
        TXFRAMESTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for TXFRAMEEND event"]
    #[inline]
    pub fn txframeend(&self) -> TXFRAMEENDR {
        TXFRAMEENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for RXFRAMESTART event"]
    #[inline]
    pub fn rxframestart(&self) -> RXFRAMESTARTR {
        RXFRAMESTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for RXFRAMEEND event"]
    #[inline]
    pub fn rxframeend(&self) -> RXFRAMEENDR {
        RXFRAMEENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for ERROR event"]
    #[inline]
    pub fn error(&self) -> ERRORR {
        ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Write '1' to Disable interrupt for RXERROR event"]
    #[inline]
    pub fn rxerror(&self) -> RXERRORR {
        RXERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Write '1' to Disable interrupt for ENDRX event"]
    #[inline]
    pub fn endrx(&self) -> ENDRXR {
        ENDRXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Write '1' to Disable interrupt for ENDTX event"]
    #[inline]
    pub fn endtx(&self) -> ENDTXR {
        ENDTXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Write '1' to Disable interrupt for AUTOCOLRESSTARTED event"]
    #[inline]
    pub fn autocolresstarted(&self) -> AUTOCOLRESSTARTEDR {
        AUTOCOLRESSTARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Write '1' to Disable interrupt for COLLISION event"]
    #[inline]
    pub fn collision(&self) -> COLLISIONR {
        COLLISIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Write '1' to Disable interrupt for SELECTED event"]
    #[inline]
    pub fn selected(&self) -> SELECTEDR {
        SELECTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Write '1' to Disable interrupt for STARTED event"]
    #[inline]
    pub fn started(&self) -> STARTEDR {
        STARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Write '1' to Disable interrupt for READY event"]
    #[inline]
    pub fn ready(&mut self) -> _READYW {
        _READYW { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for FIELDDETECTED event"]
    #[inline]
    pub fn fielddetected(&mut self) -> _FIELDDETECTEDW {
        _FIELDDETECTEDW { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for FIELDLOST event"]
    #[inline]
    pub fn fieldlost(&mut self) -> _FIELDLOSTW {
        _FIELDLOSTW { w: self }
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for TXFRAMESTART event"]
    #[inline]
    pub fn txframestart(&mut self) -> _TXFRAMESTARTW {
        _TXFRAMESTARTW { w: self }
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for TXFRAMEEND event"]
    #[inline]
    pub fn txframeend(&mut self) -> _TXFRAMEENDW {
        _TXFRAMEENDW { w: self }
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for RXFRAMESTART event"]
    #[inline]
    pub fn rxframestart(&mut self) -> _RXFRAMESTARTW {
        _RXFRAMESTARTW { w: self }
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for RXFRAMEEND event"]
    #[inline]
    pub fn rxframeend(&mut self) -> _RXFRAMEENDW {
        _RXFRAMEENDW { w: self }
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for ERROR event"]
    #[inline]
    pub fn error(&mut self) -> _ERRORW {
        _ERRORW { w: self }
    }
    #[doc = "Bit 10 - Write '1' to Disable interrupt for RXERROR event"]
    #[inline]
    pub fn rxerror(&mut self) -> _RXERRORW {
        _RXERRORW { w: self }
    }
    #[doc = "Bit 11 - Write '1' to Disable interrupt for ENDRX event"]
    #[inline]
    pub fn endrx(&mut self) -> _ENDRXW {
        _ENDRXW { w: self }
    }
    #[doc = "Bit 12 - Write '1' to Disable interrupt for ENDTX event"]
    #[inline]
    pub fn endtx(&mut self) -> _ENDTXW {
        _ENDTXW { w: self }
    }
    #[doc = "Bit 14 - Write '1' to Disable interrupt for AUTOCOLRESSTARTED event"]
    #[inline]
    pub fn autocolresstarted(&mut self) -> _AUTOCOLRESSTARTEDW {
        _AUTOCOLRESSTARTEDW { w: self }
    }
    #[doc = "Bit 18 - Write '1' to Disable interrupt for COLLISION event"]
    #[inline]
    pub fn collision(&mut self) -> _COLLISIONW {
        _COLLISIONW { w: self }
    }
    #[doc = "Bit 19 - Write '1' to Disable interrupt for SELECTED event"]
    #[inline]
    pub fn selected(&mut self) -> _SELECTEDW {
        _SELECTEDW { w: self }
    }
    #[doc = "Bit 20 - Write '1' to Disable interrupt for STARTED event"]
    #[inline]
    pub fn started(&mut self) -> _STARTEDW {
        _STARTEDW { w: self }
    }
}
