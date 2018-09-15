#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BUSY {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "No ongoing ADC conversion is taking place. ADC is ready."]
    READY,
    #[doc = "An ADC conversion is taking place. ADC is busy."]
    BUSY,
}
impl BUSYR {
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
            BUSYR::READY => false,
            BUSYR::BUSY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSYR {
        match value {
            false => BUSYR::READY,
            true => BUSYR::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == BUSYR::READY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == BUSYR::BUSY
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - ADC busy register."]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
