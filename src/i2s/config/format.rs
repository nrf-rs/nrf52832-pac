#[doc = "Reader of register FORMAT"]
pub type R = crate::R<u32, super::FORMAT>;
#[doc = "Writer for register FORMAT"]
pub type W = crate::W<u32, super::FORMAT>;
#[doc = "Register FORMAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FORMAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Frame format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMAT_A {
    #[doc = "0: Original I2S format."]
    I2S = 0,
    #[doc = "1: Alternate (left- or right-aligned) format."]
    ALIGNED = 1,
}
impl From<FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORMAT`"]
pub type FORMAT_R = crate::R<bool, FORMAT_A>;
impl FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            false => FORMAT_A::I2S,
            true => FORMAT_A::ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == FORMAT_A::I2S
    }
    #[doc = "Checks if the value of the field is `ALIGNED`"]
    #[inline(always)]
    pub fn is_aligned(&self) -> bool {
        *self == FORMAT_A::ALIGNED
    }
}
#[doc = "Write proxy for field `FORMAT`"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORMAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Original I2S format."]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(FORMAT_A::I2S)
    }
    #[doc = "Alternate (left- or right-aligned) format."]
    #[inline(always)]
    pub fn aligned(self) -> &'a mut W {
        self.variant(FORMAT_A::ALIGNED)
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
    #[doc = "Bit 0 - Frame format."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame format."]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
}
