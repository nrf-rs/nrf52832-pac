#[doc = "Reader of register PSELTXD"]
pub type R = crate::R<u32, super::PSELTXD>;
#[doc = "Writer for register PSELTXD"]
pub type W = crate::W<u32, super::PSELTXD>;
#[doc = "Register PSELTXD `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELTXD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Pin number configuration for UART TXD signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PSELTXD_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELTXD_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELTXD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELTXD`"]
pub type PSELTXD_R = crate::R<u32, PSELTXD_A>;
impl PSELTXD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PSELTXD_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PSELTXD_A::DISCONNECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELTXD_A::DISCONNECTED
    }
}
#[doc = "Write proxy for field `PSELTXD`"]
pub struct PSELTXD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELTXD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELTXD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELTXD_A::DISCONNECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART TXD signal"]
    #[inline(always)]
    pub fn pseltxd(&self) -> PSELTXD_R {
        PSELTXD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART TXD signal"]
    #[inline(always)]
    pub fn pseltxd(&mut self) -> PSELTXD_W {
        PSELTXD_W { w: self }
    }
}
