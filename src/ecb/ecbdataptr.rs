#[doc = "Reader of register ECBDATAPTR"]
pub type R = crate::R<u32, super::ECBDATAPTR>;
#[doc = "Writer for register ECBDATAPTR"]
pub type W = crate::W<u32, super::ECBDATAPTR>;
#[doc = "Register ECBDATAPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::ECBDATAPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ECBDATAPTR`"]
pub type ECBDATAPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ECBDATAPTR`"]
pub struct ECBDATAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECBDATAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
    #[inline(always)]
    pub fn ecbdataptr(&self) -> ECBDATAPTR_R {
        ECBDATAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
    #[inline(always)]
    pub fn ecbdataptr(&mut self) -> ECBDATAPTR_W {
        ECBDATAPTR_W { w: self }
    }
}
