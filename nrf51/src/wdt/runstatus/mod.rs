#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RUNSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RUNSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNSTATUSR {
    #[doc = "Watchdog timer is not running."]
    NOTRUNNING,
    #[doc = "Watchdog timer is running."]
    RUNNING,
}
impl RUNSTATUSR {
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
            RUNSTATUSR::NOTRUNNING => false,
            RUNSTATUSR::RUNNING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUNSTATUSR {
        match value {
            false => RUNSTATUSR::NOTRUNNING,
            true => RUNSTATUSR::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline]
    pub fn is_not_running(&self) -> bool {
        *self == RUNSTATUSR::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline]
    pub fn is_running(&self) -> bool {
        *self == RUNSTATUSR::RUNNING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Watchdog running status."]
    #[inline]
    pub fn runstatus(&self) -> RUNSTATUSR {
        RUNSTATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
