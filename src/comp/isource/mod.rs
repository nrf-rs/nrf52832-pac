#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISOURCE {
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
#[doc = "Possible values of the field `ISOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOURCER {
    #[doc = "Current source disabled"]
    OFF,
    #[doc = "Current source enabled (+/- 2.5 uA)"]
    IEN2MA5,
    #[doc = "Current source enabled (+/- 5 uA)"]
    IEN5MA,
    #[doc = "Current source enabled (+/- 10 uA)"]
    IEN10MA,
}
impl ISOURCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISOURCER::OFF => 0,
            ISOURCER::IEN2MA5 => 1,
            ISOURCER::IEN5MA => 2,
            ISOURCER::IEN10MA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISOURCER {
        match value {
            0 => ISOURCER::OFF,
            1 => ISOURCER::IEN2MA5,
            2 => ISOURCER::IEN5MA,
            3 => ISOURCER::IEN10MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == ISOURCER::OFF
    }
    #[doc = "Checks if the value of the field is `IEN2MA5`"]
    #[inline]
    pub fn is_ien2m_a5(&self) -> bool {
        *self == ISOURCER::IEN2MA5
    }
    #[doc = "Checks if the value of the field is `IEN5MA`"]
    #[inline]
    pub fn is_ien5m_a(&self) -> bool {
        *self == ISOURCER::IEN5MA
    }
    #[doc = "Checks if the value of the field is `IEN10MA`"]
    #[inline]
    pub fn is_ien10m_a(&self) -> bool {
        *self == ISOURCER::IEN10MA
    }
}
#[doc = "Values that can be written to the field `ISOURCE`"]
pub enum ISOURCEW {
    #[doc = "Current source disabled"]
    OFF,
    #[doc = "Current source enabled (+/- 2.5 uA)"]
    IEN2MA5,
    #[doc = "Current source enabled (+/- 5 uA)"]
    IEN5MA,
    #[doc = "Current source enabled (+/- 10 uA)"]
    IEN10MA,
}
impl ISOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ISOURCEW::OFF => 0,
            ISOURCEW::IEN2MA5 => 1,
            ISOURCEW::IEN5MA => 2,
            ISOURCEW::IEN10MA => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _ISOURCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISOURCEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Current source disabled"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(ISOURCEW::OFF)
    }
    #[doc = "Current source enabled (+/- 2.5 uA)"]
    #[inline]
    pub fn ien2m_a5(self) -> &'a mut W {
        self.variant(ISOURCEW::IEN2MA5)
    }
    #[doc = "Current source enabled (+/- 5 uA)"]
    #[inline]
    pub fn ien5m_a(self) -> &'a mut W {
        self.variant(ISOURCEW::IEN5MA)
    }
    #[doc = "Current source enabled (+/- 10 uA)"]
    #[inline]
    pub fn ien10m_a(self) -> &'a mut W {
        self.variant(ISOURCEW::IEN10MA)
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
    #[doc = "Bits 0:1 - Comparator hysteresis"]
    #[inline]
    pub fn isource(&self) -> ISOURCER {
        ISOURCER::_from({
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
    #[doc = "Bits 0:1 - Comparator hysteresis"]
    #[inline]
    pub fn isource(&mut self) -> _ISOURCEW {
        _ISOURCEW { w: self }
    }
}
