#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ISOOUT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SIZER {
    bits: u16,
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZEROR {
    #[doc = "No zero-length data received, use value in SIZE"]
    NORMAL,
    #[doc = "Zero-length data received, ignore value in SIZE"]
    ZERODATA,
}
impl ZEROR {
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
            ZEROR::NORMAL => false,
            ZEROR::ZERODATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZEROR {
        match value {
            false => ZEROR::NORMAL,
            true => ZEROR::ZERODATA,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == ZEROR::NORMAL
    }
    #[doc = "Checks if the value of the field is `ZERODATA`"]
    #[inline]
    pub fn is_zero_data(&self) -> bool {
        *self == ZEROR::ZERODATA
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Number of bytes received last on this ISO OUT data endpoint"]
    #[inline]
    pub fn size(&self) -> SIZER {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SIZER { bits }
    }
    #[doc = "Bit 16 - Zero-length data packet received"]
    #[inline]
    pub fn zero(&self) -> ZEROR {
        ZEROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
