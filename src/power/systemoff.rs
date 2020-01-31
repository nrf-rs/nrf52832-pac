#[doc = "Writer for register SYSTEMOFF"]
pub type W = crate::W<u32, super::SYSTEMOFF>;
#[doc = "Register SYSTEMOFF `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEMOFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable System OFF mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTEMOFF_AW {
    #[doc = "1: Enable System OFF mode"]
    ENTER = 1,
}
impl From<SYSTEMOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSTEMOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SYSTEMOFF`"]
pub struct SYSTEMOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEMOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSTEMOFF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable System OFF mode"]
    #[inline(always)]
    pub fn enter(self) -> &'a mut W {
        self.variant(SYSTEMOFF_AW::ENTER)
    }
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
    #[doc = "Bit 0 - Enable System OFF mode"]
    #[inline(always)]
    pub fn systemoff(&mut self) -> SYSTEMOFF_W {
        SYSTEMOFF_W { w: self }
    }
}
