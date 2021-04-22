#[doc = "Reader of register EVENTS_PORT"]
pub type R = crate::R<u32, super::EVENTS_PORT>;
#[doc = "Writer for register EVENTS_PORT"]
pub type W = crate::W<u32, super::EVENTS_PORT>;
#[doc = "Register EVENTS_PORT `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_PORT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_PORT`"]
pub type EVENTS_PORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_PORT`"]
pub struct EVENTS_PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_PORT_W<'a> {
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
    pub fn events_port(&self) -> EVENTS_PORT_R {
        EVENTS_PORT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_port(&mut self) -> EVENTS_PORT_W {
        EVENTS_PORT_W { w: self }
    }
}
