#[doc = "Reader of register MODE"]
pub type R = crate::R<u32, super::MODE>;
#[doc = "Writer for register MODE"]
pub type W = crate::W<u32, super::MODE>;
#[doc = "Register MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Select Timer mode"]
    TIMER = 0,
    #[doc = "1: Deprecated enumerator -  Select Counter mode"]
    COUNTER = 1,
    #[doc = "2: Select Low Power Counter mode"]
    LOWPOWERCOUNTER = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::TIMER),
            1 => Val(MODE_A::COUNTER),
            2 => Val(MODE_A::LOWPOWERCOUNTER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == MODE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == MODE_A::COUNTER
    }
    #[doc = "Checks if the value of the field is `LOWPOWERCOUNTER`"]
    #[inline(always)]
    pub fn is_low_power_counter(&self) -> bool {
        *self == MODE_A::LOWPOWERCOUNTER
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select Timer mode"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(MODE_A::TIMER)
    }
    #[doc = "Deprecated enumerator - Select Counter mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut W {
        self.variant(MODE_A::COUNTER)
    }
    #[doc = "Select Low Power Counter mode"]
    #[inline(always)]
    pub fn low_power_counter(self) -> &'a mut W {
        self.variant(MODE_A::LOWPOWERCOUNTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
