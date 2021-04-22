#[doc = "Reader of register EVENTS_READ"]
pub type R = crate::R<u32, super::EVENTS_READ>;
#[doc = "Writer for register EVENTS_READ"]
pub type W = crate::W<u32, super::EVENTS_READ>;
#[doc = "Register EVENTS_READ `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_READ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_READ`"]
pub type EVENTS_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_READ`"]
pub struct EVENTS_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_READ_W<'a> {
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
    pub fn events_read(&self) -> EVENTS_READ_R {
        EVENTS_READ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_read(&mut self) -> EVENTS_READ_W {
        EVENTS_READ_W { w: self }
    }
}
