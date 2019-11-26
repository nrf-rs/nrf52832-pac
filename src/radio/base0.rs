#[doc = "Reader of register BASE0"]
pub type R = crate::R<u32, super::BASE0>;
#[doc = "Writer for register BASE0"]
pub type W = crate::W<u32, super::BASE0>;
#[doc = "Register BASE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BASE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASE0`"]
pub type BASE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASE0`"]
pub struct BASE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address 0"]
    #[inline(always)]
    pub fn base0(&self) -> BASE0_R {
        BASE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address 0"]
    #[inline(always)]
    pub fn base0(&mut self) -> BASE0_W {
        BASE0_W { w: self }
    }
}
