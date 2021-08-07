#[doc = "Reader of register EVENTS_FIELDDETECTED"]
pub type R = crate::R<u32, super::EVENTS_FIELDDETECTED>;
#[doc = "Writer for register EVENTS_FIELDDETECTED"]
pub type W = crate::W<u32, super::EVENTS_FIELDDETECTED>;
#[doc = "Register EVENTS_FIELDDETECTED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_FIELDDETECTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_FIELDDETECTED`"]
pub type EVENTS_FIELDDETECTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_FIELDDETECTED`"]
pub struct EVENTS_FIELDDETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_FIELDDETECTED_W<'a> {
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
    pub fn events_fielddetected(&self) -> EVENTS_FIELDDETECTED_R {
        EVENTS_FIELDDETECTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_fielddetected(&mut self) -> EVENTS_FIELDDETECTED_W {
        EVENTS_FIELDDETECTED_W { w: self }
    }
}
