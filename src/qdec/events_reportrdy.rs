#[doc = "Reader of register EVENTS_REPORTRDY"]
pub type R = crate::R<u32, super::EVENTS_REPORTRDY>;
#[doc = "Writer for register EVENTS_REPORTRDY"]
pub type W = crate::W<u32, super::EVENTS_REPORTRDY>;
#[doc = "Register EVENTS_REPORTRDY `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_REPORTRDY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_REPORTRDY`"]
pub type EVENTS_REPORTRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_REPORTRDY`"]
pub struct EVENTS_REPORTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_REPORTRDY_W<'a> {
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
    pub fn events_reportrdy(&self) -> EVENTS_REPORTRDY_R {
        EVENTS_REPORTRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_reportrdy(&mut self) -> EVENTS_REPORTRDY_W {
        EVENTS_REPORTRDY_W { w: self }
    }
}
