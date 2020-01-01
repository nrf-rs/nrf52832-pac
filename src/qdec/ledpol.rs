#[doc = "Reader of register LEDPOL"]
pub type R = crate::R<u32, super::LEDPOL>;
#[doc = "Writer for register LEDPOL"]
pub type W = crate::W<u32, super::LEDPOL>;
#[doc = "Register LEDPOL `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDPOL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LED output pin polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDPOL_A {
    #[doc = "0: Led active on output pin low"]
    ACTIVELOW = 0,
    #[doc = "1: Led active on output pin high"]
    ACTIVEHIGH = 1,
}
impl From<LEDPOL_A> for bool {
    #[inline(always)]
    fn from(variant: LEDPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEDPOL`"]
pub type LEDPOL_R = crate::R<bool, LEDPOL_A>;
impl LEDPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDPOL_A {
        match self.bits {
            false => LEDPOL_A::ACTIVELOW,
            true => LEDPOL_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == LEDPOL_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == LEDPOL_A::ACTIVEHIGH
    }
}
#[doc = "Write proxy for field `LEDPOL`"]
pub struct LEDPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEDPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Led active on output pin low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(LEDPOL_A::ACTIVELOW)
    }
    #[doc = "Led active on output pin high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(LEDPOL_A::ACTIVEHIGH)
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
    #[doc = "Bit 0 - LED output pin polarity"]
    #[inline(always)]
    pub fn ledpol(&self) -> LEDPOL_R {
        LEDPOL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED output pin polarity"]
    #[inline(always)]
    pub fn ledpol(&mut self) -> LEDPOL_W {
        LEDPOL_W { w: self }
    }
}
