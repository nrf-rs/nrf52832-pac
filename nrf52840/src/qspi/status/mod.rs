#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPMR {
    #[doc = "External flash is not in DPM."]
    DISABLED,
    #[doc = "External flash is in DPM."]
    ENABLED,
}
impl DPMR {
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
            DPMR::DISABLED => false,
            DPMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPMR {
        match value {
            false => DPMR::DISABLED,
            true => DPMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DPMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DPMR::ENABLED
    }
}
#[doc = "Possible values of the field `READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READYR {
    #[doc = "QSPI peripheral is ready. It is allowed to trigger new tasks, writing custom instructions or enter/exit DPM."]
    READY,
    #[doc = "QSPI peripheral is busy. It is not allowed to trigger any new tasks, writing custom instructions or enter/exit DPM."]
    BUSY,
}
impl READYR {
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
            READYR::READY => true,
            READYR::BUSY => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READYR {
        match value {
            true => READYR::READY,
            false => READYR::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == READYR::READY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == READYR::BUSY
    }
}
#[doc = r" Value of the field"]
pub struct SREGR {
    bits: u8,
}
impl SREGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Deep power-down mode (DPM) status of external flash."]
    #[inline]
    pub fn dpm(&self) -> DPMR {
        DPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Ready status."]
    #[inline]
    pub fn ready(&self) -> READYR {
        READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - Value of external flash device Status Register. When the external flash has two bytes status register this field includes the value of the low byte."]
    #[inline]
    pub fn sreg(&self) -> SREGR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SREGR { bits }
    }
}
