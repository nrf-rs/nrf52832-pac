#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBREGSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `VBUSDETECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSDETECTR {
    #[doc = "VBUS voltage below valid threshold"]
    NOVBUS,
    #[doc = "VBUS voltage above valid threshold"]
    VBUSPRESENT,
}
impl VBUSDETECTR {
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
            VBUSDETECTR::NOVBUS => false,
            VBUSDETECTR::VBUSPRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBUSDETECTR {
        match value {
            false => VBUSDETECTR::NOVBUS,
            true => VBUSDETECTR::VBUSPRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOVBUS`"]
    #[inline]
    pub fn is_no_vbus(&self) -> bool {
        *self == VBUSDETECTR::NOVBUS
    }
    #[doc = "Checks if the value of the field is `VBUSPRESENT`"]
    #[inline]
    pub fn is_vbus_present(&self) -> bool {
        *self == VBUSDETECTR::VBUSPRESENT
    }
}
#[doc = "Possible values of the field `OUTPUTRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTPUTRDYR {
    #[doc = "USBREG output settling time not elapsed"]
    NOTREADY,
    #[doc = "USBREG output settling time elapsed (same information as USBPWRRDY event)"]
    READY,
}
impl OUTPUTRDYR {
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
            OUTPUTRDYR::NOTREADY => false,
            OUTPUTRDYR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTPUTRDYR {
        match value {
            false => OUTPUTRDYR::NOTREADY,
            true => OUTPUTRDYR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == OUTPUTRDYR::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == OUTPUTRDYR::READY
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
    #[inline]
    pub fn vbusdetect(&self) -> VBUSDETECTR {
        VBUSDETECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - USB supply output settling time elapsed"]
    #[inline]
    pub fn outputrdy(&self) -> OUTPUTRDYR {
        OUTPUTRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
