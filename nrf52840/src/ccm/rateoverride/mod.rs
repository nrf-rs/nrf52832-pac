#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RATEOVERRIDE {
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
#[doc = "Possible values of the field `RATEOVERRIDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATEOVERRIDER {
    #[doc = "1 Mbps"]
    _1MBIT,
    #[doc = "2 Mbps"]
    _2MBIT,
    #[doc = "125 Kbps"]
    _125KBPS,
    #[doc = "500 Kbps"]
    _500KBPS,
}
impl RATEOVERRIDER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RATEOVERRIDER::_1MBIT => 0,
            RATEOVERRIDER::_2MBIT => 1,
            RATEOVERRIDER::_125KBPS => 2,
            RATEOVERRIDER::_500KBPS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RATEOVERRIDER {
        match value {
            0 => RATEOVERRIDER::_1MBIT,
            1 => RATEOVERRIDER::_2MBIT,
            2 => RATEOVERRIDER::_125KBPS,
            3 => RATEOVERRIDER::_500KBPS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1MBIT`"]
    #[inline]
    pub fn is_1mbit(&self) -> bool {
        *self == RATEOVERRIDER::_1MBIT
    }
    #[doc = "Checks if the value of the field is `_2MBIT`"]
    #[inline]
    pub fn is_2mbit(&self) -> bool {
        *self == RATEOVERRIDER::_2MBIT
    }
    #[doc = "Checks if the value of the field is `_125KBPS`"]
    #[inline]
    pub fn is_125kbps(&self) -> bool {
        *self == RATEOVERRIDER::_125KBPS
    }
    #[doc = "Checks if the value of the field is `_500KBPS`"]
    #[inline]
    pub fn is_500kbps(&self) -> bool {
        *self == RATEOVERRIDER::_500KBPS
    }
}
#[doc = "Values that can be written to the field `RATEOVERRIDE`"]
pub enum RATEOVERRIDEW {
    #[doc = "1 Mbps"]
    _1MBIT,
    #[doc = "2 Mbps"]
    _2MBIT,
    #[doc = "125 Kbps"]
    _125KBPS,
    #[doc = "500 Kbps"]
    _500KBPS,
}
impl RATEOVERRIDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RATEOVERRIDEW::_1MBIT => 0,
            RATEOVERRIDEW::_2MBIT => 1,
            RATEOVERRIDEW::_125KBPS => 2,
            RATEOVERRIDEW::_500KBPS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RATEOVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _RATEOVERRIDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RATEOVERRIDEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 Mbps"]
    #[inline]
    pub fn _1mbit(self) -> &'a mut W {
        self.variant(RATEOVERRIDEW::_1MBIT)
    }
    #[doc = "2 Mbps"]
    #[inline]
    pub fn _2mbit(self) -> &'a mut W {
        self.variant(RATEOVERRIDEW::_2MBIT)
    }
    #[doc = "125 Kbps"]
    #[inline]
    pub fn _125kbps(self) -> &'a mut W {
        self.variant(RATEOVERRIDEW::_125KBPS)
    }
    #[doc = "500 Kbps"]
    #[inline]
    pub fn _500kbps(self) -> &'a mut W {
        self.variant(RATEOVERRIDEW::_500KBPS)
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
    #[doc = "Bits 0:1 - Data rate override setting."]
    #[inline]
    pub fn rateoverride(&self) -> RATEOVERRIDER {
        RATEOVERRIDER::_from({
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
    #[doc = "Bits 0:1 - Data rate override setting."]
    #[inline]
    pub fn rateoverride(&mut self) -> _RATEOVERRIDEW {
        _RATEOVERRIDEW { w: self }
    }
}
