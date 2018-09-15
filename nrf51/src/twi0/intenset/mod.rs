#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `STOPPED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPEDR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl STOPPEDR {
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
            STOPPEDR::DISABLED => false,
            STOPPEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPPEDR {
        match value {
            false => STOPPEDR::DISABLED,
            true => STOPPEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STOPPEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == STOPPEDR::ENABLED
    }
}
#[doc = "Possible values of the field `RXDREADY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDREADYR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl RXDREADYR {
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
            RXDREADYR::DISABLED => false,
            RXDREADYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDREADYR {
        match value {
            false => RXDREADYR::DISABLED,
            true => RXDREADYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXDREADYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXDREADYR::ENABLED
    }
}
#[doc = "Possible values of the field `TXDSENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDSENTR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl TXDSENTR {
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
            TXDSENTR::DISABLED => false,
            TXDSENTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDSENTR {
        match value {
            false => TXDSENTR::DISABLED,
            true => TXDSENTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXDSENTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXDSENTR::ENABLED
    }
}
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
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
#[doc = "Possible values of the field `BB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl BBR {
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
            BBR::DISABLED => false,
            BBR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BBR {
        match value {
            false => BBR::DISABLED,
            true => BBR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BBR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BBR::ENABLED
    }
}
#[doc = "Possible values of the field `SUSPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPENDEDR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl SUSPENDEDR {
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
            SUSPENDEDR::DISABLED => false,
            SUSPENDEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPENDEDR {
        match value {
            false => SUSPENDEDR::DISABLED,
            true => SUSPENDEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPENDEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPENDEDR::ENABLED
    }
}
#[doc = "Values that can be written to the field `STOPPED`"]
pub enum STOPPEDW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl STOPPEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPPEDW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPPEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPPEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(STOPPEDW::SET)
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
#[doc = "Values that can be written to the field `RXDREADY`"]
pub enum RXDREADYW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl RXDREADYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDREADYW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDREADYW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDREADYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDREADYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXDREADYW::SET)
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
#[doc = "Values that can be written to the field `TXDSENT`"]
pub enum TXDSENTW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl TXDSENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDSENTW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDSENTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDSENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDSENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TXDSENTW::SET)
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
#[doc = "Values that can be written to the field `ERROR`"]
pub enum ERRORW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRORW::SET => true,
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
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ERRORW::SET)
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
#[doc = "Values that can be written to the field `BB`"]
pub enum BBW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl BBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BBW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BBW<'a> {
    w: &'a mut W,
}
impl<'a> _BBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BBW::SET)
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
#[doc = "Values that can be written to the field `SUSPENDED`"]
pub enum SUSPENDEDW {
    #[doc = "Enable interrupt on write."]
    SET,
}
impl SUSPENDEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSPENDEDW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPENDEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSPENDEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SUSPENDEDW::SET)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Enable interrupt on STOPPED event."]
    #[inline]
    pub fn stopped(&self) -> STOPPEDR {
        STOPPEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable interrupt on READY event."]
    #[inline]
    pub fn rxdready(&self) -> RXDREADYR {
        RXDREADYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable interrupt on TXDSENT event."]
    #[inline]
    pub fn txdsent(&self) -> TXDSENTR {
        TXDSENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable interrupt on ERROR event."]
    #[inline]
    pub fn error(&self) -> ERRORR {
        ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable interrupt on BB event."]
    #[inline]
    pub fn bb(&self) -> BBR {
        BBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable interrupt on SUSPENDED event."]
    #[inline]
    pub fn suspended(&self) -> SUSPENDEDR {
        SUSPENDEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bit 1 - Enable interrupt on STOPPED event."]
    #[inline]
    pub fn stopped(&mut self) -> _STOPPEDW {
        _STOPPEDW { w: self }
    }
    #[doc = "Bit 2 - Enable interrupt on READY event."]
    #[inline]
    pub fn rxdready(&mut self) -> _RXDREADYW {
        _RXDREADYW { w: self }
    }
    #[doc = "Bit 7 - Enable interrupt on TXDSENT event."]
    #[inline]
    pub fn txdsent(&mut self) -> _TXDSENTW {
        _TXDSENTW { w: self }
    }
    #[doc = "Bit 9 - Enable interrupt on ERROR event."]
    #[inline]
    pub fn error(&mut self) -> _ERRORW {
        _ERRORW { w: self }
    }
    #[doc = "Bit 14 - Enable interrupt on BB event."]
    #[inline]
    pub fn bb(&mut self) -> _BBW {
        _BBW { w: self }
    }
    #[doc = "Bit 18 - Enable interrupt on SUSPENDED event."]
    #[inline]
    pub fn suspended(&mut self) -> _SUSPENDEDW {
        _SUSPENDEDW { w: self }
    }
}
