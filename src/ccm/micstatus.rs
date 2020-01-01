#[doc = "Reader of register MICSTATUS"]
pub type R = crate::R<u32, super::MICSTATUS>;
#[doc = "The result of the MIC check performed during the previous decryption operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MICSTATUS_A {
    #[doc = "0: MIC check failed"]
    CHECKFAILED = 0,
    #[doc = "1: MIC check passed"]
    CHECKPASSED = 1,
}
impl From<MICSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: MICSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MICSTATUS`"]
pub type MICSTATUS_R = crate::R<bool, MICSTATUS_A>;
impl MICSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MICSTATUS_A {
        match self.bits {
            false => MICSTATUS_A::CHECKFAILED,
            true => MICSTATUS_A::CHECKPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `CHECKFAILED`"]
    #[inline(always)]
    pub fn is_check_failed(&self) -> bool {
        *self == MICSTATUS_A::CHECKFAILED
    }
    #[doc = "Checks if the value of the field is `CHECKPASSED`"]
    #[inline(always)]
    pub fn is_check_passed(&self) -> bool {
        *self == MICSTATUS_A::CHECKPASSED
    }
}
impl R {
    #[doc = "Bit 0 - The result of the MIC check performed during the previous decryption operation"]
    #[inline(always)]
    pub fn micstatus(&self) -> MICSTATUS_R {
        MICSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
