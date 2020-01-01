#[doc = "Reader of register PSELP"]
pub type R = crate::R<u32, super::PSELP>;
#[doc = "Writer for register PSELP"]
pub type W = crate::W<u32, super::PSELP>;
#[doc = "Register PSELP `reset()`'s with value 0"]
impl crate::ResetValue for super::PSELP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Analog positive input channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSELP_A {
    #[doc = "0: Not connected"]
    NC = 0,
    #[doc = "1: AIN0"]
    ANALOGINPUT0 = 1,
    #[doc = "2: AIN1"]
    ANALOGINPUT1 = 2,
    #[doc = "3: AIN2"]
    ANALOGINPUT2 = 3,
    #[doc = "4: AIN3"]
    ANALOGINPUT3 = 4,
    #[doc = "5: AIN4"]
    ANALOGINPUT4 = 5,
    #[doc = "6: AIN5"]
    ANALOGINPUT5 = 6,
    #[doc = "7: AIN6"]
    ANALOGINPUT6 = 7,
    #[doc = "8: AIN7"]
    ANALOGINPUT7 = 8,
    #[doc = "9: VDD"]
    VDD = 9,
}
impl From<PSELP_A> for u8 {
    #[inline(always)]
    fn from(variant: PSELP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSELP`"]
pub type PSELP_R = crate::R<u8, PSELP_A>;
impl PSELP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PSELP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PSELP_A::NC),
            1 => Val(PSELP_A::ANALOGINPUT0),
            2 => Val(PSELP_A::ANALOGINPUT1),
            3 => Val(PSELP_A::ANALOGINPUT2),
            4 => Val(PSELP_A::ANALOGINPUT3),
            5 => Val(PSELP_A::ANALOGINPUT4),
            6 => Val(PSELP_A::ANALOGINPUT5),
            7 => Val(PSELP_A::ANALOGINPUT6),
            8 => Val(PSELP_A::ANALOGINPUT7),
            9 => Val(PSELP_A::VDD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == PSELP_A::NC
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT0`"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == PSELP_A::ANALOGINPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT1`"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == PSELP_A::ANALOGINPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT2`"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == PSELP_A::ANALOGINPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT3`"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == PSELP_A::ANALOGINPUT3
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT4`"]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        *self == PSELP_A::ANALOGINPUT4
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT5`"]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        *self == PSELP_A::ANALOGINPUT5
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT6`"]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        *self == PSELP_A::ANALOGINPUT6
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT7`"]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        *self == PSELP_A::ANALOGINPUT7
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == PSELP_A::VDD
    }
}
#[doc = "Write proxy for field `PSELP`"]
pub struct PSELP_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(PSELP_A::NC)
    }
    #[doc = "AIN0"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOGINPUT0)
    }
    #[doc = "AIN1"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOGINPUT1)
    }
    #[doc = "AIN2"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOGINPUT2)
    }
    #[doc = "AIN3"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOGINPUT3)
    }
    #[doc = "AIN4"]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOGINPUT4)
    }
    #[doc = "AIN5"]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOGINPUT5)
    }
    #[doc = "AIN6"]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOGINPUT6)
    }
    #[doc = "AIN7"]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOGINPUT7)
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(PSELP_A::VDD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog positive input channel"]
    #[inline(always)]
    pub fn pselp(&self) -> PSELP_R {
        PSELP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog positive input channel"]
    #[inline(always)]
    pub fn pselp(&mut self) -> PSELP_W {
        PSELP_W { w: self }
    }
}
