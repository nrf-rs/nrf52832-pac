#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
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
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
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
#[doc = "Possible values of the field `RXSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTEDR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RXSTARTEDR {
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
            RXSTARTEDR::DISABLED => false,
            RXSTARTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXSTARTEDR {
        match value {
            false => RXSTARTEDR::DISABLED,
            true => RXSTARTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXSTARTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXSTARTEDR::ENABLED
    }
}
#[doc = "Possible values of the field `TXSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTEDR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl TXSTARTEDR {
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
            TXSTARTEDR::DISABLED => false,
            TXSTARTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSTARTEDR {
        match value {
            false => TXSTARTEDR::DISABLED,
            true => TXSTARTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXSTARTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXSTARTEDR::ENABLED
    }
}
#[doc = "Possible values of the field `WRITE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITER {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl WRITER {
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
            WRITER::DISABLED => false,
            WRITER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITER {
        match value {
            false => WRITER::DISABLED,
            true => WRITER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WRITER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WRITER::ENABLED
    }
}
#[doc = "Possible values of the field `READ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl READR {
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
            READR::DISABLED => false,
            READR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READR {
        match value {
            false => READR::DISABLED,
            true => READR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == READR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == READR::ENABLED
    }
}
#[doc = "Values that can be written to the field `STOPPED`"]
pub enum STOPPEDW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl STOPPEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPPEDW::DISABLED => false,
            STOPPEDW::ENABLED => true,
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
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOPPEDW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOPPEDW::ENABLED)
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
#[doc = "Values that can be written to the field `ERROR`"]
pub enum ERRORW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRORW::DISABLED => false,
            ERRORW::ENABLED => true,
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
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRORW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRORW::ENABLED)
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
#[doc = "Values that can be written to the field `RXSTARTED`"]
pub enum RXSTARTEDW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RXSTARTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXSTARTEDW::DISABLED => false,
            RXSTARTEDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSTARTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXSTARTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXSTARTEDW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXSTARTEDW::ENABLED)
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
#[doc = "Values that can be written to the field `TXSTARTED`"]
pub enum TXSTARTEDW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl TXSTARTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSTARTEDW::DISABLED => false,
            TXSTARTEDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSTARTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSTARTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXSTARTEDW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXSTARTEDW::ENABLED)
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
#[doc = "Values that can be written to the field `WRITE`"]
pub enum WRITEW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl WRITEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRITEW::DISABLED => false,
            WRITEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRITEW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITEW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WRITEW::ENABLED)
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
#[doc = "Values that can be written to the field `READ`"]
pub enum READW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl READW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READW::DISABLED => false,
            READW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READW<'a> {
    w: &'a mut W,
}
impl<'a> _READW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Enable or disable interrupt for STOPPED event"]
    #[inline]
    pub fn stopped(&self) -> STOPPEDR {
        STOPPEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable or disable interrupt for ERROR event"]
    #[inline]
    pub fn error(&self) -> ERRORR {
        ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable or disable interrupt for RXSTARTED event"]
    #[inline]
    pub fn rxstarted(&self) -> RXSTARTEDR {
        RXSTARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable or disable interrupt for TXSTARTED event"]
    #[inline]
    pub fn txstarted(&self) -> TXSTARTEDR {
        TXSTARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable or disable interrupt for WRITE event"]
    #[inline]
    pub fn write(&self) -> WRITER {
        WRITER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable or disable interrupt for READ event"]
    #[inline]
    pub fn read(&self) -> READR {
        READR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
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
    #[doc = "Bit 1 - Enable or disable interrupt for STOPPED event"]
    #[inline]
    pub fn stopped(&mut self) -> _STOPPEDW {
        _STOPPEDW { w: self }
    }
    #[doc = "Bit 9 - Enable or disable interrupt for ERROR event"]
    #[inline]
    pub fn error(&mut self) -> _ERRORW {
        _ERRORW { w: self }
    }
    #[doc = "Bit 19 - Enable or disable interrupt for RXSTARTED event"]
    #[inline]
    pub fn rxstarted(&mut self) -> _RXSTARTEDW {
        _RXSTARTEDW { w: self }
    }
    #[doc = "Bit 20 - Enable or disable interrupt for TXSTARTED event"]
    #[inline]
    pub fn txstarted(&mut self) -> _TXSTARTEDW {
        _TXSTARTEDW { w: self }
    }
    #[doc = "Bit 25 - Enable or disable interrupt for WRITE event"]
    #[inline]
    pub fn write(&mut self) -> _WRITEW {
        _WRITEW { w: self }
    }
    #[doc = "Bit 26 - Enable or disable interrupt for READ event"]
    #[inline]
    pub fn read(&mut self) -> _READW {
        _READW { w: self }
    }
}
