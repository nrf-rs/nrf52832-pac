#[doc = "Reader of register MODE"]
pub type R = crate::R<u32, super::MODE>;
#[doc = "Writer for register MODE"]
pub type W = crate::W<u32, super::MODE>;
#[doc = "Register MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mono or stereo operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPERATION_A {
    #[doc = "0: Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    STEREO = 0,
    #[doc = "1: Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    MONO = 1,
}
impl From<OPERATION_A> for bool {
    #[inline(always)]
    fn from(variant: OPERATION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OPERATION`"]
pub type OPERATION_R = crate::R<bool, OPERATION_A>;
impl OPERATION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPERATION_A {
        match self.bits {
            false => OPERATION_A::STEREO,
            true => OPERATION_A::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == OPERATION_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == OPERATION_A::MONO
    }
}
#[doc = "Write proxy for field `OPERATION`"]
pub struct OPERATION_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERATION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPERATION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(OPERATION_A::STEREO)
    }
    #[doc = "Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(OPERATION_A::MONO)
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
#[doc = "Defines on which PDM_CLK edge Left (or mono) is sampled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE_A {
    #[doc = "0: Left (or mono) is sampled on falling edge of PDM_CLK"]
    LEFTFALLING = 0,
    #[doc = "1: Left (or mono) is sampled on rising edge of PDM_CLK"]
    LEFTRISING = 1,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<bool, EDGE_A>;
impl EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::LEFTFALLING,
            true => EDGE_A::LEFTRISING,
        }
    }
    #[doc = "Checks if the value of the field is `LEFTFALLING`"]
    #[inline(always)]
    pub fn is_left_falling(&self) -> bool {
        *self == EDGE_A::LEFTFALLING
    }
    #[doc = "Checks if the value of the field is `LEFTRISING`"]
    #[inline(always)]
    pub fn is_left_rising(&self) -> bool {
        *self == EDGE_A::LEFTRISING
    }
}
#[doc = "Write proxy for field `EDGE`"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    #[inline(always)]
    pub fn left_falling(self) -> &'a mut W {
        self.variant(EDGE_A::LEFTFALLING)
    }
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    #[inline(always)]
    pub fn left_rising(self) -> &'a mut W {
        self.variant(EDGE_A::LEFTRISING)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mono or stereo operation"]
    #[inline(always)]
    pub fn operation(&self) -> OPERATION_R {
        OPERATION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Defines on which PDM_CLK edge Left (or mono) is sampled"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mono or stereo operation"]
    #[inline(always)]
    pub fn operation(&mut self) -> OPERATION_W {
        OPERATION_W { w: self }
    }
    #[doc = "Bit 1 - Defines on which PDM_CLK edge Left (or mono) is sampled"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W { w: self }
    }
}
