#[doc = "Reader of register EVENTS_UP"]
pub type R = crate::R<u32, super::EVENTS_UP>;
#[doc = "Writer for register EVENTS_UP"]
pub type W = crate::W<u32, super::EVENTS_UP>;
#[doc = "Register EVENTS_UP `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_UP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_UP`"]
pub type EVENTS_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_UP`"]
pub struct EVENTS_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_UP_W<'a> {
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
    pub fn events_up(&self) -> EVENTS_UP_R {
        EVENTS_UP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_up(&mut self) -> EVENTS_UP_W {
        EVENTS_UP_W { w: self }
    }
}
