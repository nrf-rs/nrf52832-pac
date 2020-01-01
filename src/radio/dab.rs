#[doc = "Reader of register DAB[%s]"]
pub type R = crate::R<u32, super::DAB>;
#[doc = "Writer for register DAB[%s]"]
pub type W = crate::W<u32, super::DAB>;
#[doc = "Register DAB[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DAB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAB`"]
pub type DAB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DAB`"]
pub struct DAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DAB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Device address base segment 0"]
    #[inline(always)]
    pub fn dab(&self) -> DAB_R {
        DAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device address base segment 0"]
    #[inline(always)]
    pub fn dab(&mut self) -> DAB_W {
        DAB_W { w: self }
    }
}
