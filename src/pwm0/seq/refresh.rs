#[doc = "Reader of register REFRESH"]
pub type R = crate::R<u32, super::REFRESH>;
#[doc = "Writer for register REFRESH"]
pub type W = crate::W<u32, super::REFRESH>;
#[doc = "Register REFRESH `reset()`'s with value 0x01"]
impl crate::ResetValue for super::REFRESH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CNT_A {
    #[doc = "0: Update every PWM period"]
    CONTINUOUS = 0,
}
impl From<CNT_A> for u32 {
    #[inline(always)]
    fn from(variant: CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u32, CNT_A>;
impl CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CNT_A::CONTINUOUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CNT_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `CNT`"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Update every PWM period"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CNT_A::CONTINUOUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
}
