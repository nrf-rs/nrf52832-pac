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
#[doc = "Possible values of the field `WEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WENR {
    #[doc = "Read only access."]
    REN,
    #[doc = "Write enabled."]
    WEN,
    #[doc = "Erase enabled."]
    EEN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WENR::REN => 0,
            WENR::WEN => 1,
            WENR::EEN => 2,
            WENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WENR {
        match value {
            0 => WENR::REN,
            1 => WENR::WEN,
            2 => WENR::EEN,
            i => WENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REN`"]
    #[inline]
    pub fn is_ren(&self) -> bool {
        *self == WENR::REN
    }
    #[doc = "Checks if the value of the field is `WEN`"]
    #[inline]
    pub fn is_wen(&self) -> bool {
        *self == WENR::WEN
    }
    #[doc = "Checks if the value of the field is `EEN`"]
    #[inline]
    pub fn is_een(&self) -> bool {
        *self == WENR::EEN
    }
}
#[doc = "Values that can be written to the field `WEN`"]
pub enum WENW {
    #[doc = "Read only access."]
    REN,
    #[doc = "Write enabled."]
    WEN,
    #[doc = "Erase enabled."]
    EEN,
}
impl WENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WENW::REN => 0,
            WENW::WEN => 1,
            WENW::EEN => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WENW<'a> {
    w: &'a mut W,
}
impl<'a> _WENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Read only access."]
    #[inline]
    pub fn ren(self) -> &'a mut W {
        self.variant(WENW::REN)
    }
    #[doc = "Write enabled."]
    #[inline]
    pub fn wen(self) -> &'a mut W {
        self.variant(WENW::WEN)
    }
    #[doc = "Erase enabled."]
    #[inline]
    pub fn een(self) -> &'a mut W {
        self.variant(WENW::EEN)
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
    #[doc = "Bits 0:1 - Program write enable."]
    #[inline]
    pub fn wen(&self) -> WENR {
        WENR::_from({
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
    #[doc = "Bits 0:1 - Program write enable."]
    #[inline]
    pub fn wen(&mut self) -> _WENW {
        _WENW { w: self }
    }
}
