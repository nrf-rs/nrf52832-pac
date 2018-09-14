#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BREQUEST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `BREQUEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREQUESTR {
    #[doc = "Standard request GET_STATUS"]
    STD_GET_STATUS,
    #[doc = "Standard request CLEAR_FEATURE"]
    STD_CLEAR_FEATURE,
    #[doc = "Standard request SET_FEATURE"]
    STD_SET_FEATURE,
    #[doc = "Standard request SET_ADDRESS"]
    STD_SET_ADDRESS,
    #[doc = "Standard request GET_DESCRIPTOR"]
    STD_GET_DESCRIPTOR,
    #[doc = "Standard request SET_DESCRIPTOR"]
    STD_SET_DESCRIPTOR,
    #[doc = "Standard request GET_CONFIGURATION"]
    STD_GET_CONFIGURATION,
    #[doc = "Standard request SET_CONFIGURATION"]
    STD_SET_CONFIGURATION,
    #[doc = "Standard request GET_INTERFACE"]
    STD_GET_INTERFACE,
    #[doc = "Standard request SET_INTERFACE"]
    STD_SET_INTERFACE,
    #[doc = "Standard request SYNCH_FRAME"]
    STD_SYNCH_FRAME,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BREQUESTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BREQUESTR::STD_GET_STATUS => 0,
            BREQUESTR::STD_CLEAR_FEATURE => 1,
            BREQUESTR::STD_SET_FEATURE => 3,
            BREQUESTR::STD_SET_ADDRESS => 5,
            BREQUESTR::STD_GET_DESCRIPTOR => 6,
            BREQUESTR::STD_SET_DESCRIPTOR => 7,
            BREQUESTR::STD_GET_CONFIGURATION => 8,
            BREQUESTR::STD_SET_CONFIGURATION => 9,
            BREQUESTR::STD_GET_INTERFACE => 10,
            BREQUESTR::STD_SET_INTERFACE => 11,
            BREQUESTR::STD_SYNCH_FRAME => 12,
            BREQUESTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BREQUESTR {
        match value {
            0 => BREQUESTR::STD_GET_STATUS,
            1 => BREQUESTR::STD_CLEAR_FEATURE,
            3 => BREQUESTR::STD_SET_FEATURE,
            5 => BREQUESTR::STD_SET_ADDRESS,
            6 => BREQUESTR::STD_GET_DESCRIPTOR,
            7 => BREQUESTR::STD_SET_DESCRIPTOR,
            8 => BREQUESTR::STD_GET_CONFIGURATION,
            9 => BREQUESTR::STD_SET_CONFIGURATION,
            10 => BREQUESTR::STD_GET_INTERFACE,
            11 => BREQUESTR::STD_SET_INTERFACE,
            12 => BREQUESTR::STD_SYNCH_FRAME,
            i => BREQUESTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STD_GET_STATUS`"]
    #[inline]
    pub fn is_std_get_status(&self) -> bool {
        *self == BREQUESTR::STD_GET_STATUS
    }
    #[doc = "Checks if the value of the field is `STD_CLEAR_FEATURE`"]
    #[inline]
    pub fn is_std_clear_feature(&self) -> bool {
        *self == BREQUESTR::STD_CLEAR_FEATURE
    }
    #[doc = "Checks if the value of the field is `STD_SET_FEATURE`"]
    #[inline]
    pub fn is_std_set_feature(&self) -> bool {
        *self == BREQUESTR::STD_SET_FEATURE
    }
    #[doc = "Checks if the value of the field is `STD_SET_ADDRESS`"]
    #[inline]
    pub fn is_std_set_address(&self) -> bool {
        *self == BREQUESTR::STD_SET_ADDRESS
    }
    #[doc = "Checks if the value of the field is `STD_GET_DESCRIPTOR`"]
    #[inline]
    pub fn is_std_get_descriptor(&self) -> bool {
        *self == BREQUESTR::STD_GET_DESCRIPTOR
    }
    #[doc = "Checks if the value of the field is `STD_SET_DESCRIPTOR`"]
    #[inline]
    pub fn is_std_set_descriptor(&self) -> bool {
        *self == BREQUESTR::STD_SET_DESCRIPTOR
    }
    #[doc = "Checks if the value of the field is `STD_GET_CONFIGURATION`"]
    #[inline]
    pub fn is_std_get_configuration(&self) -> bool {
        *self == BREQUESTR::STD_GET_CONFIGURATION
    }
    #[doc = "Checks if the value of the field is `STD_SET_CONFIGURATION`"]
    #[inline]
    pub fn is_std_set_configuration(&self) -> bool {
        *self == BREQUESTR::STD_SET_CONFIGURATION
    }
    #[doc = "Checks if the value of the field is `STD_GET_INTERFACE`"]
    #[inline]
    pub fn is_std_get_interface(&self) -> bool {
        *self == BREQUESTR::STD_GET_INTERFACE
    }
    #[doc = "Checks if the value of the field is `STD_SET_INTERFACE`"]
    #[inline]
    pub fn is_std_set_interface(&self) -> bool {
        *self == BREQUESTR::STD_SET_INTERFACE
    }
    #[doc = "Checks if the value of the field is `STD_SYNCH_FRAME`"]
    #[inline]
    pub fn is_std_synch_frame(&self) -> bool {
        *self == BREQUESTR::STD_SYNCH_FRAME
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values."]
    #[inline]
    pub fn brequest(&self) -> BREQUESTR {
        BREQUESTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
