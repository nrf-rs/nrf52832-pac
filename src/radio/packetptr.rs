#[doc = "Reader of register PACKETPTR"]
pub type R = crate::R<u32, super::PACKETPTR>;
#[doc = "Writer for register PACKETPTR"]
pub type W = crate::W<u32, super::PACKETPTR>;
#[doc = "Register PACKETPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::PACKETPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PACKETPTR`"]
pub type PACKETPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PACKETPTR`"]
pub struct PACKETPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKETPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Packet pointer"]
    #[inline(always)]
    pub fn packetptr(&self) -> PACKETPTR_R {
        PACKETPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Packet pointer"]
    #[inline(always)]
    pub fn packetptr(&mut self) -> PACKETPTR_W {
        PACKETPTR_W { w: self }
    }
}
