#[doc = "Reader of register DETECTMODE"]
pub type R = crate::R<u32, super::DETECTMODE>;
#[doc = "Writer for register DETECTMODE"]
pub type W = crate::W<u32, super::DETECTMODE>;
#[doc = "Register DETECTMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::DETECTMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select between default DETECT signal behaviour and LDETECT mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECTMODE_A {
    #[doc = "0: DETECT directly connected to PIN DETECT signals"]
    DEFAULT = 0,
    #[doc = "1: Use the latched LDETECT behaviour"]
    LDETECT = 1,
}
impl From<DETECTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DETECTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETECTMODE`"]
pub type DETECTMODE_R = crate::R<bool, DETECTMODE_A>;
impl DETECTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECTMODE_A {
        match self.bits {
            false => DETECTMODE_A::DEFAULT,
            true => DETECTMODE_A::LDETECT,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == DETECTMODE_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LDETECT`"]
    #[inline(always)]
    pub fn is_ldetect(&self) -> bool {
        *self == DETECTMODE_A::LDETECT
    }
}
#[doc = "Write proxy for field `DETECTMODE`"]
pub struct DETECTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DETECTMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(DETECTMODE_A::DEFAULT)
    }
    #[doc = "Use the latched LDETECT behaviour"]
    #[inline(always)]
    pub fn ldetect(self) -> &'a mut W {
        self.variant(DETECTMODE_A::LDETECT)
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
    #[doc = "Bit 0 - Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub fn detectmode(&self) -> DETECTMODE_R {
        DETECTMODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub fn detectmode(&mut self) -> DETECTMODE_W {
        DETECTMODE_W { w: self }
    }
}
