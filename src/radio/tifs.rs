#[doc = "Reader of register TIFS"]
pub type R = crate::R<u32, super::TIFS>;
#[doc = "Writer for register TIFS"]
pub type W = crate::W<u32, super::TIFS>;
#[doc = "Register TIFS `reset()`'s with value 0"]
impl crate::ResetValue for super::TIFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIFS`"]
pub type TIFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIFS`"]
pub struct TIFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Inter Frame Spacing in us"]
    #[inline(always)]
    pub fn tifs(&self) -> TIFS_R {
        TIFS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Inter Frame Spacing in us"]
    #[inline(always)]
    pub fn tifs(&mut self) -> TIFS_W {
        TIFS_W { w: self }
    }
}
