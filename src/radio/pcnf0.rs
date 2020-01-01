#[doc = "Reader of register PCNF0"]
pub type R = crate::R<u32, super::PCNF0>;
#[doc = "Writer for register PCNF0"]
pub type W = crate::W<u32, super::PCNF0>;
#[doc = "Register PCNF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LFLEN`"]
pub type LFLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LFLEN`"]
pub struct LFLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `S0LEN`"]
pub type S0LEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0LEN`"]
pub struct S0LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S0LEN_W<'a> {
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
#[doc = "Reader of field `S1LEN`"]
pub type S1LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S1LEN`"]
pub struct S1LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S1LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Include or exclude S1 field in RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1INCL_A {
    #[doc = "0: Include S1 field in RAM only if S1LEN &gt; 0"]
    AUTOMATIC = 0,
    #[doc = "1: Always include S1 field in RAM independent of S1LEN"]
    INCLUDE = 1,
}
impl From<S1INCL_A> for bool {
    #[inline(always)]
    fn from(variant: S1INCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S1INCL`"]
pub type S1INCL_R = crate::R<bool, S1INCL_A>;
impl S1INCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1INCL_A {
        match self.bits {
            false => S1INCL_A::AUTOMATIC,
            true => S1INCL_A::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == S1INCL_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == S1INCL_A::INCLUDE
    }
}
#[doc = "Write proxy for field `S1INCL`"]
pub struct S1INCL_W<'a> {
    w: &'a mut W,
}
impl<'a> S1INCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1INCL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(S1INCL_A::AUTOMATIC)
    }
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    #[inline(always)]
    pub fn include(self) -> &'a mut W {
        self.variant(S1INCL_A::INCLUDE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Length of preamble on air. Decision point: TASKS_START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLEN_A {
    #[doc = "0: 8-bit preamble"]
    _8BIT = 0,
    #[doc = "1: 16-bit preamble"]
    _16BIT = 1,
}
impl From<PLEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLEN`"]
pub type PLEN_R = crate::R<bool, PLEN_A>;
impl PLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLEN_A {
        match self.bits {
            false => PLEN_A::_8BIT,
            true => PLEN_A::_16BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == PLEN_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == PLEN_A::_16BIT
    }
}
#[doc = "Write proxy for field `PLEN`"]
pub struct PLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "8-bit preamble"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(PLEN_A::_8BIT)
    }
    #[doc = "16-bit preamble"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(PLEN_A::_16BIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline(always)]
    pub fn lflen(&self) -> LFLEN_R {
        LFLEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline(always)]
    pub fn s0len(&self) -> S0LEN_R {
        S0LEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline(always)]
    pub fn s1len(&self) -> S1LEN_R {
        S1LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn s1incl(&self) -> S1INCL_R {
        S1INCL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn plen(&self) -> PLEN_R {
        PLEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline(always)]
    pub fn lflen(&mut self) -> LFLEN_W {
        LFLEN_W { w: self }
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline(always)]
    pub fn s0len(&mut self) -> S0LEN_W {
        S0LEN_W { w: self }
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline(always)]
    pub fn s1len(&mut self) -> S1LEN_W {
        S1LEN_W { w: self }
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn s1incl(&mut self) -> S1INCL_W {
        S1INCL_W { w: self }
    }
    #[doc = "Bit 24 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn plen(&mut self) -> PLEN_W {
        PLEN_W { w: self }
    }
}
