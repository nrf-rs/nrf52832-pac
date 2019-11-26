#[doc = "Reader of register TH"]
pub type R = crate::R<u32, super::TH>;
#[doc = "Writer for register TH"]
pub type W = crate::W<u32, super::TH>;
#[doc = "Register TH `reset()`'s with value 0"]
impl crate::ResetValue for super::TH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THDOWN`"]
pub type THDOWN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THDOWN`"]
pub struct THDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> THDOWN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `THUP`"]
pub type THUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THUP`"]
pub struct THUP_W<'a> {
    w: &'a mut W,
}
impl<'a> THUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub fn thdown(&self) -> THDOWN_R {
        THDOWN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub fn thup(&self) -> THUP_R {
        THUP_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub fn thdown(&mut self) -> THDOWN_W {
        THDOWN_W { w: self }
    }
    #[doc = "Bits 8:13 - VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub fn thup(&mut self) -> THUP_W {
        THUP_W { w: self }
    }
}
