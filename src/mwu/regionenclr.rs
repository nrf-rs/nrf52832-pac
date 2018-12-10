#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REGIONENCLR {
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
#[doc = "Possible values of the field `RGN0WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN0WAR {
    #[doc = "Write access watch in this region is disabled"]
    DISABLED,
    #[doc = "Write access watch in this region is enabled"]
    ENABLED,
}
impl RGN0WAR {
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
            RGN0WAR::DISABLED => false,
            RGN0WAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN0WAR {
        match value {
            false => RGN0WAR::DISABLED,
            true => RGN0WAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RGN0WAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RGN0WAR::ENABLED
    }
}
#[doc = "Possible values of the field `RGN0RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN0RAR {
    #[doc = "Read access watch in this region is disabled"]
    DISABLED,
    #[doc = "Read access watch in this region is enabled"]
    ENABLED,
}
impl RGN0RAR {
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
            RGN0RAR::DISABLED => false,
            RGN0RAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN0RAR {
        match value {
            false => RGN0RAR::DISABLED,
            true => RGN0RAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RGN0RAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RGN0RAR::ENABLED
    }
}
#[doc = "Possible values of the field `RGN1WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN1WAR {
    #[doc = "Write access watch in this region is disabled"]
    DISABLED,
    #[doc = "Write access watch in this region is enabled"]
    ENABLED,
}
impl RGN1WAR {
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
            RGN1WAR::DISABLED => false,
            RGN1WAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN1WAR {
        match value {
            false => RGN1WAR::DISABLED,
            true => RGN1WAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RGN1WAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RGN1WAR::ENABLED
    }
}
#[doc = "Possible values of the field `RGN1RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN1RAR {
    #[doc = "Read access watch in this region is disabled"]
    DISABLED,
    #[doc = "Read access watch in this region is enabled"]
    ENABLED,
}
impl RGN1RAR {
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
            RGN1RAR::DISABLED => false,
            RGN1RAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN1RAR {
        match value {
            false => RGN1RAR::DISABLED,
            true => RGN1RAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RGN1RAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RGN1RAR::ENABLED
    }
}
#[doc = "Possible values of the field `RGN2WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN2WAR {
    #[doc = "Write access watch in this region is disabled"]
    DISABLED,
    #[doc = "Write access watch in this region is enabled"]
    ENABLED,
}
impl RGN2WAR {
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
            RGN2WAR::DISABLED => false,
            RGN2WAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN2WAR {
        match value {
            false => RGN2WAR::DISABLED,
            true => RGN2WAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RGN2WAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RGN2WAR::ENABLED
    }
}
#[doc = "Possible values of the field `RGN2RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN2RAR {
    #[doc = "Read access watch in this region is disabled"]
    DISABLED,
    #[doc = "Read access watch in this region is enabled"]
    ENABLED,
}
impl RGN2RAR {
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
            RGN2RAR::DISABLED => false,
            RGN2RAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN2RAR {
        match value {
            false => RGN2RAR::DISABLED,
            true => RGN2RAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RGN2RAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RGN2RAR::ENABLED
    }
}
#[doc = "Possible values of the field `RGN3WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN3WAR {
    #[doc = "Write access watch in this region is disabled"]
    DISABLED,
    #[doc = "Write access watch in this region is enabled"]
    ENABLED,
}
impl RGN3WAR {
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
            RGN3WAR::DISABLED => false,
            RGN3WAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN3WAR {
        match value {
            false => RGN3WAR::DISABLED,
            true => RGN3WAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RGN3WAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RGN3WAR::ENABLED
    }
}
#[doc = "Possible values of the field `RGN3RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN3RAR {
    #[doc = "Read access watch in this region is disabled"]
    DISABLED,
    #[doc = "Read access watch in this region is enabled"]
    ENABLED,
}
impl RGN3RAR {
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
            RGN3RAR::DISABLED => false,
            RGN3RAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN3RAR {
        match value {
            false => RGN3RAR::DISABLED,
            true => RGN3RAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RGN3RAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RGN3RAR::ENABLED
    }
}
#[doc = "Possible values of the field `PRGN0WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN0WAR {
    #[doc = "Write access watch in this PREGION is disabled"]
    DISABLED,
    #[doc = "Write access watch in this PREGION is enabled"]
    ENABLED,
}
impl PRGN0WAR {
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
            PRGN0WAR::DISABLED => false,
            PRGN0WAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRGN0WAR {
        match value {
            false => PRGN0WAR::DISABLED,
            true => PRGN0WAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PRGN0WAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PRGN0WAR::ENABLED
    }
}
#[doc = "Possible values of the field `PRGN0RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN0RAR {
    #[doc = "Read access watch in this PREGION is disabled"]
    DISABLED,
    #[doc = "Read access watch in this PREGION is enabled"]
    ENABLED,
}
impl PRGN0RAR {
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
            PRGN0RAR::DISABLED => false,
            PRGN0RAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRGN0RAR {
        match value {
            false => PRGN0RAR::DISABLED,
            true => PRGN0RAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PRGN0RAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PRGN0RAR::ENABLED
    }
}
#[doc = "Possible values of the field `PRGN1WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN1WAR {
    #[doc = "Write access watch in this PREGION is disabled"]
    DISABLED,
    #[doc = "Write access watch in this PREGION is enabled"]
    ENABLED,
}
impl PRGN1WAR {
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
            PRGN1WAR::DISABLED => false,
            PRGN1WAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRGN1WAR {
        match value {
            false => PRGN1WAR::DISABLED,
            true => PRGN1WAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PRGN1WAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PRGN1WAR::ENABLED
    }
}
#[doc = "Possible values of the field `PRGN1RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN1RAR {
    #[doc = "Read access watch in this PREGION is disabled"]
    DISABLED,
    #[doc = "Read access watch in this PREGION is enabled"]
    ENABLED,
}
impl PRGN1RAR {
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
            PRGN1RAR::DISABLED => false,
            PRGN1RAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRGN1RAR {
        match value {
            false => PRGN1RAR::DISABLED,
            true => PRGN1RAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PRGN1RAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PRGN1RAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RGN0WA`"]
pub enum RGN0WAW {
    #[doc = "Disable write access watch in this region"]
    CLEAR,
}
impl RGN0WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN0WAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGN0WAW<'a> {
    w: &'a mut W,
}
impl<'a> _RGN0WAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGN0WAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN0WAW::CLEAR)
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
#[doc = "Values that can be written to the field `RGN0RA`"]
pub enum RGN0RAW {
    #[doc = "Disable read access watch in this region"]
    CLEAR,
}
impl RGN0RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN0RAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGN0RAW<'a> {
    w: &'a mut W,
}
impl<'a> _RGN0RAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGN0RAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN0RAW::CLEAR)
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
#[doc = "Values that can be written to the field `RGN1WA`"]
pub enum RGN1WAW {
    #[doc = "Disable write access watch in this region"]
    CLEAR,
}
impl RGN1WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN1WAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGN1WAW<'a> {
    w: &'a mut W,
}
impl<'a> _RGN1WAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGN1WAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN1WAW::CLEAR)
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
#[doc = "Values that can be written to the field `RGN1RA`"]
pub enum RGN1RAW {
    #[doc = "Disable read access watch in this region"]
    CLEAR,
}
impl RGN1RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN1RAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGN1RAW<'a> {
    w: &'a mut W,
}
impl<'a> _RGN1RAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGN1RAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN1RAW::CLEAR)
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
#[doc = "Values that can be written to the field `RGN2WA`"]
pub enum RGN2WAW {
    #[doc = "Disable write access watch in this region"]
    CLEAR,
}
impl RGN2WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN2WAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGN2WAW<'a> {
    w: &'a mut W,
}
impl<'a> _RGN2WAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGN2WAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN2WAW::CLEAR)
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
#[doc = "Values that can be written to the field `RGN2RA`"]
pub enum RGN2RAW {
    #[doc = "Disable read access watch in this region"]
    CLEAR,
}
impl RGN2RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN2RAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGN2RAW<'a> {
    w: &'a mut W,
}
impl<'a> _RGN2RAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGN2RAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN2RAW::CLEAR)
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
#[doc = "Values that can be written to the field `RGN3WA`"]
pub enum RGN3WAW {
    #[doc = "Disable write access watch in this region"]
    CLEAR,
}
impl RGN3WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN3WAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGN3WAW<'a> {
    w: &'a mut W,
}
impl<'a> _RGN3WAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGN3WAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN3WAW::CLEAR)
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
#[doc = "Values that can be written to the field `RGN3RA`"]
pub enum RGN3RAW {
    #[doc = "Disable read access watch in this region"]
    CLEAR,
}
impl RGN3RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN3RAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGN3RAW<'a> {
    w: &'a mut W,
}
impl<'a> _RGN3RAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGN3RAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN3RAW::CLEAR)
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
#[doc = "Values that can be written to the field `PRGN0WA`"]
pub enum PRGN0WAW {
    #[doc = "Disable write access watch in this PREGION"]
    CLEAR,
}
impl PRGN0WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRGN0WAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRGN0WAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRGN0WAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRGN0WAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable write access watch in this PREGION"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRGN0WAW::CLEAR)
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
#[doc = "Values that can be written to the field `PRGN0RA`"]
pub enum PRGN0RAW {
    #[doc = "Disable read access watch in this PREGION"]
    CLEAR,
}
impl PRGN0RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRGN0RAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRGN0RAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRGN0RAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRGN0RAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable read access watch in this PREGION"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRGN0RAW::CLEAR)
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
#[doc = "Values that can be written to the field `PRGN1WA`"]
pub enum PRGN1WAW {
    #[doc = "Disable write access watch in this PREGION"]
    CLEAR,
}
impl PRGN1WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRGN1WAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRGN1WAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRGN1WAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRGN1WAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable write access watch in this PREGION"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRGN1WAW::CLEAR)
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
#[doc = "Values that can be written to the field `PRGN1RA`"]
pub enum PRGN1RAW {
    #[doc = "Disable read access watch in this PREGION"]
    CLEAR,
}
impl PRGN1RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRGN1RAW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRGN1RAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRGN1RAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRGN1RAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable read access watch in this PREGION"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRGN1RAW::CLEAR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Disable write access watch in region\\[0\\]"]
    #[inline]
    pub fn rgn0wa(&self) -> RGN0WAR {
        RGN0WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Disable read access watch in region\\[0\\]"]
    #[inline]
    pub fn rgn0ra(&self) -> RGN0RAR {
        RGN0RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Disable write access watch in region\\[1\\]"]
    #[inline]
    pub fn rgn1wa(&self) -> RGN1WAR {
        RGN1WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Disable read access watch in region\\[1\\]"]
    #[inline]
    pub fn rgn1ra(&self) -> RGN1RAR {
        RGN1RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Disable write access watch in region\\[2\\]"]
    #[inline]
    pub fn rgn2wa(&self) -> RGN2WAR {
        RGN2WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Disable read access watch in region\\[2\\]"]
    #[inline]
    pub fn rgn2ra(&self) -> RGN2RAR {
        RGN2RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Disable write access watch in region\\[3\\]"]
    #[inline]
    pub fn rgn3wa(&self) -> RGN3WAR {
        RGN3WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Disable read access watch in region\\[3\\]"]
    #[inline]
    pub fn rgn3ra(&self) -> RGN3RAR {
        RGN3RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Disable write access watch in PREGION\\[0\\]"]
    #[inline]
    pub fn prgn0wa(&self) -> PRGN0WAR {
        PRGN0WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Disable read access watch in PREGION\\[0\\]"]
    #[inline]
    pub fn prgn0ra(&self) -> PRGN0RAR {
        PRGN0RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Disable write access watch in PREGION\\[1\\]"]
    #[inline]
    pub fn prgn1wa(&self) -> PRGN1WAR {
        PRGN1WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Disable read access watch in PREGION\\[1\\]"]
    #[inline]
    pub fn prgn1ra(&self) -> PRGN1RAR {
        PRGN1RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Disable write access watch in region\\[0\\]"]
    #[inline]
    pub fn rgn0wa(&mut self) -> _RGN0WAW {
        _RGN0WAW { w: self }
    }
    #[doc = "Bit 1 - Disable read access watch in region\\[0\\]"]
    #[inline]
    pub fn rgn0ra(&mut self) -> _RGN0RAW {
        _RGN0RAW { w: self }
    }
    #[doc = "Bit 2 - Disable write access watch in region\\[1\\]"]
    #[inline]
    pub fn rgn1wa(&mut self) -> _RGN1WAW {
        _RGN1WAW { w: self }
    }
    #[doc = "Bit 3 - Disable read access watch in region\\[1\\]"]
    #[inline]
    pub fn rgn1ra(&mut self) -> _RGN1RAW {
        _RGN1RAW { w: self }
    }
    #[doc = "Bit 4 - Disable write access watch in region\\[2\\]"]
    #[inline]
    pub fn rgn2wa(&mut self) -> _RGN2WAW {
        _RGN2WAW { w: self }
    }
    #[doc = "Bit 5 - Disable read access watch in region\\[2\\]"]
    #[inline]
    pub fn rgn2ra(&mut self) -> _RGN2RAW {
        _RGN2RAW { w: self }
    }
    #[doc = "Bit 6 - Disable write access watch in region\\[3\\]"]
    #[inline]
    pub fn rgn3wa(&mut self) -> _RGN3WAW {
        _RGN3WAW { w: self }
    }
    #[doc = "Bit 7 - Disable read access watch in region\\[3\\]"]
    #[inline]
    pub fn rgn3ra(&mut self) -> _RGN3RAW {
        _RGN3RAW { w: self }
    }
    #[doc = "Bit 24 - Disable write access watch in PREGION\\[0\\]"]
    #[inline]
    pub fn prgn0wa(&mut self) -> _PRGN0WAW {
        _PRGN0WAW { w: self }
    }
    #[doc = "Bit 25 - Disable read access watch in PREGION\\[0\\]"]
    #[inline]
    pub fn prgn0ra(&mut self) -> _PRGN0RAW {
        _PRGN0RAW { w: self }
    }
    #[doc = "Bit 26 - Disable write access watch in PREGION\\[1\\]"]
    #[inline]
    pub fn prgn1wa(&mut self) -> _PRGN1WAW {
        _PRGN1WAW { w: self }
    }
    #[doc = "Bit 27 - Disable read access watch in PREGION\\[1\\]"]
    #[inline]
    pub fn prgn1ra(&mut self) -> _PRGN1RAW {
        _PRGN1RAW { w: self }
    }
}
