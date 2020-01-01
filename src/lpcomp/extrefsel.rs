#[doc = "Reader of register EXTREFSEL"]
pub type R = crate::R<u32, super::EXTREFSEL>;
#[doc = "Writer for register EXTREFSEL"]
pub type W = crate::W<u32, super::EXTREFSEL>;
#[doc = "Register EXTREFSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTREFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External analog reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTREFSEL_A {
    #[doc = "0: Use AIN0 as external analog reference"]
    ANALOGREFERENCE0 = 0,
    #[doc = "1: Use AIN1 as external analog reference"]
    ANALOGREFERENCE1 = 1,
}
impl From<EXTREFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: EXTREFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTREFSEL`"]
pub type EXTREFSEL_R = crate::R<bool, EXTREFSEL_A>;
impl EXTREFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTREFSEL_A {
        match self.bits {
            false => EXTREFSEL_A::ANALOGREFERENCE0,
            true => EXTREFSEL_A::ANALOGREFERENCE1,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE0`"]
    #[inline(always)]
    pub fn is_analog_reference0(&self) -> bool {
        *self == EXTREFSEL_A::ANALOGREFERENCE0
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE1`"]
    #[inline(always)]
    pub fn is_analog_reference1(&self) -> bool {
        *self == EXTREFSEL_A::ANALOGREFERENCE1
    }
}
#[doc = "Write proxy for field `EXTREFSEL`"]
pub struct EXTREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTREFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTREFSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use AIN0 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE0)
    }
    #[doc = "Use AIN1 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE1)
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
    #[doc = "Bit 0 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&self) -> EXTREFSEL_R {
        EXTREFSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&mut self) -> EXTREFSEL_W {
        EXTREFSEL_W { w: self }
    }
}
