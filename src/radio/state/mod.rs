#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "RADIO is in the Disabled state"]
    DISABLED,
    #[doc = "RADIO is in the RXRU state"]
    RXRU,
    #[doc = "RADIO is in the RXIDLE state"]
    RXIDLE,
    #[doc = "RADIO is in the RX state"]
    RX,
    #[doc = "RADIO is in the RXDISABLED state"]
    RXDISABLE,
    #[doc = "RADIO is in the TXRU state"]
    TXRU,
    #[doc = "RADIO is in the TXIDLE state"]
    TXIDLE,
    #[doc = "RADIO is in the TX state"]
    TX,
    #[doc = "RADIO is in the TXDISABLED state"]
    TXDISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::DISABLED => 0,
            STATER::RXRU => 1,
            STATER::RXIDLE => 2,
            STATER::RX => 3,
            STATER::RXDISABLE => 4,
            STATER::TXRU => 9,
            STATER::TXIDLE => 10,
            STATER::TX => 11,
            STATER::TXDISABLE => 12,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            0 => STATER::DISABLED,
            1 => STATER::RXRU,
            2 => STATER::RXIDLE,
            3 => STATER::RX,
            4 => STATER::RXDISABLE,
            9 => STATER::TXRU,
            10 => STATER::TXIDLE,
            11 => STATER::TX,
            12 => STATER::TXDISABLE,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STATER::DISABLED
    }
    #[doc = "Checks if the value of the field is `RXRU`"]
    #[inline]
    pub fn is_rx_ru(&self) -> bool {
        *self == STATER::RXRU
    }
    #[doc = "Checks if the value of the field is `RXIDLE`"]
    #[inline]
    pub fn is_rx_idle(&self) -> bool {
        *self == STATER::RXIDLE
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline]
    pub fn is_rx(&self) -> bool {
        *self == STATER::RX
    }
    #[doc = "Checks if the value of the field is `RXDISABLE`"]
    #[inline]
    pub fn is_rx_disable(&self) -> bool {
        *self == STATER::RXDISABLE
    }
    #[doc = "Checks if the value of the field is `TXRU`"]
    #[inline]
    pub fn is_tx_ru(&self) -> bool {
        *self == STATER::TXRU
    }
    #[doc = "Checks if the value of the field is `TXIDLE`"]
    #[inline]
    pub fn is_tx_idle(&self) -> bool {
        *self == STATER::TXIDLE
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline]
    pub fn is_tx(&self) -> bool {
        *self == STATER::TX
    }
    #[doc = "Checks if the value of the field is `TXDISABLE`"]
    #[inline]
    pub fn is_tx_disable(&self) -> bool {
        *self == STATER::TXDISABLE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Current radio state"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
