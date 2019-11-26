#[doc = "Reader of register BCC"]
pub type R = crate::R<u32, super::BCC>;
#[doc = "Writer for register BCC"]
pub type W = crate::W<u32, super::BCC>;
#[doc = "Register BCC `reset()`'s with value 0"]
impl crate::ResetValue for super::BCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCC`"]
pub type BCC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BCC`"]
pub struct BCC_W<'a> {
    w: &'a mut W,
}
impl<'a> BCC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bit counter compare"]
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bit counter compare"]
    #[inline(always)]
    pub fn bcc(&mut self) -> BCC_W {
        BCC_W { w: self }
    }
}
