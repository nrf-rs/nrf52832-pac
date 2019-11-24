#[doc = "Reader of register CNFPTR"]
pub type R = crate::R<u32, super::CNFPTR>;
#[doc = "Writer for register CNFPTR"]
pub type W = crate::W<u32, super::CNFPTR>;
#[doc = "Register CNFPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CNFPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNFPTR`"]
pub type CNFPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CNFPTR`"]
pub struct CNFPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNFPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pointer to the data structure holding the AES key and the CCM NONCE vector (see Table 1 CCM data structure overview)"]
    #[inline(always)]
    pub fn cnfptr(&self) -> CNFPTR_R {
        CNFPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the data structure holding the AES key and the CCM NONCE vector (see Table 1 CCM data structure overview)"]
    #[inline(always)]
    pub fn cnfptr(&mut self) -> CNFPTR_W {
        CNFPTR_W { w: self }
    }
}
