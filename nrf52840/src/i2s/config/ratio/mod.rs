#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RATIO {
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
#[doc = "Possible values of the field `RATIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATIOR {
    #[doc = "LRCK = MCK / 32"]
    _32X,
    #[doc = "LRCK = MCK / 48"]
    _48X,
    #[doc = "LRCK = MCK / 64"]
    _64X,
    #[doc = "LRCK = MCK / 96"]
    _96X,
    #[doc = "LRCK = MCK / 128"]
    _128X,
    #[doc = "LRCK = MCK / 192"]
    _192X,
    #[doc = "LRCK = MCK / 256"]
    _256X,
    #[doc = "LRCK = MCK / 384"]
    _384X,
    #[doc = "LRCK = MCK / 512"]
    _512X,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RATIOR::_32X => 0,
            RATIOR::_48X => 1,
            RATIOR::_64X => 2,
            RATIOR::_96X => 3,
            RATIOR::_128X => 4,
            RATIOR::_192X => 5,
            RATIOR::_256X => 6,
            RATIOR::_384X => 7,
            RATIOR::_512X => 8,
            RATIOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RATIOR {
        match value {
            0 => RATIOR::_32X,
            1 => RATIOR::_48X,
            2 => RATIOR::_64X,
            3 => RATIOR::_96X,
            4 => RATIOR::_128X,
            5 => RATIOR::_192X,
            6 => RATIOR::_256X,
            7 => RATIOR::_384X,
            8 => RATIOR::_512X,
            i => RATIOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32X`"]
    #[inline]
    pub fn is_32x(&self) -> bool {
        *self == RATIOR::_32X
    }
    #[doc = "Checks if the value of the field is `_48X`"]
    #[inline]
    pub fn is_48x(&self) -> bool {
        *self == RATIOR::_48X
    }
    #[doc = "Checks if the value of the field is `_64X`"]
    #[inline]
    pub fn is_64x(&self) -> bool {
        *self == RATIOR::_64X
    }
    #[doc = "Checks if the value of the field is `_96X`"]
    #[inline]
    pub fn is_96x(&self) -> bool {
        *self == RATIOR::_96X
    }
    #[doc = "Checks if the value of the field is `_128X`"]
    #[inline]
    pub fn is_128x(&self) -> bool {
        *self == RATIOR::_128X
    }
    #[doc = "Checks if the value of the field is `_192X`"]
    #[inline]
    pub fn is_192x(&self) -> bool {
        *self == RATIOR::_192X
    }
    #[doc = "Checks if the value of the field is `_256X`"]
    #[inline]
    pub fn is_256x(&self) -> bool {
        *self == RATIOR::_256X
    }
    #[doc = "Checks if the value of the field is `_384X`"]
    #[inline]
    pub fn is_384x(&self) -> bool {
        *self == RATIOR::_384X
    }
    #[doc = "Checks if the value of the field is `_512X`"]
    #[inline]
    pub fn is_512x(&self) -> bool {
        *self == RATIOR::_512X
    }
}
#[doc = "Values that can be written to the field `RATIO`"]
pub enum RATIOW {
    #[doc = "LRCK = MCK / 32"]
    _32X,
    #[doc = "LRCK = MCK / 48"]
    _48X,
    #[doc = "LRCK = MCK / 64"]
    _64X,
    #[doc = "LRCK = MCK / 96"]
    _96X,
    #[doc = "LRCK = MCK / 128"]
    _128X,
    #[doc = "LRCK = MCK / 192"]
    _192X,
    #[doc = "LRCK = MCK / 256"]
    _256X,
    #[doc = "LRCK = MCK / 384"]
    _384X,
    #[doc = "LRCK = MCK / 512"]
    _512X,
}
impl RATIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RATIOW::_32X => 0,
            RATIOW::_48X => 1,
            RATIOW::_64X => 2,
            RATIOW::_96X => 3,
            RATIOW::_128X => 4,
            RATIOW::_192X => 5,
            RATIOW::_256X => 6,
            RATIOW::_384X => 7,
            RATIOW::_512X => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RATIOW<'a> {
    w: &'a mut W,
}
impl<'a> _RATIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RATIOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LRCK = MCK / 32"]
    #[inline]
    pub fn _32x(self) -> &'a mut W {
        self.variant(RATIOW::_32X)
    }
    #[doc = "LRCK = MCK / 48"]
    #[inline]
    pub fn _48x(self) -> &'a mut W {
        self.variant(RATIOW::_48X)
    }
    #[doc = "LRCK = MCK / 64"]
    #[inline]
    pub fn _64x(self) -> &'a mut W {
        self.variant(RATIOW::_64X)
    }
    #[doc = "LRCK = MCK / 96"]
    #[inline]
    pub fn _96x(self) -> &'a mut W {
        self.variant(RATIOW::_96X)
    }
    #[doc = "LRCK = MCK / 128"]
    #[inline]
    pub fn _128x(self) -> &'a mut W {
        self.variant(RATIOW::_128X)
    }
    #[doc = "LRCK = MCK / 192"]
    #[inline]
    pub fn _192x(self) -> &'a mut W {
        self.variant(RATIOW::_192X)
    }
    #[doc = "LRCK = MCK / 256"]
    #[inline]
    pub fn _256x(self) -> &'a mut W {
        self.variant(RATIOW::_256X)
    }
    #[doc = "LRCK = MCK / 384"]
    #[inline]
    pub fn _384x(self) -> &'a mut W {
        self.variant(RATIOW::_384X)
    }
    #[doc = "LRCK = MCK / 512"]
    #[inline]
    pub fn _512x(self) -> &'a mut W {
        self.variant(RATIOW::_512X)
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
    #[doc = "Bits 0:3 - MCK / LRCK ratio."]
    #[inline]
    pub fn ratio(&self) -> RATIOR {
        RATIOR::_from({
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
        W { bits: 6 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - MCK / LRCK ratio."]
    #[inline]
    pub fn ratio(&mut self) -> _RATIOW {
        _RATIOW { w: self }
    }
}
