#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PDUSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PDUSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDUSTATR {
    #[doc = "Payload less than PCNF1.MAXLEN"]
    LESSTHAN,
    #[doc = "Payload greater than PCNF1.MAXLEN"]
    GREATERTHAN,
}
impl PDUSTATR {
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
            PDUSTATR::LESSTHAN => false,
            PDUSTATR::GREATERTHAN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDUSTATR {
        match value {
            false => PDUSTATR::LESSTHAN,
            true => PDUSTATR::GREATERTHAN,
        }
    }
    #[doc = "Checks if the value of the field is `LESSTHAN`"]
    #[inline]
    pub fn is_less_than(&self) -> bool {
        *self == PDUSTATR::LESSTHAN
    }
    #[doc = "Checks if the value of the field is `GREATERTHAN`"]
    #[inline]
    pub fn is_greater_than(&self) -> bool {
        *self == PDUSTATR::GREATERTHAN
    }
}
#[doc = "Possible values of the field `CISTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CISTATR {
    #[doc = "Frame is received at 125kbps"]
    LR125KBIT,
    #[doc = "Frame is received at 500kbps"]
    LR500KBIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CISTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CISTATR::LR125KBIT => 0,
            CISTATR::LR500KBIT => 1,
            CISTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CISTATR {
        match value {
            0 => CISTATR::LR125KBIT,
            1 => CISTATR::LR500KBIT,
            i => CISTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LR125KBIT`"]
    #[inline]
    pub fn is_lr125kbit(&self) -> bool {
        *self == CISTATR::LR125KBIT
    }
    #[doc = "Checks if the value of the field is `LR500KBIT`"]
    #[inline]
    pub fn is_lr500kbit(&self) -> bool {
        *self == CISTATR::LR500KBIT
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Status on payload length vs. PCNF1.MAXLEN"]
    #[inline]
    pub fn pdustat(&self) -> PDUSTATR {
        PDUSTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Status on what rate packet is received with in Long Range"]
    #[inline]
    pub fn cistat(&self) -> CISTATR {
        CISTATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
