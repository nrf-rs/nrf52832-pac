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
    #[doc = "BAAA"]
    BAAA,
    #[doc = "CAAA"]
    CAAA,
    #[doc = "AABA"]
    AABA,
    #[doc = "AABB"]
    AABB,
    #[doc = "AACA"]
    AACA,
    #[doc = "AAAB"]
    AAAB,
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
            VARIANTR::BAAA => 1111572801,
            VARIANTR::CAAA => 1128350017,
            VARIANTR::AABA => 1094795841,
            VARIANTR::AABB => 1094795842,
            VARIANTR::AACA => 1094796097,
            VARIANTR::AAAB => 1094795586,
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
            1111572801 => VARIANTR::BAAA,
            1128350017 => VARIANTR::CAAA,
            1094795841 => VARIANTR::AABA,
            1094795842 => VARIANTR::AABB,
            1094796097 => VARIANTR::AACA,
            1094795586 => VARIANTR::AAAB,
            4294967295 => VARIANTR::UNSPECIFIED,
            i => VARIANTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANTR::AAAA
    }
    #[doc = "Checks if the value of the field is `BAAA`"]
    #[inline]
    pub fn is_baaa(&self) -> bool {
        *self == VARIANTR::BAAA
    }
    #[doc = "Checks if the value of the field is `CAAA`"]
    #[inline]
    pub fn is_caaa(&self) -> bool {
        *self == VARIANTR::CAAA
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
    #[doc = "Checks if the value of the field is `AACA`"]
    #[inline]
    pub fn is_aaca(&self) -> bool {
        *self == VARIANTR::AACA
    }
    #[doc = "Checks if the value of the field is `AAAB`"]
    #[inline]
    pub fn is_aaab(&self) -> bool {
        *self == VARIANTR::AAAB
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
    #[doc = "Bits 0:31 - Build code (hardware version and production configuration). Encoded as ASCII."]
    #[inline]
    pub fn variant(&self) -> VARIANTR {
        VARIANTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
