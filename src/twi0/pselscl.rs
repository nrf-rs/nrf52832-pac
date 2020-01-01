#[doc = "Reader of register PSELSCL"]
pub type R = crate::R<u32, super::PSELSCL>;
#[doc = "Writer for register PSELSCL"]
pub type W = crate::W<u32, super::PSELSCL>;
#[doc = "Register PSELSCL `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELSCL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for TWI SCL signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELSCL_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELSCL_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELSCL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELSCL`"]
pub type PSELSCL_R = crate::R<u32, PSELSCL_A>;
impl PSELSCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELSCL_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELSCL_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELSCL_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELSCL`"]
pub struct PSELSCL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELSCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELSCL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELSCL_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SCL signal"]
    #[inline(always)]
    pub fn pselscl(&self) -> PSELSCL_R {
        PSELSCL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SCL signal"]
    #[inline(always)]
    pub fn pselscl(&mut self) -> PSELSCL_W {
        PSELSCL_W { w: self }
    }
}
