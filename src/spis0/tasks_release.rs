#[doc = "Writer for register TASKS_RELEASE"]
pub type W = crate::W<u32, super::TASKS_RELEASE>;
#[doc = "Register TASKS_RELEASE `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_RELEASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TASKS_RELEASE`"]
pub struct TASKS_RELEASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_RELEASE_W<'a> {
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
    pub fn tasks_release(&mut self) -> TASKS_RELEASE_W {
        TASKS_RELEASE_W { w: self }
    }
}
