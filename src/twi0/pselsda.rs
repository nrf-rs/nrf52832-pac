#[doc = "Reader of register PSELSDA"]
pub type R = crate::R<u32, super::PSELSDA>;
#[doc = "Writer for register PSELSDA"]
pub type W = crate::W<u32, super::PSELSDA>;
#[doc = "Register PSELSDA `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELSDA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for TWI SDA signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELSDA_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELSDA_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELSDA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELSDA`"]
pub type PSELSDA_R = crate::R<u32, PSELSDA_A>;
impl PSELSDA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELSDA_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELSDA_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELSDA_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELSDA`"]
pub struct PSELSDA_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELSDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELSDA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELSDA_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SDA signal"]
    #[inline(always)]
    pub fn pselsda(&self) -> PSELSDA_R {
        PSELSDA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SDA signal"]
    #[inline(always)]
    pub fn pselsda(&mut self) -> PSELSDA_W {
        PSELSDA_W { w: self }
    }
}
