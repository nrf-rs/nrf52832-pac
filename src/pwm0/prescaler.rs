#[doc = "Reader of register PRESCALER"]
pub type R = crate::R<u32, super::PRESCALER>;
#[doc = "Writer for register PRESCALER"]
pub type W = crate::W<u32, super::PRESCALER>;
#[doc = "Register PRESCALER `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESCALER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pre-scaler of PWM_CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: Divide by   1 (16MHz)"]
    DIV_1 = 0,
    #[doc = "1: Divide by   2 ( 8MHz)"]
    DIV_2 = 1,
    #[doc = "2: Divide by   4 ( 4MHz)"]
    DIV_4 = 2,
    #[doc = "3: Divide by   8 ( 2MHz)"]
    DIV_8 = 3,
    #[doc = "4: Divide by  16 ( 1MHz)"]
    DIV_16 = 4,
    #[doc = "5: Divide by  32 ( 500kHz)"]
    DIV_32 = 5,
    #[doc = "6: Divide by  64 ( 250kHz)"]
    DIV_64 = 6,
    #[doc = "7: Divide by 128 ( 125kHz)"]
    DIV_128 = 7,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALER`"]
pub type PRESCALER_R = crate::R<u8, PRESCALER_A>;
impl PRESCALER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER_A {
        match self.bits {
            0 => PRESCALER_A::DIV_1,
            1 => PRESCALER_A::DIV_2,
            2 => PRESCALER_A::DIV_4,
            3 => PRESCALER_A::DIV_8,
            4 => PRESCALER_A::DIV_16,
            5 => PRESCALER_A::DIV_32,
            6 => PRESCALER_A::DIV_64,
            7 => PRESCALER_A::DIV_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == PRESCALER_A::DIV_1
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == PRESCALER_A::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == PRESCALER_A::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == PRESCALER_A::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == PRESCALER_A::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline(always)]
    pub fn is_div_32(&self) -> bool {
        *self == PRESCALER_A::DIV_32
    }
    #[doc = "Checks if the value of the field is `DIV_64`"]
    #[inline(always)]
    pub fn is_div_64(&self) -> bool {
        *self == PRESCALER_A::DIV_64
    }
    #[doc = "Checks if the value of the field is `DIV_128`"]
    #[inline(always)]
    pub fn is_div_128(&self) -> bool {
        *self == PRESCALER_A::DIV_128
    }
}
#[doc = "Write proxy for field `PRESCALER`"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1 (16MHz)"]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_1)
    }
    #[doc = "Divide by 2 ( 8MHz)"]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_2)
    }
    #[doc = "Divide by 4 ( 4MHz)"]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_4)
    }
    #[doc = "Divide by 8 ( 2MHz)"]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_8)
    }
    #[doc = "Divide by 16 ( 1MHz)"]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_16)
    }
    #[doc = "Divide by 32 ( 500kHz)"]
    #[inline(always)]
    pub fn div_32(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_32)
    }
    #[doc = "Divide by 64 ( 250kHz)"]
    #[inline(always)]
    pub fn div_64(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_64)
    }
    #[doc = "Divide by 128 ( 125kHz)"]
    #[inline(always)]
    pub fn div_128(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Pre-scaler of PWM_CLK"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pre-scaler of PWM_CLK"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
}
