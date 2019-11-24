#[doc = "Reader of register NFCID1_2ND_LAST"]
pub type R = crate::R<u32, super::NFCID1_2ND_LAST>;
#[doc = "Writer for register NFCID1_2ND_LAST"]
pub type W = crate::W<u32, super::NFCID1_2ND_LAST>;
#[doc = "Register NFCID1_2ND_LAST `reset()`'s with value 0"]
impl crate::ResetValue for super::NFCID1_2ND_LAST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NFCID1_V`"]
pub type NFCID1_V_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_V`"]
pub struct NFCID1_V_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_V_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NFCID1_U`"]
pub type NFCID1_U_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_U`"]
pub struct NFCID1_U_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NFCID1_T`"]
pub type NFCID1_T_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_T`"]
pub struct NFCID1_T_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte V"]
    #[inline(always)]
    pub fn nfcid1_v(&self) -> NFCID1_V_R {
        NFCID1_V_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte U"]
    #[inline(always)]
    pub fn nfcid1_u(&self) -> NFCID1_U_R {
        NFCID1_U_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte T"]
    #[inline(always)]
    pub fn nfcid1_t(&self) -> NFCID1_T_R {
        NFCID1_T_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte V"]
    #[inline(always)]
    pub fn nfcid1_v(&mut self) -> NFCID1_V_W {
        NFCID1_V_W { w: self }
    }
    #[doc = "Bits 8:15 - NFCID1 byte U"]
    #[inline(always)]
    pub fn nfcid1_u(&mut self) -> NFCID1_U_W {
        NFCID1_U_W { w: self }
    }
    #[doc = "Bits 16:23 - NFCID1 byte T"]
    #[inline(always)]
    pub fn nfcid1_t(&mut self) -> NFCID1_T_W {
        NFCID1_T_W { w: self }
    }
}
