#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CRCSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CRCSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCSTATUSR {
    #[doc = "Packet received with CRC error"]
    CRCERROR,
    #[doc = "Packet received with CRC ok"]
    CRCOK,
}
impl CRCSTATUSR {
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
            CRCSTATUSR::CRCERROR => false,
            CRCSTATUSR::CRCOK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCSTATUSR {
        match value {
            false => CRCSTATUSR::CRCERROR,
            true => CRCSTATUSR::CRCOK,
        }
    }
    #[doc = "Checks if the value of the field is `CRCERROR`"]
    #[inline]
    pub fn is_crcerror(&self) -> bool {
        *self == CRCSTATUSR::CRCERROR
    }
    #[doc = "Checks if the value of the field is `CRCOK`"]
    #[inline]
    pub fn is_crcok(&self) -> bool {
        *self == CRCSTATUSR::CRCOK
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CRC status of packet received"]
    #[inline]
    pub fn crcstatus(&self) -> CRCSTATUSR {
        CRCSTATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
