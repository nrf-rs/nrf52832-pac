#[doc = "Reader of register PSELCTS"]
pub type R = crate::R<u32, super::PSELCTS>;
#[doc = "Writer for register PSELCTS"]
pub type W = crate::W<u32, super::PSELCTS>;
#[doc = "Register PSELCTS `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELCTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for UART CTS signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELCTS_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELCTS_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELCTS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELCTS`"]
pub type PSELCTS_R = crate::R<u32, PSELCTS_A>;
impl PSELCTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELCTS_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELCTS_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELCTS_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELCTS`"]
pub struct PSELCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELCTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELCTS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELCTS_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART CTS signal"]
    #[inline(always)]
    pub fn pselcts(&self) -> PSELCTS_R {
        PSELCTS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART CTS signal"]
    #[inline(always)]
    pub fn pselcts(&mut self) -> PSELCTS_W {
        PSELCTS_W { w: self }
    }
}
