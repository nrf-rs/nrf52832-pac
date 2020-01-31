#[doc = "Reader of register CNT"]
pub type R = crate::R<u32, super::CNT>;
#[doc = "Writer for register CNT"]
pub type W = crate::W<u32, super::CNT>;
#[doc = "Register CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Amount of values (duty cycles) in this sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CNT_A {
    #[doc = "0: Sequence is disabled, and shall not be started as it is empty"]
    DISABLED = 0,
}
impl From<CNT_A> for u16 {
    #[inline(always)]
    fn from(variant: CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u16, CNT_A>;
impl CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CNT_A::DISABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CNT_A::DISABLED
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
    #[doc = "Sequence is disabled, and shall not be started as it is empty"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNT_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Amount of values (duty cycles) in this sequence"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Amount of values (duty cycles) in this sequence"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
}
