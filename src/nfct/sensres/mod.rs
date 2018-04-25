#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SENSRES {
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
#[doc = "Possible values of the field `BITFRAMESDD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITFRAMESDDR {
    #[doc = "SDD pattern 00000"]
    SDD00000,
    #[doc = "SDD pattern 00001"]
    SDD00001,
    #[doc = "SDD pattern 00010"]
    SDD00010,
    #[doc = "SDD pattern 00100"]
    SDD00100,
    #[doc = "SDD pattern 01000"]
    SDD01000,
    #[doc = "SDD pattern 10000"]
    SDD10000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BITFRAMESDDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITFRAMESDDR::SDD00000 => 0,
            BITFRAMESDDR::SDD00001 => 1,
            BITFRAMESDDR::SDD00010 => 2,
            BITFRAMESDDR::SDD00100 => 4,
            BITFRAMESDDR::SDD01000 => 8,
            BITFRAMESDDR::SDD10000 => 16,
            BITFRAMESDDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITFRAMESDDR {
        match value {
            0 => BITFRAMESDDR::SDD00000,
            1 => BITFRAMESDDR::SDD00001,
            2 => BITFRAMESDDR::SDD00010,
            4 => BITFRAMESDDR::SDD00100,
            8 => BITFRAMESDDR::SDD01000,
            16 => BITFRAMESDDR::SDD10000,
            i => BITFRAMESDDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDD00000`"]
    #[inline]
    pub fn is_sdd00000(&self) -> bool {
        *self == BITFRAMESDDR::SDD00000
    }
    #[doc = "Checks if the value of the field is `SDD00001`"]
    #[inline]
    pub fn is_sdd00001(&self) -> bool {
        *self == BITFRAMESDDR::SDD00001
    }
    #[doc = "Checks if the value of the field is `SDD00010`"]
    #[inline]
    pub fn is_sdd00010(&self) -> bool {
        *self == BITFRAMESDDR::SDD00010
    }
    #[doc = "Checks if the value of the field is `SDD00100`"]
    #[inline]
    pub fn is_sdd00100(&self) -> bool {
        *self == BITFRAMESDDR::SDD00100
    }
    #[doc = "Checks if the value of the field is `SDD01000`"]
    #[inline]
    pub fn is_sdd01000(&self) -> bool {
        *self == BITFRAMESDDR::SDD01000
    }
    #[doc = "Checks if the value of the field is `SDD10000`"]
    #[inline]
    pub fn is_sdd10000(&self) -> bool {
        *self == BITFRAMESDDR::SDD10000
    }
}
#[doc = r" Value of the field"]
pub struct RFU5R {
    bits: bool,
}
impl RFU5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `NFCIDSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFCIDSIZER {
    #[doc = "NFCID1 size: single (4 bytes)"]
    NFCID1SINGLE,
    #[doc = "NFCID1 size: double (7 bytes)"]
    NFCID1DOUBLE,
    #[doc = "NFCID1 size: triple (10 bytes)"]
    NFCID1TRIPLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NFCIDSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NFCIDSIZER::NFCID1SINGLE => 0,
            NFCIDSIZER::NFCID1DOUBLE => 1,
            NFCIDSIZER::NFCID1TRIPLE => 2,
            NFCIDSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NFCIDSIZER {
        match value {
            0 => NFCIDSIZER::NFCID1SINGLE,
            1 => NFCIDSIZER::NFCID1DOUBLE,
            2 => NFCIDSIZER::NFCID1TRIPLE,
            i => NFCIDSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NFCID1SINGLE`"]
    #[inline]
    pub fn is_nfcid1single(&self) -> bool {
        *self == NFCIDSIZER::NFCID1SINGLE
    }
    #[doc = "Checks if the value of the field is `NFCID1DOUBLE`"]
    #[inline]
    pub fn is_nfcid1double(&self) -> bool {
        *self == NFCIDSIZER::NFCID1DOUBLE
    }
    #[doc = "Checks if the value of the field is `NFCID1TRIPLE`"]
    #[inline]
    pub fn is_nfcid1triple(&self) -> bool {
        *self == NFCIDSIZER::NFCID1TRIPLE
    }
}
#[doc = r" Value of the field"]
pub struct PLATFCONFIGR {
    bits: u8,
}
impl PLATFCONFIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFU74R {
    bits: u8,
}
impl RFU74R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `BITFRAMESDD`"]
pub enum BITFRAMESDDW {
    #[doc = "SDD pattern 00000"]
    SDD00000,
    #[doc = "SDD pattern 00001"]
    SDD00001,
    #[doc = "SDD pattern 00010"]
    SDD00010,
    #[doc = "SDD pattern 00100"]
    SDD00100,
    #[doc = "SDD pattern 01000"]
    SDD01000,
    #[doc = "SDD pattern 10000"]
    SDD10000,
}
impl BITFRAMESDDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BITFRAMESDDW::SDD00000 => 0,
            BITFRAMESDDW::SDD00001 => 1,
            BITFRAMESDDW::SDD00010 => 2,
            BITFRAMESDDW::SDD00100 => 4,
            BITFRAMESDDW::SDD01000 => 8,
            BITFRAMESDDW::SDD10000 => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITFRAMESDDW<'a> {
    w: &'a mut W,
}
impl<'a> _BITFRAMESDDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITFRAMESDDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDD pattern 00000"]
    #[inline]
    pub fn sdd00000(self) -> &'a mut W {
        self.variant(BITFRAMESDDW::SDD00000)
    }
    #[doc = "SDD pattern 00001"]
    #[inline]
    pub fn sdd00001(self) -> &'a mut W {
        self.variant(BITFRAMESDDW::SDD00001)
    }
    #[doc = "SDD pattern 00010"]
    #[inline]
    pub fn sdd00010(self) -> &'a mut W {
        self.variant(BITFRAMESDDW::SDD00010)
    }
    #[doc = "SDD pattern 00100"]
    #[inline]
    pub fn sdd00100(self) -> &'a mut W {
        self.variant(BITFRAMESDDW::SDD00100)
    }
    #[doc = "SDD pattern 01000"]
    #[inline]
    pub fn sdd01000(self) -> &'a mut W {
        self.variant(BITFRAMESDDW::SDD01000)
    }
    #[doc = "SDD pattern 10000"]
    #[inline]
    pub fn sdd10000(self) -> &'a mut W {
        self.variant(BITFRAMESDDW::SDD10000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFU5W<'a> {
    w: &'a mut W,
}
impl<'a> _RFU5W<'a> {
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
#[doc = "Values that can be written to the field `NFCIDSIZE`"]
pub enum NFCIDSIZEW {
    #[doc = "NFCID1 size: single (4 bytes)"]
    NFCID1SINGLE,
    #[doc = "NFCID1 size: double (7 bytes)"]
    NFCID1DOUBLE,
    #[doc = "NFCID1 size: triple (10 bytes)"]
    NFCID1TRIPLE,
}
impl NFCIDSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NFCIDSIZEW::NFCID1SINGLE => 0,
            NFCIDSIZEW::NFCID1DOUBLE => 1,
            NFCIDSIZEW::NFCID1TRIPLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NFCIDSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _NFCIDSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NFCIDSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NFCID1 size: single (4 bytes)"]
    #[inline]
    pub fn nfcid1single(self) -> &'a mut W {
        self.variant(NFCIDSIZEW::NFCID1SINGLE)
    }
    #[doc = "NFCID1 size: double (7 bytes)"]
    #[inline]
    pub fn nfcid1double(self) -> &'a mut W {
        self.variant(NFCIDSIZEW::NFCID1DOUBLE)
    }
    #[doc = "NFCID1 size: triple (10 bytes)"]
    #[inline]
    pub fn nfcid1triple(self) -> &'a mut W {
        self.variant(NFCIDSIZEW::NFCID1TRIPLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLATFCONFIGW<'a> {
    w: &'a mut W,
}
impl<'a> _PLATFCONFIGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFU74W<'a> {
    w: &'a mut W,
}
impl<'a> _RFU74W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline]
    pub fn bitframesdd(&self) -> BITFRAMESDDR {
        BITFRAMESDDR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu5(&self) -> RFU5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFU5R { bits }
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the Auto collision resolution engine."]
    #[inline]
    pub fn nfcidsize(&self) -> NFCIDSIZER {
        NFCIDSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline]
    pub fn platfconfig(&self) -> PLATFCONFIGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLATFCONFIGR { bits }
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu74(&self) -> RFU74R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFU74R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline]
    pub fn bitframesdd(&mut self) -> _BITFRAMESDDW {
        _BITFRAMESDDW { w: self }
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu5(&mut self) -> _RFU5W {
        _RFU5W { w: self }
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the Auto collision resolution engine."]
    #[inline]
    pub fn nfcidsize(&mut self) -> _NFCIDSIZEW {
        _NFCIDSIZEW { w: self }
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline]
    pub fn platfconfig(&mut self) -> _PLATFCONFIGW {
        _PLATFCONFIGW { w: self }
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu74(&mut self) -> _RFU74W {
        _RFU74W { w: self }
    }
}
