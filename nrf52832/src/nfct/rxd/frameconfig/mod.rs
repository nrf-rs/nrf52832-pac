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
    #[doc = "Parity is not expected in RX frames"]
    NOPARITY,
    #[doc = "Parity is expected in RX frames"]
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
#[doc = "Possible values of the field `SOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFR {
    #[doc = "Start of Frame symbol is not expected in RX frames"]
    NOSOF,
    #[doc = "Start of Frame symbol is expected in RX frames"]
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
#[doc = "Possible values of the field `CRCMODERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCMODERXR {
    #[doc = "CRC is not expected in RX frames"]
    NOCRCRX,
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    CRC16RX,
}
impl CRCMODERXR {
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
            CRCMODERXR::NOCRCRX => false,
            CRCMODERXR::CRC16RX => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCMODERXR {
        match value {
            false => CRCMODERXR::NOCRCRX,
            true => CRCMODERXR::CRC16RX,
        }
    }
    #[doc = "Checks if the value of the field is `NOCRCRX`"]
    #[inline]
    pub fn is_no_crcrx(&self) -> bool {
        *self == CRCMODERXR::NOCRCRX
    }
    #[doc = "Checks if the value of the field is `CRC16RX`"]
    #[inline]
    pub fn is_crc16rx(&self) -> bool {
        *self == CRCMODERXR::CRC16RX
    }
}
#[doc = "Values that can be written to the field `PARITY`"]
pub enum PARITYW {
    #[doc = "Parity is not expected in RX frames"]
    NOPARITY,
    #[doc = "Parity is expected in RX frames"]
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
    #[doc = "Parity is not expected in RX frames"]
    #[inline]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITYW::NOPARITY)
    }
    #[doc = "Parity is expected in RX frames"]
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
#[doc = "Values that can be written to the field `SOF`"]
pub enum SOFW {
    #[doc = "Start of Frame symbol is not expected in RX frames"]
    NOSOF,
    #[doc = "Start of Frame symbol is expected in RX frames"]
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
    #[doc = "Start of Frame symbol is not expected in RX frames"]
    #[inline]
    pub fn no_so_f(self) -> &'a mut W {
        self.variant(SOFW::NOSOF)
    }
    #[doc = "Start of Frame symbol is expected in RX frames"]
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
#[doc = "Values that can be written to the field `CRCMODERX`"]
pub enum CRCMODERXW {
    #[doc = "CRC is not expected in RX frames"]
    NOCRCRX,
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    CRC16RX,
}
impl CRCMODERXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCMODERXW::NOCRCRX => false,
            CRCMODERXW::CRC16RX => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCMODERXW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCMODERXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCMODERXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC is not expected in RX frames"]
    #[inline]
    pub fn no_crcrx(self) -> &'a mut W {
        self.variant(CRCMODERXW::NOCRCRX)
    }
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    #[inline]
    pub fn crc16rx(self) -> &'a mut W {
        self.variant(CRCMODERXW::CRC16RX)
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
    #[doc = "Bit 0 - Parity expected or not in RX frame"]
    #[inline]
    pub fn parity(&self) -> PARITYR {
        PARITYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SoF expected or not in RX frames"]
    #[inline]
    pub fn sof(&self) -> SOFR {
        SOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CRC mode for incoming frames"]
    #[inline]
    pub fn crcmoderx(&self) -> CRCMODERXR {
        CRCMODERXR::_from({
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
        W { bits: 21 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Parity expected or not in RX frame"]
    #[inline]
    pub fn parity(&mut self) -> _PARITYW {
        _PARITYW { w: self }
    }
    #[doc = "Bit 2 - SoF expected or not in RX frames"]
    #[inline]
    pub fn sof(&mut self) -> _SOFW {
        _SOFW { w: self }
    }
    #[doc = "Bit 4 - CRC mode for incoming frames"]
    #[inline]
    pub fn crcmoderx(&mut self) -> _CRCMODERXW {
        _CRCMODERXW { w: self }
    }
}
