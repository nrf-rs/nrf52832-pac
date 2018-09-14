#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FIELDPRESENT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `FIELDPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDPRESENTR {
    #[doc = "No valid field detected"]
    NOFIELD,
    #[doc = "Valid field detected"]
    FIELDPRESENT,
}
impl FIELDPRESENTR {
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
            FIELDPRESENTR::NOFIELD => false,
            FIELDPRESENTR::FIELDPRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIELDPRESENTR {
        match value {
            false => FIELDPRESENTR::NOFIELD,
            true => FIELDPRESENTR::FIELDPRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFIELD`"]
    #[inline]
    pub fn is_no_field(&self) -> bool {
        *self == FIELDPRESENTR::NOFIELD
    }
    #[doc = "Checks if the value of the field is `FIELDPRESENT`"]
    #[inline]
    pub fn is_field_present(&self) -> bool {
        *self == FIELDPRESENTR::FIELDPRESENT
    }
}
#[doc = "Possible values of the field `LOCKDETECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKDETECTR {
    #[doc = "Not locked to field"]
    NOTLOCKED,
    #[doc = "Locked to field"]
    LOCKED,
}
impl LOCKDETECTR {
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
            LOCKDETECTR::NOTLOCKED => false,
            LOCKDETECTR::LOCKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKDETECTR {
        match value {
            false => LOCKDETECTR::NOTLOCKED,
            true => LOCKDETECTR::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLOCKED`"]
    #[inline]
    pub fn is_not_locked(&self) -> bool {
        *self == LOCKDETECTR::NOTLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == LOCKDETECTR::LOCKED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Indicates if a valid field is present. Available only in the activated state."]
    #[inline]
    pub fn fieldpresent(&self) -> FIELDPRESENTR {
        FIELDPRESENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Indicates if the low level has locked to the field"]
    #[inline]
    pub fn lockdetect(&self) -> LOCKDETECTR {
        LOCKDETECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
