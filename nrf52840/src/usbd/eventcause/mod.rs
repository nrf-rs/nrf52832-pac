#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTCAUSE {
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
#[doc = "Possible values of the field `ISOOUTCRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOOUTCRCR {
    #[doc = "No error detected"]
    NOTDETECTED,
    #[doc = "Error detected"]
    DETECTED,
}
impl ISOOUTCRCR {
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
            ISOOUTCRCR::NOTDETECTED => false,
            ISOOUTCRCR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISOOUTCRCR {
        match value {
            false => ISOOUTCRCR::NOTDETECTED,
            true => ISOOUTCRCR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == ISOOUTCRCR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == ISOOUTCRCR::DETECTED
    }
}
#[doc = "Possible values of the field `SUSPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPENDR {
    #[doc = "Suspend not detected"]
    NOTDETECTED,
    #[doc = "Suspend detected"]
    DETECTED,
}
impl SUSPENDR {
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
            SUSPENDR::NOTDETECTED => false,
            SUSPENDR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPENDR {
        match value {
            false => SUSPENDR::NOTDETECTED,
            true => SUSPENDR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == SUSPENDR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == SUSPENDR::DETECTED
    }
}
#[doc = "Possible values of the field `RESUME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUMER {
    #[doc = "Resume not detected"]
    NOTDETECTED,
    #[doc = "Resume detected"]
    DETECTED,
}
impl RESUMER {
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
            RESUMER::NOTDETECTED => false,
            RESUMER::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESUMER {
        match value {
            false => RESUMER::NOTDETECTED,
            true => RESUMER::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == RESUMER::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == RESUMER::DETECTED
    }
}
#[doc = "Possible values of the field `USBWUALLOWED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBWUALLOWEDR {
    #[doc = "Wake up not allowed"]
    NOTALLOWED,
    #[doc = "Wake up allowed"]
    ALLOWED,
}
impl USBWUALLOWEDR {
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
            USBWUALLOWEDR::NOTALLOWED => false,
            USBWUALLOWEDR::ALLOWED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBWUALLOWEDR {
        match value {
            false => USBWUALLOWEDR::NOTALLOWED,
            true => USBWUALLOWEDR::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline]
    pub fn is_not_allowed(&self) -> bool {
        *self == USBWUALLOWEDR::NOTALLOWED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline]
    pub fn is_allowed(&self) -> bool {
        *self == USBWUALLOWEDR::ALLOWED
    }
}
#[doc = "Possible values of the field `READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READYR {
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    NOTDETECTED,
    #[doc = "USBD peripheral is ready"]
    READY,
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
            READYR::NOTDETECTED => false,
            READYR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READYR {
        match value {
            false => READYR::NOTDETECTED,
            true => READYR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == READYR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == READYR::READY
    }
}
#[doc = "Values that can be written to the field `ISOOUTCRC`"]
pub enum ISOOUTCRCW {
    #[doc = "No error detected"]
    NOTDETECTED,
    #[doc = "Error detected"]
    DETECTED,
}
impl ISOOUTCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISOOUTCRCW::NOTDETECTED => false,
            ISOOUTCRCW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISOOUTCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _ISOOUTCRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISOOUTCRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error detected"]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(ISOOUTCRCW::NOTDETECTED)
    }
    #[doc = "Error detected"]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(ISOOUTCRCW::DETECTED)
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
#[doc = "Values that can be written to the field `SUSPEND`"]
pub enum SUSPENDW {
    #[doc = "Suspend not detected"]
    NOTDETECTED,
    #[doc = "Suspend detected"]
    DETECTED,
}
impl SUSPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSPENDW::NOTDETECTED => false,
            SUSPENDW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Suspend not detected"]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SUSPENDW::NOTDETECTED)
    }
    #[doc = "Suspend detected"]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(SUSPENDW::DETECTED)
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
#[doc = "Values that can be written to the field `RESUME`"]
pub enum RESUMEW {
    #[doc = "Resume not detected"]
    NOTDETECTED,
    #[doc = "Resume detected"]
    DETECTED,
}
impl RESUMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESUMEW::NOTDETECTED => false,
            RESUMEW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESUMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resume not detected"]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESUMEW::NOTDETECTED)
    }
    #[doc = "Resume detected"]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESUMEW::DETECTED)
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
#[doc = "Values that can be written to the field `USBWUALLOWED`"]
pub enum USBWUALLOWEDW {
    #[doc = "Wake up not allowed"]
    NOTALLOWED,
    #[doc = "Wake up allowed"]
    ALLOWED,
}
impl USBWUALLOWEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBWUALLOWEDW::NOTALLOWED => false,
            USBWUALLOWEDW::ALLOWED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBWUALLOWEDW<'a> {
    w: &'a mut W,
}
impl<'a> _USBWUALLOWEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBWUALLOWEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake up not allowed"]
    #[inline]
    pub fn not_allowed(self) -> &'a mut W {
        self.variant(USBWUALLOWEDW::NOTALLOWED)
    }
    #[doc = "Wake up allowed"]
    #[inline]
    pub fn allowed(self) -> &'a mut W {
        self.variant(USBWUALLOWEDW::ALLOWED)
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
#[doc = "Values that can be written to the field `READY`"]
pub enum READYW {
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    NOTDETECTED,
    #[doc = "USBD peripheral is ready"]
    READY,
}
impl READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READYW::NOTDETECTED => false,
            READYW::READY => true,
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
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(READYW::NOTDETECTED)
    }
    #[doc = "USBD peripheral is ready"]
    #[inline]
    pub fn ready(self) -> &'a mut W {
        self.variant(READYW::READY)
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
    #[doc = "Bit 0 - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline]
    pub fn isooutcrc(&self) -> ISOOUTCRCR {
        ISOOUTCRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline]
    pub fn suspend(&self) -> SUSPENDR {
        SUSPENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline]
    pub fn resume(&self) -> RESUMER {
        RESUMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline]
    pub fn usbwuallowed(&self) -> USBWUALLOWEDR {
        USBWUALLOWEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - USB device is ready for normal operation. Write '1' to clear."]
    #[inline]
    pub fn ready(&self) -> READYR {
        READYR::_from({
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
    #[doc = "Bit 0 - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline]
    pub fn isooutcrc(&mut self) -> _ISOOUTCRCW {
        _ISOOUTCRCW { w: self }
    }
    #[doc = "Bit 8 - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline]
    pub fn suspend(&mut self) -> _SUSPENDW {
        _SUSPENDW { w: self }
    }
    #[doc = "Bit 9 - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline]
    pub fn resume(&mut self) -> _RESUMEW {
        _RESUMEW { w: self }
    }
    #[doc = "Bit 10 - USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline]
    pub fn usbwuallowed(&mut self) -> _USBWUALLOWEDW {
        _USBWUALLOWEDW { w: self }
    }
    #[doc = "Bit 11 - USB device is ready for normal operation. Write '1' to clear."]
    #[inline]
    pub fn ready(&mut self) -> _READYW {
        _READYW { w: self }
    }
}
