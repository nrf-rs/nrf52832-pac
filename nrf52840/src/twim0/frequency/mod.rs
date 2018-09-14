#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FREQUENCY {
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
#[doc = "Possible values of the field `FREQUENCY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQUENCYR {
    #[doc = "100 kbps"]
    K100,
    #[doc = "250 kbps"]
    K250,
    #[doc = "400 kbps"]
    K400,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl FREQUENCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            FREQUENCYR::K100 => 26738688,
            FREQUENCYR::K250 => 67108864,
            FREQUENCYR::K400 => 104857600,
            FREQUENCYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> FREQUENCYR {
        match value {
            26738688 => FREQUENCYR::K100,
            67108864 => FREQUENCYR::K250,
            104857600 => FREQUENCYR::K400,
            i => FREQUENCYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `K100`"]
    #[inline]
    pub fn is_k100(&self) -> bool {
        *self == FREQUENCYR::K100
    }
    #[doc = "Checks if the value of the field is `K250`"]
    #[inline]
    pub fn is_k250(&self) -> bool {
        *self == FREQUENCYR::K250
    }
    #[doc = "Checks if the value of the field is `K400`"]
    #[inline]
    pub fn is_k400(&self) -> bool {
        *self == FREQUENCYR::K400
    }
}
#[doc = "Values that can be written to the field `FREQUENCY`"]
pub enum FREQUENCYW {
    #[doc = "100 kbps"]
    K100,
    #[doc = "250 kbps"]
    K250,
    #[doc = "400 kbps"]
    K400,
}
impl FREQUENCYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            FREQUENCYW::K100 => 26738688,
            FREQUENCYW::K250 => 67108864,
            FREQUENCYW::K400 => 104857600,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQUENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQUENCYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQUENCYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "100 kbps"]
    #[inline]
    pub fn k100(self) -> &'a mut W {
        self.variant(FREQUENCYW::K100)
    }
    #[doc = "250 kbps"]
    #[inline]
    pub fn k250(self) -> &'a mut W {
        self.variant(FREQUENCYW::K250)
    }
    #[doc = "400 kbps"]
    #[inline]
    pub fn k400(self) -> &'a mut W {
        self.variant(FREQUENCYW::K400)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - TWI master clock frequency"]
    #[inline]
    pub fn frequency(&self) -> FREQUENCYR {
        FREQUENCYR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 67108864 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - TWI master clock frequency"]
    #[inline]
    pub fn frequency(&mut self) -> _FREQUENCYW {
        _FREQUENCYW { w: self }
    }
}
