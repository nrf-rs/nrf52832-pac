#[doc = "Writer for register TASKS_STOPRX"]
pub type W = crate::W<u32, super::TASKS_STOPRX>;
#[doc = "Register TASKS_STOPRX `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_STOPRX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TASKS_STOPRX`"]
pub struct TASKS_STOPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_STOPRX_W<'a> {
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tasks_stoprx(&mut self) -> TASKS_STOPRX_W {
        TASKS_STOPRX_W { w: self }
    }
}
