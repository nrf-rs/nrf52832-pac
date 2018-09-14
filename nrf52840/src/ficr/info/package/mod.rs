#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PACKAGE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PACKAGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKAGER {
    #[doc = "QIxx - 73-pin aQFN"]
    QI,
    #[doc = "Unspecified"]
    UNSPECIFIED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PACKAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PACKAGER::QI => 8196,
            PACKAGER::UNSPECIFIED => 4294967295,
            PACKAGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PACKAGER {
        match value {
            8196 => PACKAGER::QI,
            4294967295 => PACKAGER::UNSPECIFIED,
            i => PACKAGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `QI`"]
    #[inline]
    pub fn is_qi(&self) -> bool {
        *self == PACKAGER::QI
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline]
    pub fn is_unspecified(&self) -> bool {
        *self == PACKAGER::UNSPECIFIED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Package option"]
    #[inline]
    pub fn package(&self) -> PACKAGER {
        PACKAGER::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
