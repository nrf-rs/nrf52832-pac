#[doc = "Reader of register PSELRTS"]
pub type R = crate::R<u32, super::PSELRTS>;
#[doc = "Writer for register PSELRTS"]
pub type W = crate::W<u32, super::PSELRTS>;
#[doc = "Register PSELRTS `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELRTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for UART RTS signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELRTS_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELRTS_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELRTS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELRTS`"]
pub type PSELRTS_R = crate::R<u32, PSELRTS_A>;
impl PSELRTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELRTS_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELRTS_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELRTS_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELRTS`"]
pub struct PSELRTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELRTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELRTS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELRTS_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART RTS signal"]
    #[inline(always)]
    pub fn pselrts(&self) -> PSELRTS_R {
        PSELRTS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART RTS signal"]
    #[inline(always)]
    pub fn pselrts(&mut self) -> PSELRTS_W {
        PSELRTS_W { w: self }
    }
}
