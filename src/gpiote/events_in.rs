#[doc = "Reader of register EVENTS_IN[%s]"]
pub type R = crate::R<u32, super::EVENTS_IN>;
#[doc = "Writer for register EVENTS_IN[%s]"]
pub type W = crate::W<u32, super::EVENTS_IN>;
#[doc = "Register EVENTS_IN[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_IN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_IN`"]
pub type EVENTS_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_IN`"]
pub struct EVENTS_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_IN_W<'a> {
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
    pub fn events_in(&self) -> EVENTS_IN_R {
        EVENTS_IN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_in(&mut self) -> EVENTS_IN_W {
        EVENTS_IN_W { w: self }
    }
}
