#[doc = "Reader of register APPROTECT"]
pub type R = crate::R<u32, super::APPROTECT>;
#[doc = "Writer for register APPROTECT"]
pub type W = crate::W<u32, super::APPROTECT>;
#[doc = "Register APPROTECT `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::APPROTECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Enable or disable Access Port protection. Any other value than 0xFF being written to this field will enable protection.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PALL_A {
    #[doc = "255: Disable"]
    DISABLED = 255,
    #[doc = "0: Enable"]
    ENABLED = 0,
}
impl From<PALL_A> for u8 {
    #[inline(always)]
    fn from(variant: PALL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PALL`"]
pub type PALL_R = crate::R<u8, PALL_A>;
impl PALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PALL_A> {
        use crate::Variant::*;
        match self.bits {
            255 => Val(PALL_A::DISABLED),
            0 => Val(PALL_A::ENABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PALL_A::ENABLED
    }
}
#[doc = "Write proxy for field `PALL`"]
pub struct PALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PALL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PALL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PALL_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enable or disable Access Port protection. Any other value than 0xFF being written to this field will enable protection."]
    #[inline(always)]
    pub fn pall(&self) -> PALL_R {
        PALL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable or disable Access Port protection. Any other value than 0xFF being written to this field will enable protection."]
    #[inline(always)]
    pub fn pall(&mut self) -> PALL_W {
        PALL_W { w: self }
    }
}
