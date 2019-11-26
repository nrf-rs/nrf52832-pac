#[doc = "Reader of register BASE1"]
pub type R = crate::R<u32, super::BASE1>;
#[doc = "Writer for register BASE1"]
pub type W = crate::W<u32, super::BASE1>;
#[doc = "Register BASE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BASE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASE1`"]
pub type BASE1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASE1`"]
pub struct BASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address 1"]
    #[inline(always)]
    pub fn base1(&self) -> BASE1_R {
        BASE1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address 1"]
    #[inline(always)]
    pub fn base1(&mut self) -> BASE1_W {
        BASE1_W { w: self }
    }
}
