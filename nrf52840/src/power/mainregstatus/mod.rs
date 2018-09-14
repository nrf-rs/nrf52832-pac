#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAINREGSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MAINREGSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAINREGSTATUSR {
    #[doc = "Normal voltage mode. Voltage supplied on VDD."]
    NORMAL,
    #[doc = "High voltage mode. Voltage supplied on VDDH."]
    HIGH,
}
impl MAINREGSTATUSR {
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
            MAINREGSTATUSR::NORMAL => false,
            MAINREGSTATUSR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAINREGSTATUSR {
        match value {
            false => MAINREGSTATUSR::NORMAL,
            true => MAINREGSTATUSR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == MAINREGSTATUSR::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == MAINREGSTATUSR::HIGH
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Main supply status"]
    #[inline]
    pub fn mainregstatus(&self) -> MAINREGSTATUSR {
        MAINREGSTATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
