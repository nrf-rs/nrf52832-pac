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
#[doc = "Selects up or up and down as wave counter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDOWN_A {
    #[doc = "0: Up counter - edge aligned PWM duty-cycle"]
    UP = 0,
    #[doc = "1: Up and down counter - center aligned PWM duty cycle"]
    UPANDDOWN = 1,
}
impl From<UPDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: UPDOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UPDOWN`"]
pub type UPDOWN_R = crate::R<bool, UPDOWN_A>;
impl UPDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDOWN_A {
        match self.bits {
            false => UPDOWN_A::UP,
            true => UPDOWN_A::UPANDDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == UPDOWN_A::UP
    }
    #[doc = "Checks if the value of the field is `UPANDDOWN`"]
    #[inline(always)]
    pub fn is_up_and_down(&self) -> bool {
        *self == UPDOWN_A::UPANDDOWN
    }
}
#[doc = "Write proxy for field `UPDOWN`"]
pub struct UPDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Up counter - edge aligned PWM duty-cycle"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(UPDOWN_A::UP)
    }
    #[doc = "Up and down counter - center aligned PWM duty cycle"]
    #[inline(always)]
    pub fn up_and_down(self) -> &'a mut W {
        self.variant(UPDOWN_A::UPANDDOWN)
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
impl R {
    #[doc = "Bit 0 - Selects up or up and down as wave counter mode"]
    #[inline(always)]
    pub fn updown(&self) -> UPDOWN_R {
        UPDOWN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects up or up and down as wave counter mode"]
    #[inline(always)]
    pub fn updown(&mut self) -> UPDOWN_W {
        UPDOWN_W { w: self }
    }
}
