#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX {
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
#[doc = "Possible values of the field `CRCERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERRORR {
    #[doc = "Valid CRC detected"]
    CRCCORRECT,
    #[doc = "CRC received does not match local check"]
    CRCERROR,
}
impl CRCERRORR {
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
            CRCERRORR::CRCCORRECT => false,
            CRCERRORR::CRCERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCERRORR {
        match value {
            false => CRCERRORR::CRCCORRECT,
            true => CRCERRORR::CRCERROR,
        }
    }
    #[doc = "Checks if the value of the field is `CRCCORRECT`"]
    #[inline]
    pub fn is_crccorrect(&self) -> bool {
        *self == CRCERRORR::CRCCORRECT
    }
    #[doc = "Checks if the value of the field is `CRCERROR`"]
    #[inline]
    pub fn is_crcerror(&self) -> bool {
        *self == CRCERRORR::CRCERROR
    }
}
#[doc = "Possible values of the field `PARITYSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYSTATUSR {
    #[doc = "Frame received with parity OK"]
    PARITYOK,
    #[doc = "Frame received with parity error"]
    PARITYERROR,
}
impl PARITYSTATUSR {
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
            PARITYSTATUSR::PARITYOK => false,
            PARITYSTATUSR::PARITYERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARITYSTATUSR {
        match value {
            false => PARITYSTATUSR::PARITYOK,
            true => PARITYSTATUSR::PARITYERROR,
        }
    }
    #[doc = "Checks if the value of the field is `PARITYOK`"]
    #[inline]
    pub fn is_parity_ok(&self) -> bool {
        *self == PARITYSTATUSR::PARITYOK
    }
    #[doc = "Checks if the value of the field is `PARITYERROR`"]
    #[inline]
    pub fn is_parity_error(&self) -> bool {
        *self == PARITYSTATUSR::PARITYERROR
    }
}
#[doc = "Possible values of the field `OVERRUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUNR {
    #[doc = "No overrun detected"]
    NOOVERRUN,
    #[doc = "Overrun error"]
    OVERRUN,
}
impl OVERRUNR {
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
            OVERRUNR::NOOVERRUN => false,
            OVERRUNR::OVERRUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERRUNR {
        match value {
            false => OVERRUNR::NOOVERRUN,
            true => OVERRUNR::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVERRUNR::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline]
    pub fn is_overrun(&self) -> bool {
        *self == OVERRUNR::OVERRUN
    }
}
#[doc = "Values that can be written to the field `CRCERROR`"]
pub enum CRCERRORW {
    #[doc = "Valid CRC detected"]
    CRCCORRECT,
    #[doc = "CRC received does not match local check"]
    CRCERROR,
}
impl CRCERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCERRORW::CRCCORRECT => false,
            CRCERRORW::CRCERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCERRORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Valid CRC detected"]
    #[inline]
    pub fn crccorrect(self) -> &'a mut W {
        self.variant(CRCERRORW::CRCCORRECT)
    }
    #[doc = "CRC received does not match local check"]
    #[inline]
    pub fn crcerror(self) -> &'a mut W {
        self.variant(CRCERRORW::CRCERROR)
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
#[doc = "Values that can be written to the field `PARITYSTATUS`"]
pub enum PARITYSTATUSW {
    #[doc = "Frame received with parity OK"]
    PARITYOK,
    #[doc = "Frame received with parity error"]
    PARITYERROR,
}
impl PARITYSTATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PARITYSTATUSW::PARITYOK => false,
            PARITYSTATUSW::PARITYERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYSTATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYSTATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYSTATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame received with parity OK"]
    #[inline]
    pub fn parity_ok(self) -> &'a mut W {
        self.variant(PARITYSTATUSW::PARITYOK)
    }
    #[doc = "Frame received with parity error"]
    #[inline]
    pub fn parity_error(self) -> &'a mut W {
        self.variant(PARITYSTATUSW::PARITYERROR)
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
#[doc = "Values that can be written to the field `OVERRUN`"]
pub enum OVERRUNW {
    #[doc = "No overrun detected"]
    NOOVERRUN,
    #[doc = "Overrun error"]
    OVERRUN,
}
impl OVERRUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERRUNW::NOOVERRUN => false,
            OVERRUNW::OVERRUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERRUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERRUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun detected"]
    #[inline]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVERRUNW::NOOVERRUN)
    }
    #[doc = "Overrun error"]
    #[inline]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVERRUNW::OVERRUN)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - No valid End of Frame detected"]
    #[inline]
    pub fn crcerror(&self) -> CRCERRORR {
        CRCERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Parity status of received frame"]
    #[inline]
    pub fn paritystatus(&self) -> PARITYSTATUSR {
        PARITYSTATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Overrun detected"]
    #[inline]
    pub fn overrun(&self) -> OVERRUNR {
        OVERRUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - No valid End of Frame detected"]
    #[inline]
    pub fn crcerror(&mut self) -> _CRCERRORW {
        _CRCERRORW { w: self }
    }
    #[doc = "Bit 2 - Parity status of received frame"]
    #[inline]
    pub fn paritystatus(&mut self) -> _PARITYSTATUSW {
        _PARITYSTATUSW { w: self }
    }
    #[doc = "Bit 3 - Overrun detected"]
    #[inline]
    pub fn overrun(&mut self) -> _OVERRUNW {
        _OVERRUNW { w: self }
    }
}
