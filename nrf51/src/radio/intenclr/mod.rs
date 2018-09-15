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
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
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
#[doc = "Possible values of the field `ADDRESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESSR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl ADDRESSR {
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
            ADDRESSR::DISABLED => false,
            ADDRESSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRESSR {
        match value {
            false => ADDRESSR::DISABLED,
            true => ADDRESSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESSR::ENABLED
    }
}
#[doc = "Possible values of the field `PAYLOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAYLOADR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl PAYLOADR {
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
            PAYLOADR::DISABLED => false,
            PAYLOADR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAYLOADR {
        match value {
            false => PAYLOADR::DISABLED,
            true => PAYLOADR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PAYLOADR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PAYLOADR::ENABLED
    }
}
#[doc = "Possible values of the field `END`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl ENDR {
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
            ENDR::DISABLED => false,
            ENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDR {
        match value {
            false => ENDR::DISABLED,
            true => ENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDR::ENABLED
    }
}
#[doc = "Possible values of the field `DISABLED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLEDR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl DISABLEDR {
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
            DISABLEDR::DISABLED => false,
            DISABLEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISABLEDR {
        match value {
            false => DISABLEDR::DISABLED,
            true => DISABLEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLEDR::ENABLED
    }
}
#[doc = "Possible values of the field `DEVMATCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMATCHR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl DEVMATCHR {
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
            DEVMATCHR::DISABLED => false,
            DEVMATCHR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEVMATCHR {
        match value {
            false => DEVMATCHR::DISABLED,
            true => DEVMATCHR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DEVMATCHR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DEVMATCHR::ENABLED
    }
}
#[doc = "Possible values of the field `DEVMISS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMISSR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl DEVMISSR {
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
            DEVMISSR::DISABLED => false,
            DEVMISSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEVMISSR {
        match value {
            false => DEVMISSR::DISABLED,
            true => DEVMISSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DEVMISSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DEVMISSR::ENABLED
    }
}
#[doc = "Possible values of the field `RSSIEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSIENDR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl RSSIENDR {
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
            RSSIENDR::DISABLED => false,
            RSSIENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSSIENDR {
        match value {
            false => RSSIENDR::DISABLED,
            true => RSSIENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RSSIENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RSSIENDR::ENABLED
    }
}
#[doc = "Possible values of the field `BCMATCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCMATCHR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl BCMATCHR {
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
            BCMATCHR::DISABLED => false,
            BCMATCHR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCMATCHR {
        match value {
            false => BCMATCHR::DISABLED,
            true => BCMATCHR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BCMATCHR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BCMATCHR::ENABLED
    }
}
#[doc = "Values that can be written to the field `READY`"]
pub enum READYW {
    #[doc = "Disable interrupt on write."]
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
    #[doc = "Disable interrupt on write."]
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
#[doc = "Values that can be written to the field `ADDRESS`"]
pub enum ADDRESSW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl ADDRESSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRESSW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRESSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRESSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRESSW::CLEAR)
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
#[doc = "Values that can be written to the field `PAYLOAD`"]
pub enum PAYLOADW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl PAYLOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAYLOADW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAYLOADW<'a> {
    w: &'a mut W,
}
impl<'a> _PAYLOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAYLOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PAYLOADW::CLEAR)
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
#[doc = "Values that can be written to the field `END`"]
pub enum ENDW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl ENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDW::CLEAR)
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
#[doc = "Values that can be written to the field `DISABLED`"]
pub enum DISABLEDW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl DISABLEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISABLEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLEDW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(DISABLEDW::CLEAR)
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
#[doc = "Values that can be written to the field `DEVMATCH`"]
pub enum DEVMATCHW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl DEVMATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEVMATCHW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVMATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVMATCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVMATCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(DEVMATCHW::CLEAR)
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
#[doc = "Values that can be written to the field `DEVMISS`"]
pub enum DEVMISSW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl DEVMISSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEVMISSW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVMISSW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVMISSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVMISSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(DEVMISSW::CLEAR)
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
#[doc = "Values that can be written to the field `RSSIEND`"]
pub enum RSSIENDW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl RSSIENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSSIENDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSSIENDW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSIENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSSIENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSSIENDW::CLEAR)
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
#[doc = "Values that can be written to the field `BCMATCH`"]
pub enum BCMATCHW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl BCMATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCMATCHW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCMATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _BCMATCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCMATCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCMATCHW::CLEAR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Disable interrupt on READY event."]
    #[inline]
    pub fn ready(&self) -> READYR {
        READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Disable interrupt on ADDRESS event."]
    #[inline]
    pub fn address(&self) -> ADDRESSR {
        ADDRESSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Disable interrupt on PAYLOAD event."]
    #[inline]
    pub fn payload(&self) -> PAYLOADR {
        PAYLOADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Disable interrupt on END event."]
    #[inline]
    pub fn end(&self) -> ENDR {
        ENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Disable interrupt on DISABLED event."]
    #[inline]
    pub fn disabled(&self) -> DISABLEDR {
        DISABLEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Disable interrupt on DEVMATCH event."]
    #[inline]
    pub fn devmatch(&self) -> DEVMATCHR {
        DEVMATCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Disable interrupt on DEVMISS event."]
    #[inline]
    pub fn devmiss(&self) -> DEVMISSR {
        DEVMISSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Disable interrupt on RSSIEND event."]
    #[inline]
    pub fn rssiend(&self) -> RSSIENDR {
        RSSIENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Disable interrupt on BCMATCH event."]
    #[inline]
    pub fn bcmatch(&self) -> BCMATCHR {
        BCMATCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - Disable interrupt on READY event."]
    #[inline]
    pub fn ready(&mut self) -> _READYW {
        _READYW { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on ADDRESS event."]
    #[inline]
    pub fn address(&mut self) -> _ADDRESSW {
        _ADDRESSW { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on PAYLOAD event."]
    #[inline]
    pub fn payload(&mut self) -> _PAYLOADW {
        _PAYLOADW { w: self }
    }
    #[doc = "Bit 3 - Disable interrupt on END event."]
    #[inline]
    pub fn end(&mut self) -> _ENDW {
        _ENDW { w: self }
    }
    #[doc = "Bit 4 - Disable interrupt on DISABLED event."]
    #[inline]
    pub fn disabled(&mut self) -> _DISABLEDW {
        _DISABLEDW { w: self }
    }
    #[doc = "Bit 5 - Disable interrupt on DEVMATCH event."]
    #[inline]
    pub fn devmatch(&mut self) -> _DEVMATCHW {
        _DEVMATCHW { w: self }
    }
    #[doc = "Bit 6 - Disable interrupt on DEVMISS event."]
    #[inline]
    pub fn devmiss(&mut self) -> _DEVMISSW {
        _DEVMISSW { w: self }
    }
    #[doc = "Bit 7 - Disable interrupt on RSSIEND event."]
    #[inline]
    pub fn rssiend(&mut self) -> _RSSIENDW {
        _RSSIENDW { w: self }
    }
    #[doc = "Bit 10 - Disable interrupt on BCMATCH event."]
    #[inline]
    pub fn bcmatch(&mut self) -> _BCMATCHW {
        _BCMATCHW { w: self }
    }
}
