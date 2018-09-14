#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EPIN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `GETSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GETSTATUSR {
    #[doc = "Endpoint is not halted"]
    NOTHALTED,
    #[doc = "Endpoint is halted"]
    HALTED,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl GETSTATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GETSTATUSR::NOTHALTED => 0,
            GETSTATUSR::HALTED => 1,
            GETSTATUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GETSTATUSR {
        match value {
            0 => GETSTATUSR::NOTHALTED,
            1 => GETSTATUSR::HALTED,
            i => GETSTATUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALTED`"]
    #[inline]
    pub fn is_not_halted(&self) -> bool {
        *self == GETSTATUSR::NOTHALTED
    }
    #[doc = "Checks if the value of the field is `HALTED`"]
    #[inline]
    pub fn is_halted(&self) -> bool {
        *self == GETSTATUSR::HALTED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline]
    pub fn getstatus(&self) -> GETSTATUSR {
        GETSTATUSR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
