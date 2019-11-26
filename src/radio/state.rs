#[doc = "Reader of register STATE"]
pub type R = crate::R<u32, super::STATE>;
#[doc = "Current radio state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: RADIO is in the Disabled state"]
    DISABLED,
    #[doc = "1: RADIO is in the RXRU state"]
    RXRU,
    #[doc = "2: RADIO is in the RXIDLE state"]
    RXIDLE,
    #[doc = "3: RADIO is in the RX state"]
    RX,
    #[doc = "4: RADIO is in the RXDISABLED state"]
    RXDISABLE,
    #[doc = "9: RADIO is in the TXRU state"]
    TXRU,
    #[doc = "10: RADIO is in the TXIDLE state"]
    TXIDLE,
    #[doc = "11: RADIO is in the TX state"]
    TX,
    #[doc = "12: RADIO is in the TXDISABLED state"]
    TXDISABLE,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        match variant {
            STATE_A::DISABLED => 0,
            STATE_A::RXRU => 1,
            STATE_A::RXIDLE => 2,
            STATE_A::RX => 3,
            STATE_A::RXDISABLE => 4,
            STATE_A::TXRU => 9,
            STATE_A::TXIDLE => 10,
            STATE_A::TX => 11,
            STATE_A::TXDISABLE => 12,
        }
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_A::DISABLED),
            1 => Val(STATE_A::RXRU),
            2 => Val(STATE_A::RXIDLE),
            3 => Val(STATE_A::RX),
            4 => Val(STATE_A::RXDISABLE),
            9 => Val(STATE_A::TXRU),
            10 => Val(STATE_A::TXIDLE),
            11 => Val(STATE_A::TX),
            12 => Val(STATE_A::TXDISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RXRU`"]
    #[inline(always)]
    pub fn is_rx_ru(&self) -> bool {
        *self == STATE_A::RXRU
    }
    #[doc = "Checks if the value of the field is `RXIDLE`"]
    #[inline(always)]
    pub fn is_rx_idle(&self) -> bool {
        *self == STATE_A::RXIDLE
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == STATE_A::RX
    }
    #[doc = "Checks if the value of the field is `RXDISABLE`"]
    #[inline(always)]
    pub fn is_rx_disable(&self) -> bool {
        *self == STATE_A::RXDISABLE
    }
    #[doc = "Checks if the value of the field is `TXRU`"]
    #[inline(always)]
    pub fn is_tx_ru(&self) -> bool {
        *self == STATE_A::TXRU
    }
    #[doc = "Checks if the value of the field is `TXIDLE`"]
    #[inline(always)]
    pub fn is_tx_idle(&self) -> bool {
        *self == STATE_A::TXIDLE
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == STATE_A::TX
    }
    #[doc = "Checks if the value of the field is `TXDISABLE`"]
    #[inline(always)]
    pub fn is_tx_disable(&self) -> bool {
        *self == STATE_A::TXDISABLE
    }
}
impl R {
    #[doc = "Bits 0:3 - Current radio state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x0f) as u8)
    }
}
