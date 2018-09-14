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
    #[doc = "1 Mbit/s Nordic proprietary radio mode"]
    NRF_1MBIT,
    #[doc = "2 Mbit/s Nordic proprietary radio mode"]
    NRF_2MBIT,
    #[doc = "1 Mbit/s BLE"]
    BLE_1MBIT,
    #[doc = "2 Mbit/s BLE"]
    BLE_2MBIT,
    #[doc = "Long range 125 kbit/s TX, 125 kbit/s and 500 kbit/s RX"]
    BLE_LR125KBIT,
    #[doc = "Long range 500 kbit/s TX, 125 kbit/s and 500 kbit/s RX"]
    BLE_LR500KBIT,
    #[doc = "IEEE 802.15.4-2006 250 kbit/s"]
    IEEE802154_250KBIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::NRF_1MBIT => 0,
            MODER::NRF_2MBIT => 1,
            MODER::BLE_1MBIT => 3,
            MODER::BLE_2MBIT => 4,
            MODER::BLE_LR125KBIT => 5,
            MODER::BLE_LR500KBIT => 6,
            MODER::IEEE802154_250KBIT => 15,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::NRF_1MBIT,
            1 => MODER::NRF_2MBIT,
            3 => MODER::BLE_1MBIT,
            4 => MODER::BLE_2MBIT,
            5 => MODER::BLE_LR125KBIT,
            6 => MODER::BLE_LR500KBIT,
            15 => MODER::IEEE802154_250KBIT,
            i => MODER::_Reserved(i),
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
    #[doc = "Checks if the value of the field is `BLE_1MBIT`"]
    #[inline]
    pub fn is_ble_1mbit(&self) -> bool {
        *self == MODER::BLE_1MBIT
    }
    #[doc = "Checks if the value of the field is `BLE_2MBIT`"]
    #[inline]
    pub fn is_ble_2mbit(&self) -> bool {
        *self == MODER::BLE_2MBIT
    }
    #[doc = "Checks if the value of the field is `BLE_LR125KBIT`"]
    #[inline]
    pub fn is_ble_lr125kbit(&self) -> bool {
        *self == MODER::BLE_LR125KBIT
    }
    #[doc = "Checks if the value of the field is `BLE_LR500KBIT`"]
    #[inline]
    pub fn is_ble_lr500kbit(&self) -> bool {
        *self == MODER::BLE_LR500KBIT
    }
    #[doc = "Checks if the value of the field is `IEEE802154_250KBIT`"]
    #[inline]
    pub fn is_ieee802154_250kbit(&self) -> bool {
        *self == MODER::IEEE802154_250KBIT
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "1 Mbit/s Nordic proprietary radio mode"]
    NRF_1MBIT,
    #[doc = "2 Mbit/s Nordic proprietary radio mode"]
    NRF_2MBIT,
    #[doc = "1 Mbit/s BLE"]
    BLE_1MBIT,
    #[doc = "2 Mbit/s BLE"]
    BLE_2MBIT,
    #[doc = "Long range 125 kbit/s TX, 125 kbit/s and 500 kbit/s RX"]
    BLE_LR125KBIT,
    #[doc = "Long range 500 kbit/s TX, 125 kbit/s and 500 kbit/s RX"]
    BLE_LR500KBIT,
    #[doc = "IEEE 802.15.4-2006 250 kbit/s"]
    IEEE802154_250KBIT,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::NRF_1MBIT => 0,
            MODEW::NRF_2MBIT => 1,
            MODEW::BLE_1MBIT => 3,
            MODEW::BLE_2MBIT => 4,
            MODEW::BLE_LR125KBIT => 5,
            MODEW::BLE_LR500KBIT => 6,
            MODEW::IEEE802154_250KBIT => 15,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 Mbit/s Nordic proprietary radio mode"]
    #[inline]
    pub fn nrf_1mbit(self) -> &'a mut W {
        self.variant(MODEW::NRF_1MBIT)
    }
    #[doc = "2 Mbit/s Nordic proprietary radio mode"]
    #[inline]
    pub fn nrf_2mbit(self) -> &'a mut W {
        self.variant(MODEW::NRF_2MBIT)
    }
    #[doc = "1 Mbit/s BLE"]
    #[inline]
    pub fn ble_1mbit(self) -> &'a mut W {
        self.variant(MODEW::BLE_1MBIT)
    }
    #[doc = "2 Mbit/s BLE"]
    #[inline]
    pub fn ble_2mbit(self) -> &'a mut W {
        self.variant(MODEW::BLE_2MBIT)
    }
    #[doc = "Long range 125 kbit/s TX, 125 kbit/s and 500 kbit/s RX"]
    #[inline]
    pub fn ble_lr125kbit(self) -> &'a mut W {
        self.variant(MODEW::BLE_LR125KBIT)
    }
    #[doc = "Long range 500 kbit/s TX, 125 kbit/s and 500 kbit/s RX"]
    #[inline]
    pub fn ble_lr500kbit(self) -> &'a mut W {
        self.variant(MODEW::BLE_LR500KBIT)
    }
    #[doc = "IEEE 802.15.4-2006 250 kbit/s"]
    #[inline]
    pub fn ieee802154_250kbit(self) -> &'a mut W {
        self.variant(MODEW::IEEE802154_250KBIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
