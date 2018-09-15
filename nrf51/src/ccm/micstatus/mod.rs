#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MICSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MICSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MICSTATUSR {
    #[doc = "MIC check failed."]
    CHECKFAILED,
    #[doc = "MIC check passed."]
    CHECKPASSED,
}
impl MICSTATUSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MICSTATUSR::CHECKFAILED => false,
            MICSTATUSR::CHECKPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MICSTATUSR {
        match value {
            false => MICSTATUSR::CHECKFAILED,
            true => MICSTATUSR::CHECKPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `CHECKFAILED`"]
    #[inline]
    pub fn is_check_failed(&self) -> bool {
        *self == MICSTATUSR::CHECKFAILED
    }
    #[doc = "Checks if the value of the field is `CHECKPASSED`"]
    #[inline]
    pub fn is_check_passed(&self) -> bool {
        *self == MICSTATUSR::CHECKPASSED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Result of the MIC check performed during the previous CCM RX STARTCRYPT"]
    #[inline]
    pub fn micstatus(&self) -> MICSTATUSR {
        MICSTATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
