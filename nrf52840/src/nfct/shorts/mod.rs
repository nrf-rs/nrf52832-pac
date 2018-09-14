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
#[doc = "Possible values of the field `FIELDDETECTED_ACTIVATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDDETECTED_ACTIVATER {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl FIELDDETECTED_ACTIVATER {
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
            FIELDDETECTED_ACTIVATER::DISABLED => false,
            FIELDDETECTED_ACTIVATER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIELDDETECTED_ACTIVATER {
        match value {
            false => FIELDDETECTED_ACTIVATER::DISABLED,
            true => FIELDDETECTED_ACTIVATER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDDETECTED_ACTIVATER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDDETECTED_ACTIVATER::ENABLED
    }
}
#[doc = "Possible values of the field `FIELDLOST_SENSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDLOST_SENSER {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl FIELDLOST_SENSER {
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
            FIELDLOST_SENSER::DISABLED => false,
            FIELDLOST_SENSER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIELDLOST_SENSER {
        match value {
            false => FIELDLOST_SENSER::DISABLED,
            true => FIELDLOST_SENSER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDLOST_SENSER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDLOST_SENSER::ENABLED
    }
}
#[doc = "Possible values of the field `TXFRAMEEND_ENABLERXDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMEEND_ENABLERXDATAR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl TXFRAMEEND_ENABLERXDATAR {
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
            TXFRAMEEND_ENABLERXDATAR::DISABLED => false,
            TXFRAMEEND_ENABLERXDATAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFRAMEEND_ENABLERXDATAR {
        match value {
            false => TXFRAMEEND_ENABLERXDATAR::DISABLED,
            true => TXFRAMEEND_ENABLERXDATAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXFRAMEEND_ENABLERXDATAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMEEND_ENABLERXDATAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FIELDDETECTED_ACTIVATE`"]
pub enum FIELDDETECTED_ACTIVATEW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl FIELDDETECTED_ACTIVATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIELDDETECTED_ACTIVATEW::DISABLED => false,
            FIELDDETECTED_ACTIVATEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIELDDETECTED_ACTIVATEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIELDDETECTED_ACTIVATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIELDDETECTED_ACTIVATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_ACTIVATEW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_ACTIVATEW::ENABLED)
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
#[doc = "Values that can be written to the field `FIELDLOST_SENSE`"]
pub enum FIELDLOST_SENSEW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl FIELDLOST_SENSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIELDLOST_SENSEW::DISABLED => false,
            FIELDLOST_SENSEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIELDLOST_SENSEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIELDLOST_SENSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIELDLOST_SENSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDLOST_SENSEW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDLOST_SENSEW::ENABLED)
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
#[doc = "Values that can be written to the field `TXFRAMEEND_ENABLERXDATA`"]
pub enum TXFRAMEEND_ENABLERXDATAW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl TXFRAMEEND_ENABLERXDATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFRAMEEND_ENABLERXDATAW::DISABLED => false,
            TXFRAMEEND_ENABLERXDATAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFRAMEEND_ENABLERXDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFRAMEEND_ENABLERXDATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFRAMEEND_ENABLERXDATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFRAMEEND_ENABLERXDATAW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFRAMEEND_ENABLERXDATAW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Shortcut between FIELDDETECTED event and ACTIVATE task"]
    #[inline]
    pub fn fielddetected_activate(&self) -> FIELDDETECTED_ACTIVATER {
        FIELDDETECTED_ACTIVATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Shortcut between FIELDLOST event and SENSE task"]
    #[inline]
    pub fn fieldlost_sense(&self) -> FIELDLOST_SENSER {
        FIELDLOST_SENSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Shortcut between TXFRAMEEND event and ENABLERXDATA task"]
    #[inline]
    pub fn txframeend_enablerxdata(&self) -> TXFRAMEEND_ENABLERXDATAR {
        TXFRAMEEND_ENABLERXDATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Shortcut between FIELDDETECTED event and ACTIVATE task"]
    #[inline]
    pub fn fielddetected_activate(&mut self) -> _FIELDDETECTED_ACTIVATEW {
        _FIELDDETECTED_ACTIVATEW { w: self }
    }
    #[doc = "Bit 1 - Shortcut between FIELDLOST event and SENSE task"]
    #[inline]
    pub fn fieldlost_sense(&mut self) -> _FIELDLOST_SENSEW {
        _FIELDLOST_SENSEW { w: self }
    }
    #[doc = "Bit 5 - Shortcut between TXFRAMEEND event and ENABLERXDATA task"]
    #[inline]
    pub fn txframeend_enablerxdata(&mut self) -> _TXFRAMEEND_ENABLERXDATAW {
        _TXFRAMEEND_ENABLERXDATAW { w: self }
    }
}
