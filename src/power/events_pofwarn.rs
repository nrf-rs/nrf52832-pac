#[doc = "Reader of register EVENTS_POFWARN"]
pub type R = crate::R<u32, super::EVENTS_POFWARN>;
#[doc = "Writer for register EVENTS_POFWARN"]
pub type W = crate::W<u32, super::EVENTS_POFWARN>;
#[doc = "Register EVENTS_POFWARN `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_POFWARN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_POFWARN`"]
pub type EVENTS_POFWARN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_POFWARN`"]
pub struct EVENTS_POFWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_POFWARN_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_pofwarn(&self) -> EVENTS_POFWARN_R {
        EVENTS_POFWARN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_pofwarn(&mut self) -> EVENTS_POFWARN_W {
        EVENTS_POFWARN_W { w: self }
    }
}
