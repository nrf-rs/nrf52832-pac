#[doc = "Reader of register DAP[%s]"]
pub type R = crate::R<u32, super::DAP>;
#[doc = "Writer for register DAP[%s]"]
pub type W = crate::W<u32, super::DAP>;
#[doc = "Register DAP[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAP`"]
pub type DAP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DAP`"]
pub struct DAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Device address prefix 0"]
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device address prefix 0"]
    #[inline(always)]
    pub fn dap(&mut self) -> DAP_W {
        DAP_W { w: self }
    }
}
