#[doc = "Reader of register HYST"]
pub type R = crate::R<u32, super::HYST>;
#[doc = "Writer for register HYST"]
pub type W = crate::W<u32, super::HYST>;
#[doc = "Register HYST `reset()`'s with value 0"]
impl crate::ResetValue for super::HYST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "0: Comparator hysteresis disabled"]
    NOHYST = 0,
    #[doc = "1: Comparator hysteresis enabled"]
    HYST50MV = 1,
}
impl From<HYST_A> for bool {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HYST`"]
pub type HYST_R = crate::R<bool, HYST_A>;
impl HYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            false => HYST_A::NOHYST,
            true => HYST_A::HYST50MV,
        }
    }
    #[doc = "Checks if the value of the field is `NOHYST`"]
    #[inline(always)]
    pub fn is_no_hyst(&self) -> bool {
        *self == HYST_A::NOHYST
    }
    #[doc = "Checks if the value of the field is `HYST50MV`"]
    #[inline(always)]
    pub fn is_hyst50m_v(&self) -> bool {
        *self == HYST_A::HYST50MV
    }
}
#[doc = "Write proxy for field `HYST`"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator hysteresis disabled"]
    #[inline(always)]
    pub fn no_hyst(self) -> &'a mut W {
        self.variant(HYST_A::NOHYST)
    }
    #[doc = "Comparator hysteresis enabled"]
    #[inline(always)]
    pub fn hyst50m_v(self) -> &'a mut W {
        self.variant(HYST_A::HYST50MV)
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
    #[doc = "Bit 0 - Comparator hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
}
