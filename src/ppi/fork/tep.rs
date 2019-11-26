#[doc = "Reader of register TEP"]
pub type R = crate::R<u32, super::TEP>;
#[doc = "Writer for register TEP"]
pub type W = crate::W<u32, super::TEP>;
#[doc = "Register TEP `reset()`'s with value 0"]
impl crate::ResetValue for super::TEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEP`"]
pub type TEP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TEP`"]
pub struct TEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pointer to task register"]
    #[inline(always)]
    pub fn tep(&self) -> TEP_R {
        TEP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to task register"]
    #[inline(always)]
    pub fn tep(&mut self) -> TEP_W {
        TEP_W { w: self }
    }
}
