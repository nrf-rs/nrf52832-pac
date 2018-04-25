#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REGIONEN {
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
    #[doc = "Disable write access watch in this region"]
    DISABLE,
    #[doc = "Enable write access watch in this region"]
    ENABLE,
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
            RGN0WAR::DISABLE => false,
            RGN0WAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN0WAR {
        match value {
            false => RGN0WAR::DISABLE,
            true => RGN0WAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RGN0WAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RGN0WAR::ENABLE
    }
}
#[doc = "Possible values of the field `RGN0RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN0RAR {
    #[doc = "Disable read access watch in this region"]
    DISABLE,
    #[doc = "Enable read access watch in this region"]
    ENABLE,
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
            RGN0RAR::DISABLE => false,
            RGN0RAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN0RAR {
        match value {
            false => RGN0RAR::DISABLE,
            true => RGN0RAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RGN0RAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RGN0RAR::ENABLE
    }
}
#[doc = "Possible values of the field `RGN1WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN1WAR {
    #[doc = "Disable write access watch in this region"]
    DISABLE,
    #[doc = "Enable write access watch in this region"]
    ENABLE,
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
            RGN1WAR::DISABLE => false,
            RGN1WAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN1WAR {
        match value {
            false => RGN1WAR::DISABLE,
            true => RGN1WAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RGN1WAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RGN1WAR::ENABLE
    }
}
#[doc = "Possible values of the field `RGN1RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN1RAR {
    #[doc = "Disable read access watch in this region"]
    DISABLE,
    #[doc = "Enable read access watch in this region"]
    ENABLE,
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
            RGN1RAR::DISABLE => false,
            RGN1RAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN1RAR {
        match value {
            false => RGN1RAR::DISABLE,
            true => RGN1RAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RGN1RAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RGN1RAR::ENABLE
    }
}
#[doc = "Possible values of the field `RGN2WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN2WAR {
    #[doc = "Disable write access watch in this region"]
    DISABLE,
    #[doc = "Enable write access watch in this region"]
    ENABLE,
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
            RGN2WAR::DISABLE => false,
            RGN2WAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN2WAR {
        match value {
            false => RGN2WAR::DISABLE,
            true => RGN2WAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RGN2WAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RGN2WAR::ENABLE
    }
}
#[doc = "Possible values of the field `RGN2RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN2RAR {
    #[doc = "Disable read access watch in this region"]
    DISABLE,
    #[doc = "Enable read access watch in this region"]
    ENABLE,
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
            RGN2RAR::DISABLE => false,
            RGN2RAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN2RAR {
        match value {
            false => RGN2RAR::DISABLE,
            true => RGN2RAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RGN2RAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RGN2RAR::ENABLE
    }
}
#[doc = "Possible values of the field `RGN3WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN3WAR {
    #[doc = "Disable write access watch in this region"]
    DISABLE,
    #[doc = "Enable write access watch in this region"]
    ENABLE,
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
            RGN3WAR::DISABLE => false,
            RGN3WAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN3WAR {
        match value {
            false => RGN3WAR::DISABLE,
            true => RGN3WAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RGN3WAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RGN3WAR::ENABLE
    }
}
#[doc = "Possible values of the field `RGN3RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN3RAR {
    #[doc = "Disable read access watch in this region"]
    DISABLE,
    #[doc = "Enable read access watch in this region"]
    ENABLE,
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
            RGN3RAR::DISABLE => false,
            RGN3RAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGN3RAR {
        match value {
            false => RGN3RAR::DISABLE,
            true => RGN3RAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RGN3RAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RGN3RAR::ENABLE
    }
}
#[doc = "Possible values of the field `PRGN0WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN0WAR {
    #[doc = "Disable write access watch in this PREGION"]
    DISABLE,
    #[doc = "Enable write access watch in this PREGION"]
    ENABLE,
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
            PRGN0WAR::DISABLE => false,
            PRGN0WAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRGN0WAR {
        match value {
            false => PRGN0WAR::DISABLE,
            true => PRGN0WAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PRGN0WAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PRGN0WAR::ENABLE
    }
}
#[doc = "Possible values of the field `PRGN0RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN0RAR {
    #[doc = "Disable read access watch in this PREGION"]
    DISABLE,
    #[doc = "Enable read access watch in this PREGION"]
    ENABLE,
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
            PRGN0RAR::DISABLE => false,
            PRGN0RAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRGN0RAR {
        match value {
            false => PRGN0RAR::DISABLE,
            true => PRGN0RAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PRGN0RAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PRGN0RAR::ENABLE
    }
}
#[doc = "Possible values of the field `PRGN1WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN1WAR {
    #[doc = "Disable write access watch in this PREGION"]
    DISABLE,
    #[doc = "Enable write access watch in this PREGION"]
    ENABLE,
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
            PRGN1WAR::DISABLE => false,
            PRGN1WAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRGN1WAR {
        match value {
            false => PRGN1WAR::DISABLE,
            true => PRGN1WAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PRGN1WAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PRGN1WAR::ENABLE
    }
}
#[doc = "Possible values of the field `PRGN1RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN1RAR {
    #[doc = "Disable read access watch in this PREGION"]
    DISABLE,
    #[doc = "Enable read access watch in this PREGION"]
    ENABLE,
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
            PRGN1RAR::DISABLE => false,
            PRGN1RAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRGN1RAR {
        match value {
            false => PRGN1RAR::DISABLE,
            true => PRGN1RAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PRGN1RAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PRGN1RAR::ENABLE
    }
}
#[doc = "Values that can be written to the field `RGN0WA`"]
pub enum RGN0WAW {
    #[doc = "Disable write access watch in this region"]
    DISABLE,
    #[doc = "Enable write access watch in this region"]
    ENABLE,
}
impl RGN0WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN0WAW::DISABLE => false,
            RGN0WAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN0WAW::DISABLE)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN0WAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable read access watch in this region"]
    ENABLE,
}
impl RGN0RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN0RAW::DISABLE => false,
            RGN0RAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN0RAW::DISABLE)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN0RAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable write access watch in this region"]
    ENABLE,
}
impl RGN1WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN1WAW::DISABLE => false,
            RGN1WAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN1WAW::DISABLE)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN1WAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable read access watch in this region"]
    ENABLE,
}
impl RGN1RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN1RAW::DISABLE => false,
            RGN1RAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN1RAW::DISABLE)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN1RAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable write access watch in this region"]
    ENABLE,
}
impl RGN2WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN2WAW::DISABLE => false,
            RGN2WAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN2WAW::DISABLE)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN2WAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable read access watch in this region"]
    ENABLE,
}
impl RGN2RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN2RAW::DISABLE => false,
            RGN2RAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN2RAW::DISABLE)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN2RAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable write access watch in this region"]
    ENABLE,
}
impl RGN3WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN3WAW::DISABLE => false,
            RGN3WAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN3WAW::DISABLE)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN3WAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable read access watch in this region"]
    ENABLE,
}
impl RGN3RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGN3RAW::DISABLE => false,
            RGN3RAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN3RAW::DISABLE)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN3RAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable write access watch in this PREGION"]
    ENABLE,
}
impl PRGN0WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRGN0WAW::DISABLE => false,
            PRGN0WAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(PRGN0WAW::DISABLE)
    }
    #[doc = "Enable write access watch in this PREGION"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRGN0WAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable read access watch in this PREGION"]
    ENABLE,
}
impl PRGN0RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRGN0RAW::DISABLE => false,
            PRGN0RAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(PRGN0RAW::DISABLE)
    }
    #[doc = "Enable read access watch in this PREGION"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRGN0RAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable write access watch in this PREGION"]
    ENABLE,
}
impl PRGN1WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRGN1WAW::DISABLE => false,
            PRGN1WAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(PRGN1WAW::DISABLE)
    }
    #[doc = "Enable write access watch in this PREGION"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRGN1WAW::ENABLE)
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
    DISABLE,
    #[doc = "Enable read access watch in this PREGION"]
    ENABLE,
}
impl PRGN1RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRGN1RAW::DISABLE => false,
            PRGN1RAW::ENABLE => true,
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
    pub fn disable(self) -> &'a mut W {
        self.variant(PRGN1RAW::DISABLE)
    }
    #[doc = "Enable read access watch in this PREGION"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRGN1RAW::ENABLE)
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
    #[doc = "Bit 0 - Enable/disable write access watch in region[0]"]
    #[inline]
    pub fn rgn0wa(&self) -> RGN0WAR {
        RGN0WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable/disable read access watch in region[0]"]
    #[inline]
    pub fn rgn0ra(&self) -> RGN0RAR {
        RGN0RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable/disable write access watch in region[1]"]
    #[inline]
    pub fn rgn1wa(&self) -> RGN1WAR {
        RGN1WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable/disable read access watch in region[1]"]
    #[inline]
    pub fn rgn1ra(&self) -> RGN1RAR {
        RGN1RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable/disable write access watch in region[2]"]
    #[inline]
    pub fn rgn2wa(&self) -> RGN2WAR {
        RGN2WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable/disable read access watch in region[2]"]
    #[inline]
    pub fn rgn2ra(&self) -> RGN2RAR {
        RGN2RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable/disable write access watch in region[3]"]
    #[inline]
    pub fn rgn3wa(&self) -> RGN3WAR {
        RGN3WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable/disable read access watch in region[3]"]
    #[inline]
    pub fn rgn3ra(&self) -> RGN3RAR {
        RGN3RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable/disable write access watch in PREGION[0]"]
    #[inline]
    pub fn prgn0wa(&self) -> PRGN0WAR {
        PRGN0WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable/disable read access watch in PREGION[0]"]
    #[inline]
    pub fn prgn0ra(&self) -> PRGN0RAR {
        PRGN0RAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable/disable write access watch in PREGION[1]"]
    #[inline]
    pub fn prgn1wa(&self) -> PRGN1WAR {
        PRGN1WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable/disable read access watch in PREGION[1]"]
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
    #[doc = "Bit 0 - Enable/disable write access watch in region[0]"]
    #[inline]
    pub fn rgn0wa(&mut self) -> _RGN0WAW {
        _RGN0WAW { w: self }
    }
    #[doc = "Bit 1 - Enable/disable read access watch in region[0]"]
    #[inline]
    pub fn rgn0ra(&mut self) -> _RGN0RAW {
        _RGN0RAW { w: self }
    }
    #[doc = "Bit 2 - Enable/disable write access watch in region[1]"]
    #[inline]
    pub fn rgn1wa(&mut self) -> _RGN1WAW {
        _RGN1WAW { w: self }
    }
    #[doc = "Bit 3 - Enable/disable read access watch in region[1]"]
    #[inline]
    pub fn rgn1ra(&mut self) -> _RGN1RAW {
        _RGN1RAW { w: self }
    }
    #[doc = "Bit 4 - Enable/disable write access watch in region[2]"]
    #[inline]
    pub fn rgn2wa(&mut self) -> _RGN2WAW {
        _RGN2WAW { w: self }
    }
    #[doc = "Bit 5 - Enable/disable read access watch in region[2]"]
    #[inline]
    pub fn rgn2ra(&mut self) -> _RGN2RAW {
        _RGN2RAW { w: self }
    }
    #[doc = "Bit 6 - Enable/disable write access watch in region[3]"]
    #[inline]
    pub fn rgn3wa(&mut self) -> _RGN3WAW {
        _RGN3WAW { w: self }
    }
    #[doc = "Bit 7 - Enable/disable read access watch in region[3]"]
    #[inline]
    pub fn rgn3ra(&mut self) -> _RGN3RAW {
        _RGN3RAW { w: self }
    }
    #[doc = "Bit 24 - Enable/disable write access watch in PREGION[0]"]
    #[inline]
    pub fn prgn0wa(&mut self) -> _PRGN0WAW {
        _PRGN0WAW { w: self }
    }
    #[doc = "Bit 25 - Enable/disable read access watch in PREGION[0]"]
    #[inline]
    pub fn prgn0ra(&mut self) -> _PRGN0RAW {
        _PRGN0RAW { w: self }
    }
    #[doc = "Bit 26 - Enable/disable write access watch in PREGION[1]"]
    #[inline]
    pub fn prgn1wa(&mut self) -> _PRGN1WAW {
        _PRGN1WAW { w: self }
    }
    #[doc = "Bit 27 - Enable/disable read access watch in PREGION[1]"]
    #[inline]
    pub fn prgn1ra(&mut self) -> _PRGN1RAW {
        _PRGN1RAW { w: self }
    }
}
