#[doc = "Reader of register GPREGRET2"]
pub type R = crate::R<u32, super::GPREGRET2>;
#[doc = "Writer for register GPREGRET2"]
pub type W = crate::W<u32, super::GPREGRET2>;
#[doc = "Register GPREGRET2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPREGRET2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPREGRET`"]
pub type GPREGRET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPREGRET`"]
pub struct GPREGRET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPREGRET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - General purpose retention register"]
    #[inline(always)]
    pub fn gpregret(&self) -> GPREGRET_R {
        GPREGRET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - General purpose retention register"]
    #[inline(always)]
    pub fn gpregret(&mut self) -> GPREGRET_W {
        GPREGRET_W { w: self }
    }
}
