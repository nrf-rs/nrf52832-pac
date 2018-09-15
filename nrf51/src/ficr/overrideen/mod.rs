#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OVERRIDEEN {
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
#[doc = "Possible values of the field `NRF_1MBIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRF_1MBITR {
    #[doc = "Override the default values for NRF_1Mbit mode."]
    OVERRIDE,
    #[doc = "Do not override the default values for NRF_1Mbit mode."]
    NOTOVERRIDE,
}
impl NRF_1MBITR {
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
            NRF_1MBITR::OVERRIDE => false,
            NRF_1MBITR::NOTOVERRIDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NRF_1MBITR {
        match value {
            false => NRF_1MBITR::OVERRIDE,
            true => NRF_1MBITR::NOTOVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline]
    pub fn is_override_(&self) -> bool {
        *self == NRF_1MBITR::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `NOTOVERRIDE`"]
    #[inline]
    pub fn is_not_override(&self) -> bool {
        *self == NRF_1MBITR::NOTOVERRIDE
    }
}
#[doc = "Possible values of the field `BLE_1MBIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_1MBITR {
    #[doc = "Override the default values for BLE_1Mbit mode."]
    OVERRIDE,
    #[doc = "Do not override the default values for BLE_1Mbit mode."]
    NOTOVERRIDE,
}
impl BLE_1MBITR {
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
            BLE_1MBITR::OVERRIDE => false,
            BLE_1MBITR::NOTOVERRIDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLE_1MBITR {
        match value {
            false => BLE_1MBITR::OVERRIDE,
            true => BLE_1MBITR::NOTOVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline]
    pub fn is_override_(&self) -> bool {
        *self == BLE_1MBITR::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `NOTOVERRIDE`"]
    #[inline]
    pub fn is_not_override(&self) -> bool {
        *self == BLE_1MBITR::NOTOVERRIDE
    }
}
#[doc = "Values that can be written to the field `NRF_1MBIT`"]
pub enum NRF_1MBITW {
    #[doc = "Override the default values for NRF_1Mbit mode."]
    OVERRIDE,
    #[doc = "Do not override the default values for NRF_1Mbit mode."]
    NOTOVERRIDE,
}
impl NRF_1MBITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NRF_1MBITW::OVERRIDE => false,
            NRF_1MBITW::NOTOVERRIDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NRF_1MBITW<'a> {
    w: &'a mut W,
}
impl<'a> _NRF_1MBITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NRF_1MBITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Override the default values for NRF_1Mbit mode."]
    #[inline]
    pub fn override_(self) -> &'a mut W {
        self.variant(NRF_1MBITW::OVERRIDE)
    }
    #[doc = "Do not override the default values for NRF_1Mbit mode."]
    #[inline]
    pub fn not_override(self) -> &'a mut W {
        self.variant(NRF_1MBITW::NOTOVERRIDE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLE_1MBIT`"]
pub enum BLE_1MBITW {
    #[doc = "Override the default values for BLE_1Mbit mode."]
    OVERRIDE,
    #[doc = "Do not override the default values for BLE_1Mbit mode."]
    NOTOVERRIDE,
}
impl BLE_1MBITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLE_1MBITW::OVERRIDE => false,
            BLE_1MBITW::NOTOVERRIDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLE_1MBITW<'a> {
    w: &'a mut W,
}
impl<'a> _BLE_1MBITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLE_1MBITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Override the default values for BLE_1Mbit mode."]
    #[inline]
    pub fn override_(self) -> &'a mut W {
        self.variant(BLE_1MBITW::OVERRIDE)
    }
    #[doc = "Do not override the default values for BLE_1Mbit mode."]
    #[inline]
    pub fn not_override(self) -> &'a mut W {
        self.variant(BLE_1MBITW::NOTOVERRIDE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Override default values for NRF_1Mbit mode."]
    #[inline]
    pub fn nrf_1mbit(&self) -> NRF_1MBITR {
        NRF_1MBITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Override default values for BLE_1Mbit mode."]
    #[inline]
    pub fn ble_1mbit(&self) -> BLE_1MBITR {
        BLE_1MBITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Override default values for NRF_1Mbit mode."]
    #[inline]
    pub fn nrf_1mbit(&mut self) -> _NRF_1MBITW {
        _NRF_1MBITW { w: self }
    }
    #[doc = "Bit 3 - Override default values for BLE_1Mbit mode."]
    #[inline]
    pub fn ble_1mbit(&mut self) -> _BLE_1MBITW {
        _BLE_1MBITW { w: self }
    }
}
