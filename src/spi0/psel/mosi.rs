#[doc = "Reader of register MOSI"]
pub type R = crate::R<u32, super::MOSI>;
#[doc = "Writer for register MOSI"]
pub type W = crate::W<u32, super::MOSI>;
#[doc = "Register MOSI `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MOSI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for SPI MOSI signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELMOSI_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELMOSI_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELMOSI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELMOSI`"]
pub type PSELMOSI_R = crate::R<u32, PSELMOSI_A>;
impl PSELMOSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELMOSI_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELMOSI_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELMOSI_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELMOSI`"]
pub struct PSELMOSI_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELMOSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELMOSI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELMOSI_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MOSI signal"]
    #[inline(always)]
    pub fn pselmosi(&self) -> PSELMOSI_R {
        PSELMOSI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MOSI signal"]
    #[inline(always)]
    pub fn pselmosi(&mut self) -> PSELMOSI_W {
        PSELMOSI_W { w: self }
    }
}
