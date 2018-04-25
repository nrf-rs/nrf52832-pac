#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VARIANT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `VARIANT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VARIANTR {
    #[doc = "AAAA"]
    AAAA,
    #[doc = "AAAB"]
    AAAB,
    #[doc = "AABA"]
    AABA,
    #[doc = "AABB"]
    AABB,
    #[doc = "AAB0"]
    AAB0,
    #[doc = "SPA0"]
    SPA0,
    #[doc = "Unspecified"]
    UNSPECIFIED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl VARIANTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            VARIANTR::AAAA => 1094795585,
            VARIANTR::AAAB => 1094795586,
            VARIANTR::AABA => 1094795841,
            VARIANTR::AABB => 1094795842,
            VARIANTR::AAB0 => 1094795824,
            VARIANTR::SPA0 => 1397768496,
            VARIANTR::UNSPECIFIED => 4294967295,
            VARIANTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> VARIANTR {
        match value {
            1094795585 => VARIANTR::AAAA,
            1094795586 => VARIANTR::AAAB,
            1094795841 => VARIANTR::AABA,
            1094795842 => VARIANTR::AABB,
            1094795824 => VARIANTR::AAB0,
            1397768496 => VARIANTR::SPA0,
            4294967295 => VARIANTR::UNSPECIFIED,
            i => VARIANTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANTR::AAAA
    }
    #[doc = "Checks if the value of the field is `AAAB`"]
    #[inline]
    pub fn is_aaab(&self) -> bool {
        *self == VARIANTR::AAAB
    }
    #[doc = "Checks if the value of the field is `AABA`"]
    #[inline]
    pub fn is_aaba(&self) -> bool {
        *self == VARIANTR::AABA
    }
    #[doc = "Checks if the value of the field is `AABB`"]
    #[inline]
    pub fn is_aabb(&self) -> bool {
        *self == VARIANTR::AABB
    }
    #[doc = "Checks if the value of the field is `AAB0`"]
    #[inline]
    pub fn is_aab0(&self) -> bool {
        *self == VARIANTR::AAB0
    }
    #[doc = "Checks if the value of the field is `SPA0`"]
    #[inline]
    pub fn is_spa0(&self) -> bool {
        *self == VARIANTR::SPA0
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline]
    pub fn is_unspecified(&self) -> bool {
        *self == VARIANTR::UNSPECIFIED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline]
    pub fn variant(&self) -> VARIANTR {
        VARIANTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
