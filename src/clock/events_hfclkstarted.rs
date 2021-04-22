#[doc = "Reader of register EVENTS_HFCLKSTARTED"]
pub type R = crate::R<u32, super::EVENTS_HFCLKSTARTED>;
#[doc = "Writer for register EVENTS_HFCLKSTARTED"]
pub type W = crate::W<u32, super::EVENTS_HFCLKSTARTED>;
#[doc = "Register EVENTS_HFCLKSTARTED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_HFCLKSTARTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_HFCLKSTARTED`"]
pub type EVENTS_HFCLKSTARTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_HFCLKSTARTED`"]
pub struct EVENTS_HFCLKSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_HFCLKSTARTED_W<'a> {
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
    pub fn events_hfclkstarted(&self) -> EVENTS_HFCLKSTARTED_R {
        EVENTS_HFCLKSTARTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_hfclkstarted(&mut self) -> EVENTS_HFCLKSTARTED_W {
        EVENTS_HFCLKSTARTED_W { w: self }
    }
}
