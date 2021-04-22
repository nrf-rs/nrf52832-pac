#[doc = "Reader of register EVENTS_TXSTARTED"]
pub type R = crate::R<u32, super::EVENTS_TXSTARTED>;
#[doc = "Writer for register EVENTS_TXSTARTED"]
pub type W = crate::W<u32, super::EVENTS_TXSTARTED>;
#[doc = "Register EVENTS_TXSTARTED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_TXSTARTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_TXSTARTED`"]
pub type EVENTS_TXSTARTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_TXSTARTED`"]
pub struct EVENTS_TXSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_TXSTARTED_W<'a> {
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
    pub fn events_txstarted(&self) -> EVENTS_TXSTARTED_R {
        EVENTS_TXSTARTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_txstarted(&mut self) -> EVENTS_TXSTARTED_W {
        EVENTS_TXSTARTED_W { w: self }
    }
}
