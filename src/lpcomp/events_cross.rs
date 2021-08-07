#[doc = "Reader of register EVENTS_CROSS"]
pub type R = crate::R<u32, super::EVENTS_CROSS>;
#[doc = "Writer for register EVENTS_CROSS"]
pub type W = crate::W<u32, super::EVENTS_CROSS>;
#[doc = "Register EVENTS_CROSS `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_CROSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_CROSS`"]
pub type EVENTS_CROSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_CROSS`"]
pub struct EVENTS_CROSS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_CROSS_W<'a> {
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
    pub fn events_cross(&self) -> EVENTS_CROSS_R {
        EVENTS_CROSS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_cross(&mut self) -> EVENTS_CROSS_W {
        EVENTS_CROSS_W { w: self }
    }
}
