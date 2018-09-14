#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SLEEPSTATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SLEEPSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPSTATER {
    #[doc = "State is IDLE."]
    IDLE,
    #[doc = "State is SLEEP_A."]
    SLEEPA,
}
impl SLEEPSTATER {
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
            SLEEPSTATER::IDLE => false,
            SLEEPSTATER::SLEEPA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPSTATER {
        match value {
            false => SLEEPSTATER::IDLE,
            true => SLEEPSTATER::SLEEPA,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == SLEEPSTATER::IDLE
    }
    #[doc = "Checks if the value of the field is `SLEEPA`"]
    #[inline]
    pub fn is_sleep_a(&self) -> bool {
        *self == SLEEPSTATER::SLEEPA
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task."]
    #[inline]
    pub fn sleepstate(&self) -> SLEEPSTATER {
        SLEEPSTATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
