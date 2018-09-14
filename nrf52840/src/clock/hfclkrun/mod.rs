#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HFCLKRUN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUSR {
    #[doc = "Task not triggered"]
    NOTTRIGGERED,
    #[doc = "Task triggered"]
    TRIGGERED,
}
impl STATUSR {
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
            STATUSR::NOTTRIGGERED => false,
            STATUSR::TRIGGERED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATUSR {
        match value {
            false => STATUSR::NOTTRIGGERED,
            true => STATUSR::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTTRIGGERED`"]
    #[inline]
    pub fn is_not_triggered(&self) -> bool {
        *self == STATUSR::NOTTRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline]
    pub fn is_triggered(&self) -> bool {
        *self == STATUSR::TRIGGERED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - HFCLKSTART task triggered or not"]
    #[inline]
    pub fn status(&self) -> STATUSR {
        STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
