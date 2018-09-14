#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRAMECONFIG {
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
#[doc = "Possible values of the field `PARITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYR {
    #[doc = "Parity is not added to TX frames"]
    NOPARITY,
    #[doc = "Parity is added to TX frames"]
    PARITY,
}
impl PARITYR {
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
            PARITYR::NOPARITY => false,
            PARITYR::PARITY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARITYR {
        match value {
            false => PARITYR::NOPARITY,
            true => PARITYR::PARITY,
        }
    }
    #[doc = "Checks if the value of the field is `NOPARITY`"]
    #[inline]
    pub fn is_no_parity(&self) -> bool {
        *self == PARITYR::NOPARITY
    }
    #[doc = "Checks if the value of the field is `PARITY`"]
    #[inline]
    pub fn is_parity(&self) -> bool {
        *self == PARITYR::PARITY
    }
}
#[doc = "Possible values of the field `DISCARDMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCARDMODER {
    #[doc = "Unused bits are discarded at end of frame (EoF)"]
    DISCARDEND,
    #[doc = "Unused bits are discarded at start of frame (SoF)"]
    DISCARDSTART,
}
impl DISCARDMODER {
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
            DISCARDMODER::DISCARDEND => false,
            DISCARDMODER::DISCARDSTART => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISCARDMODER {
        match value {
            false => DISCARDMODER::DISCARDEND,
            true => DISCARDMODER::DISCARDSTART,
        }
    }
    #[doc = "Checks if the value of the field is `DISCARDEND`"]
    #[inline]
    pub fn is_discard_end(&self) -> bool {
        *self == DISCARDMODER::DISCARDEND
    }
    #[doc = "Checks if the value of the field is `DISCARDSTART`"]
    #[inline]
    pub fn is_discard_start(&self) -> bool {
        *self == DISCARDMODER::DISCARDSTART
    }
}
#[doc = "Possible values of the field `SOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFR {
    #[doc = "SoF symbol not added"]
    NOSOF,
    #[doc = "SoF symbol added"]
    SOF,
}
impl SOFR {
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
            SOFR::NOSOF => false,
            SOFR::SOF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFR {
        match value {
            false => SOFR::NOSOF,
            true => SOFR::SOF,
        }
    }
    #[doc = "Checks if the value of the field is `NOSOF`"]
    #[inline]
    pub fn is_no_so_f(&self) -> bool {
        *self == SOFR::NOSOF
    }
    #[doc = "Checks if the value of the field is `SOF`"]
    #[inline]
    pub fn is_so_f(&self) -> bool {
        *self == SOFR::SOF
    }
}
#[doc = "Possible values of the field `CRCMODETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCMODETXR {
    #[doc = "CRC is not added to the frame"]
    NOCRCTX,
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    CRC16TX,
}
impl CRCMODETXR {
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
            CRCMODETXR::NOCRCTX => false,
            CRCMODETXR::CRC16TX => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCMODETXR {
        match value {
            false => CRCMODETXR::NOCRCTX,
            true => CRCMODETXR::CRC16TX,
        }
    }
    #[doc = "Checks if the value of the field is `NOCRCTX`"]
    #[inline]
    pub fn is_no_crctx(&self) -> bool {
        *self == CRCMODETXR::NOCRCTX
    }
    #[doc = "Checks if the value of the field is `CRC16TX`"]
    #[inline]
    pub fn is_crc16tx(&self) -> bool {
        *self == CRCMODETXR::CRC16TX
    }
}
#[doc = "Values that can be written to the field `PARITY`"]
pub enum PARITYW {
    #[doc = "Parity is not added to TX frames"]
    NOPARITY,
    #[doc = "Parity is added to TX frames"]
    PARITY,
}
impl PARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PARITYW::NOPARITY => false,
            PARITYW::PARITY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Parity is not added to TX frames"]
    #[inline]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITYW::NOPARITY)
    }
    #[doc = "Parity is added to TX frames"]
    #[inline]
    pub fn parity(self) -> &'a mut W {
        self.variant(PARITYW::PARITY)
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
#[doc = "Values that can be written to the field `DISCARDMODE`"]
pub enum DISCARDMODEW {
    #[doc = "Unused bits are discarded at end of frame (EoF)"]
    DISCARDEND,
    #[doc = "Unused bits are discarded at start of frame (SoF)"]
    DISCARDSTART,
}
impl DISCARDMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISCARDMODEW::DISCARDEND => false,
            DISCARDMODEW::DISCARDSTART => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCARDMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCARDMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCARDMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Unused bits are discarded at end of frame (EoF)"]
    #[inline]
    pub fn discard_end(self) -> &'a mut W {
        self.variant(DISCARDMODEW::DISCARDEND)
    }
    #[doc = "Unused bits are discarded at start of frame (SoF)"]
    #[inline]
    pub fn discard_start(self) -> &'a mut W {
        self.variant(DISCARDMODEW::DISCARDSTART)
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
#[doc = "Values that can be written to the field `SOF`"]
pub enum SOFW {
    #[doc = "SoF symbol not added"]
    NOSOF,
    #[doc = "SoF symbol added"]
    SOF,
}
impl SOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFW::NOSOF => false,
            SOFW::SOF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SoF symbol not added"]
    #[inline]
    pub fn no_so_f(self) -> &'a mut W {
        self.variant(SOFW::NOSOF)
    }
    #[doc = "SoF symbol added"]
    #[inline]
    pub fn so_f(self) -> &'a mut W {
        self.variant(SOFW::SOF)
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
#[doc = "Values that can be written to the field `CRCMODETX`"]
pub enum CRCMODETXW {
    #[doc = "CRC is not added to the frame"]
    NOCRCTX,
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    CRC16TX,
}
impl CRCMODETXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCMODETXW::NOCRCTX => false,
            CRCMODETXW::CRC16TX => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCMODETXW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCMODETXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCMODETXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC is not added to the frame"]
    #[inline]
    pub fn no_crctx(self) -> &'a mut W {
        self.variant(CRCMODETXW::NOCRCTX)
    }
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    #[inline]
    pub fn crc16tx(self) -> &'a mut W {
        self.variant(CRCMODETXW::CRC16TX)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Indicates if parity is added to the frame"]
    #[inline]
    pub fn parity(&self) -> PARITYR {
        PARITYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Discarding unused bits at start or end of a frame"]
    #[inline]
    pub fn discardmode(&self) -> DISCARDMODER {
        DISCARDMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Adding SoF or not in TX frames"]
    #[inline]
    pub fn sof(&self) -> SOFR {
        SOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CRC mode for outgoing frames"]
    #[inline]
    pub fn crcmodetx(&self) -> CRCMODETXR {
        CRCMODETXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 23 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Indicates if parity is added to the frame"]
    #[inline]
    pub fn parity(&mut self) -> _PARITYW {
        _PARITYW { w: self }
    }
    #[doc = "Bit 1 - Discarding unused bits at start or end of a frame"]
    #[inline]
    pub fn discardmode(&mut self) -> _DISCARDMODEW {
        _DISCARDMODEW { w: self }
    }
    #[doc = "Bit 2 - Adding SoF or not in TX frames"]
    #[inline]
    pub fn sof(&mut self) -> _SOFW {
        _SOFW { w: self }
    }
    #[doc = "Bit 4 - CRC mode for outgoing frames"]
    #[inline]
    pub fn crcmodetx(&mut self) -> _CRCMODETXW {
        _CRCMODETXW { w: self }
    }
}
