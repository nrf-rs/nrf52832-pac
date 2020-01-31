#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is sleeping\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Pause watchdog while the CPU is sleeping"]
    PAUSE = 0,
    #[doc = "1: Keep the watchdog running while the CPU is sleeping"]
    RUN = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, SLEEP_A>;
impl SLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::PAUSE,
            true => SLEEP_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == SLEEP_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == SLEEP_A::RUN
    }
}
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pause watchdog while the CPU is sleeping"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(SLEEP_A::PAUSE)
    }
    #[doc = "Keep the watchdog running while the CPU is sleeping"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(SLEEP_A::RUN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: Pause watchdog while the CPU is halted by the debugger"]
    PAUSE = 0,
    #[doc = "1: Keep the watchdog running while the CPU is halted by the debugger"]
    RUN = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HALT`"]
pub type HALT_R = crate::R<bool, HALT_A>;
impl HALT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::PAUSE,
            true => HALT_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == HALT_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == HALT_A::RUN
    }
}
#[doc = "Write proxy for field `HALT`"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pause watchdog while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(HALT_A::PAUSE)
    }
    #[doc = "Keep the watchdog running while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(HALT_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 3 - Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
}
