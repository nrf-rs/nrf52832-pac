#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
#[doc = "Possible values of the field `HWFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWFCR {
    #[doc = "Hardware flow control disabled."]
    DISABLED,
    #[doc = "Hardware flow control enabled."]
    ENABLED,
}
impl HWFCR {
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
            HWFCR::DISABLED => false,
            HWFCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HWFCR {
        match value {
            false => HWFCR::DISABLED,
            true => HWFCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HWFCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HWFCR::ENABLED
    }
}
#[doc = "Possible values of the field `PARITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYR {
    #[doc = "Parity bit excluded."]
    EXCLUDED,
    #[doc = "Parity bit included."]
    INCLUDED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PARITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARITYR::EXCLUDED => 0,
            PARITYR::INCLUDED => 7,
            PARITYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARITYR {
        match value {
            0 => PARITYR::EXCLUDED,
            7 => PARITYR::INCLUDED,
            i => PARITYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == PARITYR::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == PARITYR::INCLUDED
    }
}
#[doc = "Values that can be written to the field `HWFC`"]
pub enum HWFCW {
    #[doc = "Hardware flow control disabled."]
    DISABLED,
    #[doc = "Hardware flow control enabled."]
    ENABLED,
}
impl HWFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HWFCW::DISABLED => false,
            HWFCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HWFCW<'a> {
    w: &'a mut W,
}
impl<'a> _HWFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HWFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware flow control disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWFCW::DISABLED)
    }
    #[doc = "Hardware flow control enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWFCW::ENABLED)
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
#[doc = "Values that can be written to the field `PARITY`"]
pub enum PARITYW {
    #[doc = "Parity bit excluded."]
    EXCLUDED,
    #[doc = "Parity bit included."]
    INCLUDED,
}
impl PARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARITYW::EXCLUDED => 0,
            PARITYW::INCLUDED => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Parity bit excluded."]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(PARITYW::EXCLUDED)
    }
    #[doc = "Parity bit included."]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(PARITYW::INCLUDED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Hardware flow control."]
    #[inline]
    pub fn hwfc(&self) -> HWFCR {
        HWFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Include parity bit."]
    #[inline]
    pub fn parity(&self) -> PARITYR {
        PARITYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Hardware flow control."]
    #[inline]
    pub fn hwfc(&mut self) -> _HWFCW {
        _HWFCW { w: self }
    }
    #[doc = "Bits 1:3 - Include parity bit."]
    #[inline]
    pub fn parity(&mut self) -> _PARITYW {
        _PARITYW { w: self }
    }
}
