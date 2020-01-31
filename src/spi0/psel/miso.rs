#[doc = "Reader of register MISO"]
pub type R = crate::R<u32, super::MISO>;
#[doc = "Writer for register MISO"]
pub type W = crate::W<u32, super::MISO>;
#[doc = "Register MISO `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MISO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for SPI MISO signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELMISO_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELMISO_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELMISO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELMISO`"]
pub type PSELMISO_R = crate::R<u32, PSELMISO_A>;
impl PSELMISO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELMISO_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELMISO_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELMISO_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELMISO`"]
pub struct PSELMISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELMISO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELMISO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELMISO_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MISO signal"]
    #[inline(always)]
    pub fn pselmiso(&self) -> PSELMISO_R {
        PSELMISO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MISO signal"]
    #[inline(always)]
    pub fn pselmiso(&mut self) -> PSELMISO_W {
        PSELMISO_W { w: self }
    }
}
