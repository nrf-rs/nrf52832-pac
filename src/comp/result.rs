#[doc = "Reader of register RESULT"]
pub type R = crate::R<u32, super::RESULT>;
#[doc = "Result of last compare. Decision point SAMPLE task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULT_A {
    #[doc = "0: Input voltage is below the threshold (VIN+ &lt; VIN-)"]
    BELOW = 0,
    #[doc = "1: Input voltage is above the threshold (VIN+ &gt; VIN-)"]
    ABOVE = 1,
}
impl From<RESULT_A> for bool {
    #[inline(always)]
    fn from(variant: RESULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<bool, RESULT_A>;
impl RESULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESULT_A {
        match self.bits {
            false => RESULT_A::BELOW,
            true => RESULT_A::ABOVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RESULT_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RESULT_A::ABOVE
    }
}
impl R {
    #[doc = "Bit 0 - Result of last compare. Decision point SAMPLE task."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x01) != 0)
    }
}
