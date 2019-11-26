#[doc = "Reader of register END"]
pub type R = crate::R<u32, super::END>;
#[doc = "Writer for register END"]
pub type W = crate::W<u32, super::END>;
#[doc = "Register END `reset()`'s with value 0"]
impl crate::ResetValue for super::END {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `END`"]
pub type END_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `END`"]
pub struct END_W<'a> {
    w: &'a mut W,
}
impl<'a> END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - End address of region."]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - End address of region."]
    #[inline(always)]
    pub fn end(&mut self) -> END_W {
        END_W { w: self }
    }
}
