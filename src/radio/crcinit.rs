#[doc = "Reader of register CRCINIT"]
pub type R = crate::R<u32, super::CRCINIT>;
#[doc = "Writer for register CRCINIT"]
pub type W = crate::W<u32, super::CRCINIT>;
#[doc = "Register CRCINIT `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCINIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRCINIT`"]
pub type CRCINIT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRCINIT`"]
pub struct CRCINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCINIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - CRC initial value"]
    #[inline(always)]
    pub fn crcinit(&self) -> CRCINIT_R {
        CRCINIT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - CRC initial value"]
    #[inline(always)]
    pub fn crcinit(&mut self) -> CRCINIT_W {
        CRCINIT_W { w: self }
    }
}
