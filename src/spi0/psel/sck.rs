#[doc = "Reader of register SCK"]
pub type R = crate::R<u32, super::SCK>;
#[doc = "Writer for register SCK"]
pub type W = crate::W<u32, super::SCK>;
#[doc = "Register SCK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::SCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for SPI SCK signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELSCK_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELSCK_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELSCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELSCK`"]
pub type PSELSCK_R = crate::R<u32, PSELSCK_A>;
impl PSELSCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELSCK_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELSCK_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELSCK_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELSCK`"]
pub struct PSELSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELSCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELSCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELSCK_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI SCK signal"]
    #[inline(always)]
    pub fn pselsck(&self) -> PSELSCK_R {
        PSELSCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI SCK signal"]
    #[inline(always)]
    pub fn pselsck(&mut self) -> PSELSCK_W {
        PSELSCK_W { w: self }
    }
}
