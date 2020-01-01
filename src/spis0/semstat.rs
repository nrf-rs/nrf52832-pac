#[doc = "Reader of register SEMSTAT"]
pub type R = crate::R<u32, super::SEMSTAT>;
#[doc = "Semaphore status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEMSTAT_A {
    #[doc = "0: Semaphore is free"]
    FREE = 0,
    #[doc = "1: Semaphore is assigned to CPU"]
    CPU = 1,
    #[doc = "2: Semaphore is assigned to SPI slave"]
    SPIS = 2,
    #[doc = "3: Semaphore is assigned to SPI but a handover to the CPU is pending"]
    CPUPENDING = 3,
}
impl From<SEMSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEMSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEMSTAT`"]
pub type SEMSTAT_R = crate::R<u8, SEMSTAT_A>;
impl SEMSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMSTAT_A {
        match self.bits {
            0 => SEMSTAT_A::FREE,
            1 => SEMSTAT_A::CPU,
            2 => SEMSTAT_A::SPIS,
            3 => SEMSTAT_A::CPUPENDING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == SEMSTAT_A::FREE
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == SEMSTAT_A::CPU
    }
    #[doc = "Checks if the value of the field is `SPIS`"]
    #[inline(always)]
    pub fn is_spis(&self) -> bool {
        *self == SEMSTAT_A::SPIS
    }
    #[doc = "Checks if the value of the field is `CPUPENDING`"]
    #[inline(always)]
    pub fn is_cpupending(&self) -> bool {
        *self == SEMSTAT_A::CPUPENDING
    }
}
impl R {
    #[doc = "Bits 0:1 - Semaphore status"]
    #[inline(always)]
    pub fn semstat(&self) -> SEMSTAT_R {
        SEMSTAT_R::new((self.bits & 0x03) as u8)
    }
}
