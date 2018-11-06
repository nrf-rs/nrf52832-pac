#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HYST {
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
#[doc = "Possible values of the field `HYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTR {
    #[doc = "Comparator hysteresis disabled"]
    NOHYST,
    #[doc = "Comparator hysteresis disabled (typ. 50 mV)"]
    HYST50MV,
}
impl HYSTR {
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
            HYSTR::NOHYST => false,
            HYSTR::HYST50MV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSTR {
        match value {
            false => HYSTR::NOHYST,
            true => HYSTR::HYST50MV,
        }
    }
    #[doc = "Checks if the value of the field is `NOHYST`"]
    #[inline]
    pub fn is_no_hyst(&self) -> bool {
        *self == HYSTR::NOHYST
    }
    #[doc = "Checks if the value of the field is `HYST50MV`"]
    #[inline]
    pub fn is_hyst50m_v(&self) -> bool {
        *self == HYSTR::HYST50MV
    }
}
#[doc = "Values that can be written to the field `HYST`"]
pub enum HYSTW {
    #[doc = "Comparator hysteresis disabled"]
    NOHYST,
    #[doc = "Comparator hysteresis disabled (typ. 50 mV)"]
    HYST50MV,
}
impl HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSTW::NOHYST => false,
            HYSTW::HYST50MV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Comparator hysteresis disabled"]
    #[inline]
    pub fn no_hyst(self) -> &'a mut W {
        self.variant(HYSTW::NOHYST)
    }
    #[doc = "Comparator hysteresis disabled (typ. 50 mV)"]
    #[inline]
    pub fn hyst50m_v(self) -> &'a mut W {
        self.variant(HYSTW::HYST50MV)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Comparator hysteresis enable"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        HYSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Comparator hysteresis enable"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
}
