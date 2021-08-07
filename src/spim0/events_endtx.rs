#[doc = "Reader of register EVENTS_ENDTX"]
pub type R = crate::R<u32, super::EVENTS_ENDTX>;
#[doc = "Writer for register EVENTS_ENDTX"]
pub type W = crate::W<u32, super::EVENTS_ENDTX>;
#[doc = "Register EVENTS_ENDTX `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_ENDTX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_ENDTX`"]
pub type EVENTS_ENDTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_ENDTX`"]
pub struct EVENTS_ENDTX_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_ENDTX_W<'a> {
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
    pub fn events_endtx(&self) -> EVENTS_ENDTX_R {
        EVENTS_ENDTX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_endtx(&mut self) -> EVENTS_ENDTX_W {
        EVENTS_ENDTX_W { w: self }
    }
}
