#[doc = "Reader of register NIRK"]
pub type R = crate::R<u32, super::NIRK>;
#[doc = "Writer for register NIRK"]
pub type W = crate::W<u32, super::NIRK>;
#[doc = "Register NIRK `reset()`'s with value 0x01"]
impl crate::ResetValue for super::NIRK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `NIRK`"]
pub type NIRK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NIRK`"]
pub struct NIRK_W<'a> {
    w: &'a mut W,
}
impl<'a> NIRK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of Identity root keys available in the IRK data structure"]
    #[inline(always)]
    pub fn nirk(&self) -> NIRK_R {
        NIRK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of Identity root keys available in the IRK data structure"]
    #[inline(always)]
    pub fn nirk(&mut self) -> NIRK_W {
        NIRK_W { w: self }
    }
}
