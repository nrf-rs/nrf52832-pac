#[doc = "Reader of register CRCCNF"]
pub type R = crate::R<u32, super::CRCCNF>;
#[doc = "Writer for register CRCCNF"]
pub type W = crate::W<u32, super::CRCCNF>;
#[doc = "Register CRCCNF `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCCNF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CRC length in number of bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEN_A {
    #[doc = "0: CRC length is zero and CRC calculation is disabled"]
    DISABLED = 0,
    #[doc = "1: CRC length is one byte and CRC calculation is enabled"]
    ONE = 1,
    #[doc = "2: CRC length is two bytes and CRC calculation is enabled"]
    TWO = 2,
    #[doc = "3: CRC length is three bytes and CRC calculation is enabled"]
    THREE = 3,
}
impl From<LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEN`"]
pub type LEN_R = crate::R<u8, LEN_A>;
impl LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEN_A {
        match self.bits {
            0 => LEN_A::DISABLED,
            1 => LEN_A::ONE,
            2 => LEN_A::TWO,
            3 => LEN_A::THREE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == LEN_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == LEN_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == LEN_A::THREE
    }
}
#[doc = "Write proxy for field `LEN`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CRC length is zero and CRC calculation is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LEN_A::DISABLED)
    }
    #[doc = "CRC length is one byte and CRC calculation is enabled"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(LEN_A::ONE)
    }
    #[doc = "CRC length is two bytes and CRC calculation is enabled"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(LEN_A::TWO)
    }
    #[doc = "CRC length is three bytes and CRC calculation is enabled"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(LEN_A::THREE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Include or exclude packet address field out of CRC calculation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKIPADDR_A {
    #[doc = "0: CRC calculation includes address field"]
    INCLUDE = 0,
    #[doc = "1: CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
    SKIP = 1,
}
impl From<SKIPADDR_A> for bool {
    #[inline(always)]
    fn from(variant: SKIPADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SKIPADDR`"]
pub type SKIPADDR_R = crate::R<bool, SKIPADDR_A>;
impl SKIPADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKIPADDR_A {
        match self.bits {
            false => SKIPADDR_A::INCLUDE,
            true => SKIPADDR_A::SKIP,
        }
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == SKIPADDR_A::INCLUDE
    }
    #[doc = "Checks if the value of the field is `SKIP`"]
    #[inline(always)]
    pub fn is_skip(&self) -> bool {
        *self == SKIPADDR_A::SKIP
    }
}
#[doc = "Write proxy for field `SKIPADDR`"]
pub struct SKIPADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIPADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SKIPADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC calculation includes address field"]
    #[inline(always)]
    pub fn include(self) -> &'a mut W {
        self.variant(SKIPADDR_A::INCLUDE)
    }
    #[doc = "CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
    #[inline(always)]
    pub fn skip(self) -> &'a mut W {
        self.variant(SKIPADDR_A::SKIP)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC length in number of bytes."]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Include or exclude packet address field out of CRC calculation."]
    #[inline(always)]
    pub fn skipaddr(&self) -> SKIPADDR_R {
        SKIPADDR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC length in number of bytes."]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Bit 8 - Include or exclude packet address field out of CRC calculation."]
    #[inline(always)]
    pub fn skipaddr(&mut self) -> SKIPADDR_W {
        SKIPADDR_W { w: self }
    }
}
