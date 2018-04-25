#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RESULT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RESULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULTR {
    #[doc = "Input voltage is below the threshold (VIN+ &lt; VIN-)"]
    BELOW,
    #[doc = "Input voltage is above the threshold (VIN+ &gt; VIN-)"]
    ABOVE,
}
impl RESULTR {
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
            RESULTR::BELOW => false,
            RESULTR::ABOVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESULTR {
        match value {
            false => RESULTR::BELOW,
            true => RESULTR::ABOVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline]
    pub fn is_below(&self) -> bool {
        *self == RESULTR::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline]
    pub fn is_above(&self) -> bool {
        *self == RESULTR::ABOVE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Result of last compare. Decision point SAMPLE task."]
    #[inline]
    pub fn result(&self) -> RESULTR {
        RESULTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
