#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERRORSRC {
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
#[doc = "Possible values of the field `OVERFLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOWR {
    #[doc = "Error did not occur"]
    NOTDETECTED,
    #[doc = "Error occurred"]
    DETECTED,
}
impl OVERFLOWR {
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
            OVERFLOWR::NOTDETECTED => false,
            OVERFLOWR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERFLOWR {
        match value {
            false => OVERFLOWR::NOTDETECTED,
            true => OVERFLOWR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == OVERFLOWR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == OVERFLOWR::DETECTED
    }
}
#[doc = "Possible values of the field `DNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACKR {
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
}
impl DNACKR {
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
            DNACKR::NOTRECEIVED => false,
            DNACKR::RECEIVED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DNACKR {
        match value {
            false => DNACKR::NOTRECEIVED,
            true => DNACKR::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline]
    pub fn is_not_received(&self) -> bool {
        *self == DNACKR::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline]
    pub fn is_received(&self) -> bool {
        *self == DNACKR::RECEIVED
    }
}
#[doc = "Possible values of the field `OVERREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREADR {
    #[doc = "Error did not occur"]
    NOTDETECTED,
    #[doc = "Error occurred"]
    DETECTED,
}
impl OVERREADR {
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
            OVERREADR::NOTDETECTED => false,
            OVERREADR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERREADR {
        match value {
            false => OVERREADR::NOTDETECTED,
            true => OVERREADR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == OVERREADR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == OVERREADR::DETECTED
    }
}
#[doc = "Values that can be written to the field `OVERFLOW`"]
pub enum OVERFLOWW {
    #[doc = "Error did not occur"]
    NOTDETECTED,
    #[doc = "Error occurred"]
    DETECTED,
}
impl OVERFLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERFLOWW::NOTDETECTED => false,
            OVERFLOWW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERFLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error did not occur"]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OVERFLOWW::NOTDETECTED)
    }
    #[doc = "Error occurred"]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(OVERFLOWW::DETECTED)
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
#[doc = "Values that can be written to the field `DNACK`"]
pub enum DNACKW {
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
}
impl DNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DNACKW::NOTRECEIVED => false,
            DNACKW::RECEIVED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error did not occur"]
    #[inline]
    pub fn not_received(self) -> &'a mut W {
        self.variant(DNACKW::NOTRECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline]
    pub fn received(self) -> &'a mut W {
        self.variant(DNACKW::RECEIVED)
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
#[doc = "Values that can be written to the field `OVERREAD`"]
pub enum OVERREADW {
    #[doc = "Error did not occur"]
    NOTDETECTED,
    #[doc = "Error occurred"]
    DETECTED,
}
impl OVERREADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERREADW::NOTDETECTED => false,
            OVERREADW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERREADW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERREADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERREADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error did not occur"]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OVERREADW::NOTDETECTED)
    }
    #[doc = "Error occurred"]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(OVERREADW::DETECTED)
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
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline]
    pub fn overflow(&self) -> OVERFLOWR {
        OVERFLOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline]
    pub fn dnack(&self) -> DNACKR {
        DNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline]
    pub fn overread(&self) -> OVERREADR {
        OVERREADR::_from({
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
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline]
    pub fn overflow(&mut self) -> _OVERFLOWW {
        _OVERFLOWW { w: self }
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline]
    pub fn dnack(&mut self) -> _DNACKW {
        _DNACKW { w: self }
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline]
    pub fn overread(&mut self) -> _OVERREADW {
        _OVERREADW { w: self }
    }
}
