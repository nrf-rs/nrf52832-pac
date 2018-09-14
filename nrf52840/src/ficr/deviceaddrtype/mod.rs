#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEVICEADDRTYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DEVICEADDRTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVICEADDRTYPER {
    #[doc = "Public address"]
    PUBLIC,
    #[doc = "Random address"]
    RANDOM,
}
impl DEVICEADDRTYPER {
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
            DEVICEADDRTYPER::PUBLIC => false,
            DEVICEADDRTYPER::RANDOM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEVICEADDRTYPER {
        match value {
            false => DEVICEADDRTYPER::PUBLIC,
            true => DEVICEADDRTYPER::RANDOM,
        }
    }
    #[doc = "Checks if the value of the field is `PUBLIC`"]
    #[inline]
    pub fn is_public(&self) -> bool {
        *self == DEVICEADDRTYPER::PUBLIC
    }
    #[doc = "Checks if the value of the field is `RANDOM`"]
    #[inline]
    pub fn is_random(&self) -> bool {
        *self == DEVICEADDRTYPER::RANDOM
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Device address type"]
    #[inline]
    pub fn deviceaddrtype(&self) -> DEVICEADDRTYPER {
        DEVICEADDRTYPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
