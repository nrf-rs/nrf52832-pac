#[doc = "Reader of register NFCID1_LAST"]
pub type R = crate::R<u32, super::NFCID1_LAST>;
#[doc = "Writer for register NFCID1_LAST"]
pub type W = crate::W<u32, super::NFCID1_LAST>;
#[doc = "Register NFCID1_LAST `reset()`'s with value 0x6363"]
impl crate::ResetValue for super::NFCID1_LAST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6363
    }
}
#[doc = "Reader of field `NFCID1_Z`"]
pub type NFCID1_Z_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_Z`"]
pub struct NFCID1_Z_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_Z_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NFCID1_Y`"]
pub type NFCID1_Y_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_Y`"]
pub struct NFCID1_Y_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NFCID1_X`"]
pub type NFCID1_X_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_X`"]
pub struct NFCID1_X_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `NFCID1_W`"]
pub type NFCID1_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFCID1_W`"]
pub struct NFCID1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn nfcid1_z(&self) -> NFCID1_Z_R {
        NFCID1_Z_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn nfcid1_y(&self) -> NFCID1_Y_R {
        NFCID1_Y_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn nfcid1_x(&self) -> NFCID1_X_R {
        NFCID1_X_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn nfcid1_w(&self) -> NFCID1_W_R {
        NFCID1_W_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn nfcid1_z(&mut self) -> NFCID1_Z_W {
        NFCID1_Z_W { w: self }
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn nfcid1_y(&mut self) -> NFCID1_Y_W {
        NFCID1_Y_W { w: self }
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn nfcid1_x(&mut self) -> NFCID1_X_W {
        NFCID1_X_W { w: self }
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn nfcid1_w(&mut self) -> NFCID1_W_W {
        NFCID1_W_W { w: self }
    }
}
