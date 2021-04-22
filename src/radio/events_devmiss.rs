#[doc = "Reader of register EVENTS_DEVMISS"]
pub type R = crate::R<u32, super::EVENTS_DEVMISS>;
#[doc = "Writer for register EVENTS_DEVMISS"]
pub type W = crate::W<u32, super::EVENTS_DEVMISS>;
#[doc = "Register EVENTS_DEVMISS `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_DEVMISS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_DEVMISS`"]
pub type EVENTS_DEVMISS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_DEVMISS`"]
pub struct EVENTS_DEVMISS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_DEVMISS_W<'a> {
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
    pub fn events_devmiss(&self) -> EVENTS_DEVMISS_R {
        EVENTS_DEVMISS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_devmiss(&mut self) -> EVENTS_DEVMISS_W {
        EVENTS_DEVMISS_W { w: self }
    }
}
