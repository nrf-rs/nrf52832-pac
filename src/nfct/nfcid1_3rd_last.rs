#[doc = "Reader of register NFCID1_3RD_LAST"]
pub type R = crate::R<u32, super::NFCID1_3RD_LAST>;
#[doc = "Writer for register NFCID1_3RD_LAST"]
pub type W = crate::W<u32, super::NFCID1_3RD_LAST>;
#[doc = "Register NFCID1_3RD_LAST `reset()`'s with value 0"]
impl crate::ResetValue for super::NFCID1_3RD_LAST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NFCID1_S`"]
pub type NFCID1_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_S`"]
pub struct NFCID1_S_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NFCID1_R`"]
pub type NFCID1_R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_R`"]
pub struct NFCID1_R_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NFCID1_Q`"]
pub type NFCID1_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_Q`"]
pub struct NFCID1_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn nfcid1_s(&self) -> NFCID1_S_R {
        NFCID1_S_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn nfcid1_r(&self) -> NFCID1_R_R {
        NFCID1_R_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn nfcid1_q(&self) -> NFCID1_Q_R {
        NFCID1_Q_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn nfcid1_s(&mut self) -> NFCID1_S_W {
        NFCID1_S_W { w: self }
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn nfcid1_r(&mut self) -> NFCID1_R_W {
        NFCID1_R_W { w: self }
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn nfcid1_q(&mut self) -> NFCID1_Q_W {
        NFCID1_Q_W { w: self }
    }
}
