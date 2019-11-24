#[doc = "Reader of register EEP"]
pub type R = crate::R<u32, super::EEP>;
#[doc = "Writer for register EEP"]
pub type W = crate::W<u32, super::EEP>;
#[doc = "Register EEP `reset()`'s with value 0"]
impl crate::ResetValue for super::EEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EEP`"]
pub type EEP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EEP`"]
pub struct EEP_W<'a> {
    w: &'a mut W,
}
impl<'a> EEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pointer to event register. Accepts only addresses to registers from the Event group."]
    #[inline(always)]
    pub fn eep(&self) -> EEP_R {
        EEP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to event register. Accepts only addresses to registers from the Event group."]
    #[inline(always)]
    pub fn eep(&mut self) -> EEP_W {
        EEP_W { w: self }
    }
}
