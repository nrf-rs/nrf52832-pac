#[doc = "Reader of register CUSTOMER[%s]"]
pub type R = crate::R<u32, super::CUSTOMER>;
#[doc = "Writer for register CUSTOMER[%s]"]
pub type W = crate::W<u32, super::CUSTOMER>;
#[doc = "Register CUSTOMER[%s]
`reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CUSTOMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `CUSTOMER`"]
pub type CUSTOMER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CUSTOMER`"]
pub struct CUSTOMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CUSTOMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reserved for customer"]
    #[inline(always)]
    pub fn customer(&self) -> CUSTOMER_R {
        CUSTOMER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for customer"]
    #[inline(always)]
    pub fn customer(&mut self) -> CUSTOMER_W {
        CUSTOMER_W { w: self }
    }
}
