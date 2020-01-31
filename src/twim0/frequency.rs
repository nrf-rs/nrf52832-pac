#[doc = "Reader of register FREQUENCY"]
pub type R = crate::R<u32, super::FREQUENCY>;
#[doc = "Writer for register FREQUENCY"]
pub type W = crate::W<u32, super::FREQUENCY>;
#[doc = "Register FREQUENCY `reset()`'s with value 0x0400_0000"]
impl crate::ResetValue for super::FREQUENCY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400_0000
    }
}
#[doc = "TWI master clock frequency\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FREQUENCY_A {
    #[doc = "26738688: 100 kbps"]
    K100 = 26738688,
    #[doc = "67108864: 250 kbps"]
    K250 = 67108864,
    #[doc = "104857600: 400 kbps"]
    K400 = 104857600,
}
impl From<FREQUENCY_A> for u32 {
    #[inline(always)]
    fn from(variant: FREQUENCY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FREQUENCY`"]
pub type FREQUENCY_R = crate::R<u32, FREQUENCY_A>;
impl FREQUENCY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FREQUENCY_A> {
        use crate::Variant::*;
        match self.bits {
            26738688 => Val(FREQUENCY_A::K100),
            67108864 => Val(FREQUENCY_A::K250),
            104857600 => Val(FREQUENCY_A::K400),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K100`"]
    #[inline(always)]
    pub fn is_k100(&self) -> bool {
        *self == FREQUENCY_A::K100
    }
    #[doc = "Checks if the value of the field is `K250`"]
    #[inline(always)]
    pub fn is_k250(&self) -> bool {
        *self == FREQUENCY_A::K250
    }
    #[doc = "Checks if the value of the field is `K400`"]
    #[inline(always)]
    pub fn is_k400(&self) -> bool {
        *self == FREQUENCY_A::K400
    }
}
#[doc = "Write proxy for field `FREQUENCY`"]
pub struct FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQUENCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQUENCY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "100 kbps"]
    #[inline(always)]
    pub fn k100(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K100)
    }
    #[doc = "250 kbps"]
    #[inline(always)]
    pub fn k250(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K250)
    }
    #[doc = "400 kbps"]
    #[inline(always)]
    pub fn k400(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K400)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TWI master clock frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TWI master clock frequency"]
    #[inline(always)]
    pub fn frequency(&mut self) -> FREQUENCY_W {
        FREQUENCY_W { w: self }
    }
}
