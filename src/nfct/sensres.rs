#[doc = "Reader of register SENSRES"]
pub type R = crate::R<u32, super::SENSRES>;
#[doc = "Writer for register SENSRES"]
pub type W = crate::W<u32, super::SENSRES>;
#[doc = "Register SENSRES `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SENSRES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BITFRAMESDD_A {
    #[doc = "0: SDD pattern 00000"]
    SDD00000 = 0,
    #[doc = "1: SDD pattern 00001"]
    SDD00001 = 1,
    #[doc = "2: SDD pattern 00010"]
    SDD00010 = 2,
    #[doc = "4: SDD pattern 00100"]
    SDD00100 = 4,
    #[doc = "8: SDD pattern 01000"]
    SDD01000 = 8,
    #[doc = "16: SDD pattern 10000"]
    SDD10000 = 16,
}
impl From<BITFRAMESDD_A> for u8 {
    #[inline(always)]
    fn from(variant: BITFRAMESDD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BITFRAMESDD`"]
pub type BITFRAMESDD_R = crate::R<u8, BITFRAMESDD_A>;
impl BITFRAMESDD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BITFRAMESDD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BITFRAMESDD_A::SDD00000),
            1 => Val(BITFRAMESDD_A::SDD00001),
            2 => Val(BITFRAMESDD_A::SDD00010),
            4 => Val(BITFRAMESDD_A::SDD00100),
            8 => Val(BITFRAMESDD_A::SDD01000),
            16 => Val(BITFRAMESDD_A::SDD10000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDD00000`"]
    #[inline(always)]
    pub fn is_sdd00000(&self) -> bool {
        *self == BITFRAMESDD_A::SDD00000
    }
    #[doc = "Checks if the value of the field is `SDD00001`"]
    #[inline(always)]
    pub fn is_sdd00001(&self) -> bool {
        *self == BITFRAMESDD_A::SDD00001
    }
    #[doc = "Checks if the value of the field is `SDD00010`"]
    #[inline(always)]
    pub fn is_sdd00010(&self) -> bool {
        *self == BITFRAMESDD_A::SDD00010
    }
    #[doc = "Checks if the value of the field is `SDD00100`"]
    #[inline(always)]
    pub fn is_sdd00100(&self) -> bool {
        *self == BITFRAMESDD_A::SDD00100
    }
    #[doc = "Checks if the value of the field is `SDD01000`"]
    #[inline(always)]
    pub fn is_sdd01000(&self) -> bool {
        *self == BITFRAMESDD_A::SDD01000
    }
    #[doc = "Checks if the value of the field is `SDD10000`"]
    #[inline(always)]
    pub fn is_sdd10000(&self) -> bool {
        *self == BITFRAMESDD_A::SDD10000
    }
}
#[doc = "Write proxy for field `BITFRAMESDD`"]
pub struct BITFRAMESDD_W<'a> {
    w: &'a mut W,
}
impl<'a> BITFRAMESDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITFRAMESDD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDD pattern 00000"]
    #[inline(always)]
    pub fn sdd00000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00000)
    }
    #[doc = "SDD pattern 00001"]
    #[inline(always)]
    pub fn sdd00001(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00001)
    }
    #[doc = "SDD pattern 00010"]
    #[inline(always)]
    pub fn sdd00010(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00010)
    }
    #[doc = "SDD pattern 00100"]
    #[inline(always)]
    pub fn sdd00100(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00100)
    }
    #[doc = "SDD pattern 01000"]
    #[inline(always)]
    pub fn sdd01000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD01000)
    }
    #[doc = "SDD pattern 10000"]
    #[inline(always)]
    pub fn sdd10000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD10000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `RFU5`"]
pub type RFU5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFU5`"]
pub struct RFU5_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "NFCID1 size. This value is used by the Auto collision resolution engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NFCIDSIZE_A {
    #[doc = "0: NFCID1 size: single (4 bytes)"]
    NFCID1SINGLE = 0,
    #[doc = "1: NFCID1 size: double (7 bytes)"]
    NFCID1DOUBLE = 1,
    #[doc = "2: NFCID1 size: triple (10 bytes)"]
    NFCID1TRIPLE = 2,
}
impl From<NFCIDSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCIDSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NFCIDSIZE`"]
pub type NFCIDSIZE_R = crate::R<u8, NFCIDSIZE_A>;
impl NFCIDSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NFCIDSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NFCIDSIZE_A::NFCID1SINGLE),
            1 => Val(NFCIDSIZE_A::NFCID1DOUBLE),
            2 => Val(NFCIDSIZE_A::NFCID1TRIPLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NFCID1SINGLE`"]
    #[inline(always)]
    pub fn is_nfcid1single(&self) -> bool {
        *self == NFCIDSIZE_A::NFCID1SINGLE
    }
    #[doc = "Checks if the value of the field is `NFCID1DOUBLE`"]
    #[inline(always)]
    pub fn is_nfcid1double(&self) -> bool {
        *self == NFCIDSIZE_A::NFCID1DOUBLE
    }
    #[doc = "Checks if the value of the field is `NFCID1TRIPLE`"]
    #[inline(always)]
    pub fn is_nfcid1triple(&self) -> bool {
        *self == NFCIDSIZE_A::NFCID1TRIPLE
    }
}
#[doc = "Write proxy for field `NFCIDSIZE`"]
pub struct NFCIDSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCIDSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NFCIDSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NFCID1 size: single (4 bytes)"]
    #[inline(always)]
    pub fn nfcid1single(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1SINGLE)
    }
    #[doc = "NFCID1 size: double (7 bytes)"]
    #[inline(always)]
    pub fn nfcid1double(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1DOUBLE)
    }
    #[doc = "NFCID1 size: triple (10 bytes)"]
    #[inline(always)]
    pub fn nfcid1triple(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1TRIPLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PLATFCONFIG`"]
pub type PLATFCONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLATFCONFIG`"]
pub struct PLATFCONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> PLATFCONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RFU74`"]
pub type RFU74_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFU74`"]
pub struct RFU74_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU74_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn bitframesdd(&self) -> BITFRAMESDD_R {
        BITFRAMESDD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu5(&self) -> RFU5_R {
        RFU5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the Auto collision resolution engine."]
    #[inline(always)]
    pub fn nfcidsize(&self) -> NFCIDSIZE_R {
        NFCIDSIZE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn platfconfig(&self) -> PLATFCONFIG_R {
        PLATFCONFIG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu74(&self) -> RFU74_R {
        RFU74_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn bitframesdd(&mut self) -> BITFRAMESDD_W {
        BITFRAMESDD_W { w: self }
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu5(&mut self) -> RFU5_W {
        RFU5_W { w: self }
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the Auto collision resolution engine."]
    #[inline(always)]
    pub fn nfcidsize(&mut self) -> NFCIDSIZE_W {
        NFCIDSIZE_W { w: self }
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn platfconfig(&mut self) -> PLATFCONFIG_W {
        PLATFCONFIG_W { w: self }
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu74(&mut self) -> RFU74_W {
        RFU74_W { w: self }
    }
}
