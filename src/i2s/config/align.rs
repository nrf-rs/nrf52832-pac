#[doc = "Reader of register ALIGN"]
pub type R = crate::R<u32, super::ALIGN>;
#[doc = "Writer for register ALIGN"]
pub type W = crate::W<u32, super::ALIGN>;
#[doc = "Register ALIGN `reset()`'s with value 0"]
impl crate::ResetValue for super::ALIGN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Alignment of sample within a frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_A {
    #[doc = "0: Left-aligned."]
    LEFT = 0,
    #[doc = "1: Right-aligned."]
    RIGHT = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALIGN`"]
pub type ALIGN_R = crate::R<bool, ALIGN_A>;
impl ALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::LEFT,
            true => ALIGN_A::RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::RIGHT
    }
}
#[doc = "Write proxy for field `ALIGN`"]
pub struct ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Left-aligned."]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::LEFT)
    }
    #[doc = "Right-aligned."]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::RIGHT)
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
impl R {
    #[doc = "Bit 0 - Alignment of sample within a frame."]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alignment of sample within a frame."]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W { w: self }
    }
}
