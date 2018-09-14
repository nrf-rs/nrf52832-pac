#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SEMSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SEMSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMSTATR {
    #[doc = "Semaphore is free"]
    FREE,
    #[doc = "Semaphore is assigned to CPU"]
    CPU,
    #[doc = "Semaphore is assigned to SPI slave"]
    SPIS,
    #[doc = "Semaphore is assigned to SPI but a handover to the CPU is pending"]
    CPUPENDING,
}
impl SEMSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEMSTATR::FREE => 0,
            SEMSTATR::CPU => 1,
            SEMSTATR::SPIS => 2,
            SEMSTATR::CPUPENDING => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEMSTATR {
        match value {
            0 => SEMSTATR::FREE,
            1 => SEMSTATR::CPU,
            2 => SEMSTATR::SPIS,
            3 => SEMSTATR::CPUPENDING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline]
    pub fn is_free(&self) -> bool {
        *self == SEMSTATR::FREE
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline]
    pub fn is_cpu(&self) -> bool {
        *self == SEMSTATR::CPU
    }
    #[doc = "Checks if the value of the field is `SPIS`"]
    #[inline]
    pub fn is_spis(&self) -> bool {
        *self == SEMSTATR::SPIS
    }
    #[doc = "Checks if the value of the field is `CPUPENDING`"]
    #[inline]
    pub fn is_cpupending(&self) -> bool {
        *self == SEMSTATR::CPUPENDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Semaphore status"]
    #[inline]
    pub fn semstat(&self) -> SEMSTATR {
        SEMSTATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
