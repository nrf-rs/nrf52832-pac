#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FLASH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `FLASH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHR {
    #[doc = "128 kByte FLASH"]
    K128,
    #[doc = "256 kByte FLASH"]
    K256,
    #[doc = "512 kByte FLASH"]
    K512,
    #[doc = "1 MByte FLASH"]
    K1024,
    #[doc = "2 MByte FLASH"]
    K2048,
    #[doc = "Unspecified"]
    UNSPECIFIED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl FLASHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            FLASHR::K128 => 128,
            FLASHR::K256 => 256,
            FLASHR::K512 => 512,
            FLASHR::K1024 => 1024,
            FLASHR::K2048 => 2048,
            FLASHR::UNSPECIFIED => 4294967295,
            FLASHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> FLASHR {
        match value {
            128 => FLASHR::K128,
            256 => FLASHR::K256,
            512 => FLASHR::K512,
            1024 => FLASHR::K1024,
            2048 => FLASHR::K2048,
            4294967295 => FLASHR::UNSPECIFIED,
            i => FLASHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `K128`"]
    #[inline]
    pub fn is_k128(&self) -> bool {
        *self == FLASHR::K128
    }
    #[doc = "Checks if the value of the field is `K256`"]
    #[inline]
    pub fn is_k256(&self) -> bool {
        *self == FLASHR::K256
    }
    #[doc = "Checks if the value of the field is `K512`"]
    #[inline]
    pub fn is_k512(&self) -> bool {
        *self == FLASHR::K512
    }
    #[doc = "Checks if the value of the field is `K1024`"]
    #[inline]
    pub fn is_k1024(&self) -> bool {
        *self == FLASHR::K1024
    }
    #[doc = "Checks if the value of the field is `K2048`"]
    #[inline]
    pub fn is_k2048(&self) -> bool {
        *self == FLASHR::K2048
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline]
    pub fn is_unspecified(&self) -> bool {
        *self == FLASHR::UNSPECIFIED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline]
    pub fn flash(&self) -> FLASHR {
        FLASHR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
