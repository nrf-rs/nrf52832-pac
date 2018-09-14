#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BMREQUESTTYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RECIPIENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECIPIENTR {
    #[doc = "Device"]
    DEVICE,
    #[doc = "Interface"]
    INTERFACE,
    #[doc = "Endpoint"]
    ENDPOINT,
    #[doc = "Other"]
    OTHER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RECIPIENTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RECIPIENTR::DEVICE => 0,
            RECIPIENTR::INTERFACE => 1,
            RECIPIENTR::ENDPOINT => 2,
            RECIPIENTR::OTHER => 3,
            RECIPIENTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RECIPIENTR {
        match value {
            0 => RECIPIENTR::DEVICE,
            1 => RECIPIENTR::INTERFACE,
            2 => RECIPIENTR::ENDPOINT,
            3 => RECIPIENTR::OTHER,
            i => RECIPIENTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline]
    pub fn is_device(&self) -> bool {
        *self == RECIPIENTR::DEVICE
    }
    #[doc = "Checks if the value of the field is `INTERFACE`"]
    #[inline]
    pub fn is_interface(&self) -> bool {
        *self == RECIPIENTR::INTERFACE
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline]
    pub fn is_endpoint(&self) -> bool {
        *self == RECIPIENTR::ENDPOINT
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline]
    pub fn is_other(&self) -> bool {
        *self == RECIPIENTR::OTHER
    }
}
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "Standard"]
    STANDARD,
    #[doc = "Class"]
    CLASS,
    #[doc = "Vendor"]
    VENDOR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPER::STANDARD => 0,
            TYPER::CLASS => 1,
            TYPER::VENDOR => 2,
            TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPER {
        match value {
            0 => TYPER::STANDARD,
            1 => TYPER::CLASS,
            2 => TYPER::VENDOR,
            i => TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == TYPER::STANDARD
    }
    #[doc = "Checks if the value of the field is `CLASS`"]
    #[inline]
    pub fn is_class(&self) -> bool {
        *self == TYPER::CLASS
    }
    #[doc = "Checks if the value of the field is `VENDOR`"]
    #[inline]
    pub fn is_vendor(&self) -> bool {
        *self == TYPER::VENDOR
    }
}
#[doc = "Possible values of the field `DIRECTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTIONR {
    #[doc = "Host-to-device"]
    HOSTTODEVICE,
    #[doc = "Device-to-host"]
    DEVICETOHOST,
}
impl DIRECTIONR {
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
            DIRECTIONR::HOSTTODEVICE => false,
            DIRECTIONR::DEVICETOHOST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRECTIONR {
        match value {
            false => DIRECTIONR::HOSTTODEVICE,
            true => DIRECTIONR::DEVICETOHOST,
        }
    }
    #[doc = "Checks if the value of the field is `HOSTTODEVICE`"]
    #[inline]
    pub fn is_host_to_device(&self) -> bool {
        *self == DIRECTIONR::HOSTTODEVICE
    }
    #[doc = "Checks if the value of the field is `DEVICETOHOST`"]
    #[inline]
    pub fn is_device_to_host(&self) -> bool {
        *self == DIRECTIONR::DEVICETOHOST
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Data transfer type"]
    #[inline]
    pub fn recipient(&self) -> RECIPIENTR {
        RECIPIENTR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Data transfer type"]
    #[inline]
    pub fn type_(&self) -> TYPER {
        TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Data transfer direction"]
    #[inline]
    pub fn direction(&self) -> DIRECTIONR {
        DIRECTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
