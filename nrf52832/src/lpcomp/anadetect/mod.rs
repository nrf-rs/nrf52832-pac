#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANADETECT {
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
#[doc = "Possible values of the field `ANADETECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANADETECTR {
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    CROSS,
    #[doc = "Generate ANADETECT on upward crossing only"]
    UP,
    #[doc = "Generate ANADETECT on downward crossing only"]
    DOWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ANADETECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ANADETECTR::CROSS => 0,
            ANADETECTR::UP => 1,
            ANADETECTR::DOWN => 2,
            ANADETECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ANADETECTR {
        match value {
            0 => ANADETECTR::CROSS,
            1 => ANADETECTR::UP,
            2 => ANADETECTR::DOWN,
            i => ANADETECTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CROSS`"]
    #[inline]
    pub fn is_cross(&self) -> bool {
        *self == ANADETECTR::CROSS
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == ANADETECTR::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline]
    pub fn is_down(&self) -> bool {
        *self == ANADETECTR::DOWN
    }
}
#[doc = "Values that can be written to the field `ANADETECT`"]
pub enum ANADETECTW {
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    CROSS,
    #[doc = "Generate ANADETECT on upward crossing only"]
    UP,
    #[doc = "Generate ANADETECT on downward crossing only"]
    DOWN,
}
impl ANADETECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ANADETECTW::CROSS => 0,
            ANADETECTW::UP => 1,
            ANADETECTW::DOWN => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANADETECTW<'a> {
    w: &'a mut W,
}
impl<'a> _ANADETECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANADETECTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    #[inline]
    pub fn cross(self) -> &'a mut W {
        self.variant(ANADETECTW::CROSS)
    }
    #[doc = "Generate ANADETECT on upward crossing only"]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(ANADETECTW::UP)
    }
    #[doc = "Generate ANADETECT on downward crossing only"]
    #[inline]
    pub fn down(self) -> &'a mut W {
        self.variant(ANADETECTW::DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline]
    pub fn anadetect(&self) -> ANADETECTR {
        ANADETECTR::_from({
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
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline]
    pub fn anadetect(&mut self) -> _ANADETECTW {
        _ANADETECTW { w: self }
    }
}
