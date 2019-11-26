#[doc = "Reader of register IMISS"]
pub type R = crate::R<u32, super::IMISS>;
#[doc = "Writer for register IMISS"]
pub type W = crate::W<u32, super::IMISS>;
#[doc = "Register IMISS `reset()`'s with value 0"]
impl crate::ResetValue for super::IMISS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MISSES`"]
pub type MISSES_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MISSES`"]
pub struct MISSES_W<'a> {
    w: &'a mut W,
}
impl<'a> MISSES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of cache misses"]
    #[inline(always)]
    pub fn misses(&self) -> MISSES_R {
        MISSES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of cache misses"]
    #[inline(always)]
    pub fn misses(&mut self) -> MISSES_W {
        MISSES_W { w: self }
    }
}
