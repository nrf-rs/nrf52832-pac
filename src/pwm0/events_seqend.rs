#[doc = "Reader of register EVENTS_SEQEND[%s]"]
pub type R = crate::R<u32, super::EVENTS_SEQEND>;
#[doc = "Writer for register EVENTS_SEQEND[%s]"]
pub type W = crate::W<u32, super::EVENTS_SEQEND>;
#[doc = "Register EVENTS_SEQEND[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_SEQEND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_SEQEND`"]
pub type EVENTS_SEQEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_SEQEND`"]
pub struct EVENTS_SEQEND_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_SEQEND_W<'a> {
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
    pub fn events_seqend(&self) -> EVENTS_SEQEND_R {
        EVENTS_SEQEND_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_seqend(&mut self) -> EVENTS_SEQEND_W {
        EVENTS_SEQEND_W { w: self }
    }
}
