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
    #[doc = "QFxx - 48-pin QFN"]
    QF,
    #[doc = "CHxx - 7x8 WLCSP 56 balls"]
    CH,
    #[doc = "CIxx - 7x8 WLCSP 56 balls"]
    CI,
    #[doc = "CKxx - 7x8 WLCSP 56 balls with backside coating for light protection"]
    CK,
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
            PACKAGER::QF => 8192,
            PACKAGER::CH => 8193,
            PACKAGER::CI => 8194,
            PACKAGER::CK => 8197,
            PACKAGER::UNSPECIFIED => 4294967295,
            PACKAGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PACKAGER {
        match value {
            8192 => PACKAGER::QF,
            8193 => PACKAGER::CH,
            8194 => PACKAGER::CI,
            8197 => PACKAGER::CK,
            4294967295 => PACKAGER::UNSPECIFIED,
            i => PACKAGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `QF`"]
    #[inline]
    pub fn is_qf(&self) -> bool {
        *self == PACKAGER::QF
    }
    #[doc = "Checks if the value of the field is `CH`"]
    #[inline]
    pub fn is_ch(&self) -> bool {
        *self == PACKAGER::CH
    }
    #[doc = "Checks if the value of the field is `CI`"]
    #[inline]
    pub fn is_ci(&self) -> bool {
        *self == PACKAGER::CI
    }
    #[doc = "Checks if the value of the field is `CK`"]
    #[inline]
    pub fn is_ck(&self) -> bool {
        *self == PACKAGER::CK
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
