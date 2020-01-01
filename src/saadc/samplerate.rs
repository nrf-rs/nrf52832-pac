#[doc = "Reader of register SAMPLERATE"]
pub type R = crate::R<u32, super::SAMPLERATE>;
#[doc = "Writer for register SAMPLERATE"]
pub type W = crate::W<u32, super::SAMPLERATE>;
#[doc = "Register SAMPLERATE `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMPLERATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CC`"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Select mode for sample rate control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Rate is controlled from SAMPLE task"]
    TASK = 0,
    #[doc = "1: Rate is controlled from local timer (use CC to control the rate)"]
    TIMERS = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::TASK,
            true => MODE_A::TIMERS,
        }
    }
    #[doc = "Checks if the value of the field is `TASK`"]
    #[inline(always)]
    pub fn is_task(&self) -> bool {
        *self == MODE_A::TASK
    }
    #[doc = "Checks if the value of the field is `TIMERS`"]
    #[inline(always)]
    pub fn is_timers(&self) -> bool {
        *self == MODE_A::TIMERS
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
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rate is controlled from SAMPLE task"]
    #[inline(always)]
    pub fn task(self) -> &'a mut W {
        self.variant(MODE_A::TASK)
    }
    #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
    #[inline(always)]
    pub fn timers(self) -> &'a mut W {
        self.variant(MODE_A::TIMERS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Capture and compare value. Sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - Select mode for sample rate control"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Capture and compare value. Sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Bit 12 - Select mode for sample rate control"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
