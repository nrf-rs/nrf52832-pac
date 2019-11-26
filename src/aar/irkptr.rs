#[doc = "Reader of register IRKPTR"]
pub type R = crate::R<u32, super::IRKPTR>;
#[doc = "Writer for register IRKPTR"]
pub type W = crate::W<u32, super::IRKPTR>;
#[doc = "Register IRKPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::IRKPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRKPTR`"]
pub type IRKPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IRKPTR`"]
pub struct IRKPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> IRKPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pointer to the IRK data structure"]
    #[inline(always)]
    pub fn irkptr(&self) -> IRKPTR_R {
        IRKPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the IRK data structure"]
    #[inline(always)]
    pub fn irkptr(&mut self) -> IRKPTR_W {
        IRKPTR_W { w: self }
    }
}
