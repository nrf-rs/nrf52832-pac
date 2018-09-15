#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "1Mbit/s Nordic propietary radio mode."]
    NRF_1MBIT,
    #[doc = "2Mbit/s Nordic propietary radio mode."]
    NRF_2MBIT,
    #[doc = "250kbit/s Nordic propietary radio mode."]
    NRF_250KBIT,
    #[doc = "1Mbit/s Bluetooth Low Energy"]
    BLE_1MBIT,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::NRF_1MBIT => 0,
            MODER::NRF_2MBIT => 1,
            MODER::NRF_250KBIT => 2,
            MODER::BLE_1MBIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::NRF_1MBIT,
            1 => MODER::NRF_2MBIT,
            2 => MODER::NRF_250KBIT,
            3 => MODER::BLE_1MBIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NRF_1MBIT`"]
    #[inline]
    pub fn is_nrf_1mbit(&self) -> bool {
        *self == MODER::NRF_1MBIT
    }
    #[doc = "Checks if the value of the field is `NRF_2MBIT`"]
    #[inline]
    pub fn is_nrf_2mbit(&self) -> bool {
        *self == MODER::NRF_2MBIT
    }
    #[doc = "Checks if the value of the field is `NRF_250KBIT`"]
    #[inline]
    pub fn is_nrf_250kbit(&self) -> bool {
        *self == MODER::NRF_250KBIT
    }
    #[doc = "Checks if the value of the field is `BLE_1MBIT`"]
    #[inline]
    pub fn is_ble_1mbit(&self) -> bool {
        *self == MODER::BLE_1MBIT
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "1Mbit/s Nordic propietary radio mode."]
    NRF_1MBIT,
    #[doc = "2Mbit/s Nordic propietary radio mode."]
    NRF_2MBIT,
    #[doc = "250kbit/s Nordic propietary radio mode."]
    NRF_250KBIT,
    #[doc = "1Mbit/s Bluetooth Low Energy"]
    BLE_1MBIT,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::NRF_1MBIT => 0,
            MODEW::NRF_2MBIT => 1,
            MODEW::NRF_250KBIT => 2,
            MODEW::BLE_1MBIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1Mbit/s Nordic propietary radio mode."]
    #[inline]
    pub fn nrf_1mbit(self) -> &'a mut W {
        self.variant(MODEW::NRF_1MBIT)
    }
    #[doc = "2Mbit/s Nordic propietary radio mode."]
    #[inline]
    pub fn nrf_2mbit(self) -> &'a mut W {
        self.variant(MODEW::NRF_2MBIT)
    }
    #[doc = "250kbit/s Nordic propietary radio mode."]
    #[inline]
    pub fn nrf_250kbit(self) -> &'a mut W {
        self.variant(MODEW::NRF_250KBIT)
    }
    #[doc = "1Mbit/s Bluetooth Low Energy"]
    #[inline]
    pub fn ble_1mbit(self) -> &'a mut W {
        self.variant(MODEW::BLE_1MBIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
