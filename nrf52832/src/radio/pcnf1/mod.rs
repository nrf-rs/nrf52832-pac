#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCNF1 {
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
#[doc = r" Value of the field"]
pub struct MAXLENR {
    bits: u8,
}
impl MAXLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STATLENR {
    bits: u8,
}
impl STATLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BALENR {
    bits: u8,
}
impl BALENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENDIAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANR {
    #[doc = "Least Significant bit on air first"]
    LITTLE,
    #[doc = "Most significant bit on air first"]
    BIG,
}
impl ENDIANR {
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
            ENDIANR::LITTLE => false,
            ENDIANR::BIG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDIANR {
        match value {
            false => ENDIANR::LITTLE,
            true => ENDIANR::BIG,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline]
    pub fn is_little(&self) -> bool {
        *self == ENDIANR::LITTLE
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline]
    pub fn is_big(&self) -> bool {
        *self == ENDIANR::BIG
    }
}
#[doc = "Possible values of the field `WHITEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHITEENR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl WHITEENR {
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
            WHITEENR::DISABLED => false,
            WHITEENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WHITEENR {
        match value {
            false => WHITEENR::DISABLED,
            true => WHITEENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WHITEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WHITEENR::ENABLED
    }
}
#[doc = r" Proxy"]
pub struct _MAXLENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STATLENW<'a> {
    w: &'a mut W,
}
impl<'a> _STATLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BALENW<'a> {
    w: &'a mut W,
}
impl<'a> _BALENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENDIAN`"]
pub enum ENDIANW {
    #[doc = "Least Significant bit on air first"]
    LITTLE,
    #[doc = "Most significant bit on air first"]
    BIG,
}
impl ENDIANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDIANW::LITTLE => false,
            ENDIANW::BIG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDIANW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDIANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDIANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Least Significant bit on air first"]
    #[inline]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIANW::LITTLE)
    }
    #[doc = "Most significant bit on air first"]
    #[inline]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIANW::BIG)
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
#[doc = "Values that can be written to the field `WHITEEN`"]
pub enum WHITEENW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl WHITEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WHITEENW::DISABLED => false,
            WHITEENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WHITEENW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WHITEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WHITEENW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WHITEENW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline]
    pub fn maxlen(&self) -> MAXLENR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXLENR { bits }
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline]
    pub fn statlen(&self) -> STATLENR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STATLENR { bits }
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline]
    pub fn balen(&self) -> BALENR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BALENR { bits }
    }
    #[doc = "Bit 24 - On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
    #[inline]
    pub fn endian(&self) -> ENDIANR {
        ENDIANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline]
    pub fn whiteen(&self) -> WHITEENR {
        WHITEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline]
    pub fn maxlen(&mut self) -> _MAXLENW {
        _MAXLENW { w: self }
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline]
    pub fn statlen(&mut self) -> _STATLENW {
        _STATLENW { w: self }
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline]
    pub fn balen(&mut self) -> _BALENW {
        _BALENW { w: self }
    }
    #[doc = "Bit 24 - On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
    #[inline]
    pub fn endian(&mut self) -> _ENDIANW {
        _ENDIANW { w: self }
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline]
    pub fn whiteen(&mut self) -> _WHITEENW {
        _WHITEENW { w: self }
    }
}
