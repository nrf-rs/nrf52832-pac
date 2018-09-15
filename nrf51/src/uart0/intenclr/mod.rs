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
#[doc = "Possible values of the field `CTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl CTSR {
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
            CTSR::DISABLED => false,
            CTSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSR {
        match value {
            false => CTSR::DISABLED,
            true => CTSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CTSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CTSR::ENABLED
    }
}
#[doc = "Possible values of the field `NCTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTSR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl NCTSR {
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
            NCTSR::DISABLED => false,
            NCTSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NCTSR {
        match value {
            false => NCTSR::DISABLED,
            true => NCTSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NCTSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NCTSR::ENABLED
    }
}
#[doc = "Possible values of the field `RXDRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDRDYR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl RXDRDYR {
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
            RXDRDYR::DISABLED => false,
            RXDRDYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDRDYR {
        match value {
            false => RXDRDYR::DISABLED,
            true => RXDRDYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXDRDYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXDRDYR::ENABLED
    }
}
#[doc = "Possible values of the field `TXDRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDRDYR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl TXDRDYR {
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
            TXDRDYR::DISABLED => false,
            TXDRDYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDRDYR {
        match value {
            false => TXDRDYR::DISABLED,
            true => TXDRDYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXDRDYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXDRDYR::ENABLED
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
#[doc = "Possible values of the field `RXTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOR {
    #[doc = "Interrupt disabled."]
    DISABLED,
    #[doc = "Interrupt enabled."]
    ENABLED,
}
impl RXTOR {
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
            RXTOR::DISABLED => false,
            RXTOR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTOR {
        match value {
            false => RXTOR::DISABLED,
            true => RXTOR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXTOR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXTOR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CTS`"]
pub enum CTSW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl CTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSW::CLEAR)
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
#[doc = "Values that can be written to the field `NCTS`"]
pub enum NCTSW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl NCTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NCTSW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NCTSW<'a> {
    w: &'a mut W,
}
impl<'a> _NCTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NCTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCTSW::CLEAR)
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
#[doc = "Values that can be written to the field `RXDRDY`"]
pub enum RXDRDYW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl RXDRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDRDYW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXDRDYW::CLEAR)
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
#[doc = "Values that can be written to the field `TXDRDY`"]
pub enum TXDRDYW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl TXDRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDRDYW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXDRDYW::CLEAR)
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
    #[doc = "Disable interrupt on write."]
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
    #[doc = "Disable interrupt on write."]
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXTO`"]
pub enum RXTOW {
    #[doc = "Disable interrupt on write."]
    CLEAR,
}
impl RXTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTOW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTOW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXTOW::CLEAR)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - Disable interrupt on CTS event."]
    #[inline]
    pub fn cts(&self) -> CTSR {
        CTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Disable interrupt on NCTS event."]
    #[inline]
    pub fn ncts(&self) -> NCTSR {
        NCTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Disable interrupt on RXRDY event."]
    #[inline]
    pub fn rxdrdy(&self) -> RXDRDYR {
        RXDRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Disable interrupt on TXRDY event."]
    #[inline]
    pub fn txdrdy(&self) -> TXDRDYR {
        TXDRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Disable interrupt on ERROR event."]
    #[inline]
    pub fn error(&self) -> ERRORR {
        ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Disable interrupt on RXTO event."]
    #[inline]
    pub fn rxto(&self) -> RXTOR {
        RXTOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - Disable interrupt on CTS event."]
    #[inline]
    pub fn cts(&mut self) -> _CTSW {
        _CTSW { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on NCTS event."]
    #[inline]
    pub fn ncts(&mut self) -> _NCTSW {
        _NCTSW { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on RXRDY event."]
    #[inline]
    pub fn rxdrdy(&mut self) -> _RXDRDYW {
        _RXDRDYW { w: self }
    }
    #[doc = "Bit 7 - Disable interrupt on TXRDY event."]
    #[inline]
    pub fn txdrdy(&mut self) -> _TXDRDYW {
        _TXDRDYW { w: self }
    }
    #[doc = "Bit 9 - Disable interrupt on ERROR event."]
    #[inline]
    pub fn error(&mut self) -> _ERRORW {
        _ERRORW { w: self }
    }
    #[doc = "Bit 17 - Disable interrupt on RXTO event."]
    #[inline]
    pub fn rxto(&mut self) -> _RXTOW {
        _RXTOW { w: self }
    }
}
