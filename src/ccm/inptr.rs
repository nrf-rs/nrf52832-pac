#[doc = "Reader of register INPTR"]
pub type R = crate::R<u32, super::INPTR>;
#[doc = "Writer for register INPTR"]
pub type W = crate::W<u32, super::INPTR>;
#[doc = "Register INPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INPTR`"]
pub type INPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INPTR`"]
pub struct INPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Input pointer"]
    #[inline(always)]
    pub fn inptr(&self) -> INPTR_R {
        INPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input pointer"]
    #[inline(always)]
    pub fn inptr(&mut self) -> INPTR_W {
        INPTR_W { w: self }
    }
}
