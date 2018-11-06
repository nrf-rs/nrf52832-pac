#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OVERSAMPLE {
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
#[doc = "Possible values of the field `OVERSAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERSAMPLER {
    #[doc = "Bypass oversampling"]
    BYPASS,
    #[doc = "Oversample 2x"]
    OVER2X,
    #[doc = "Oversample 4x"]
    OVER4X,
    #[doc = "Oversample 8x"]
    OVER8X,
    #[doc = "Oversample 16x"]
    OVER16X,
    #[doc = "Oversample 32x"]
    OVER32X,
    #[doc = "Oversample 64x"]
    OVER64X,
    #[doc = "Oversample 128x"]
    OVER128X,
    #[doc = "Oversample 256x"]
    OVER256X,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OVERSAMPLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OVERSAMPLER::BYPASS => 0,
            OVERSAMPLER::OVER2X => 1,
            OVERSAMPLER::OVER4X => 2,
            OVERSAMPLER::OVER8X => 3,
            OVERSAMPLER::OVER16X => 4,
            OVERSAMPLER::OVER32X => 5,
            OVERSAMPLER::OVER64X => 6,
            OVERSAMPLER::OVER128X => 7,
            OVERSAMPLER::OVER256X => 8,
            OVERSAMPLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OVERSAMPLER {
        match value {
            0 => OVERSAMPLER::BYPASS,
            1 => OVERSAMPLER::OVER2X,
            2 => OVERSAMPLER::OVER4X,
            3 => OVERSAMPLER::OVER8X,
            4 => OVERSAMPLER::OVER16X,
            5 => OVERSAMPLER::OVER32X,
            6 => OVERSAMPLER::OVER64X,
            7 => OVERSAMPLER::OVER128X,
            8 => OVERSAMPLER::OVER256X,
            i => OVERSAMPLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == OVERSAMPLER::BYPASS
    }
    #[doc = "Checks if the value of the field is `OVER2X`"]
    #[inline]
    pub fn is_over2x(&self) -> bool {
        *self == OVERSAMPLER::OVER2X
    }
    #[doc = "Checks if the value of the field is `OVER4X`"]
    #[inline]
    pub fn is_over4x(&self) -> bool {
        *self == OVERSAMPLER::OVER4X
    }
    #[doc = "Checks if the value of the field is `OVER8X`"]
    #[inline]
    pub fn is_over8x(&self) -> bool {
        *self == OVERSAMPLER::OVER8X
    }
    #[doc = "Checks if the value of the field is `OVER16X`"]
    #[inline]
    pub fn is_over16x(&self) -> bool {
        *self == OVERSAMPLER::OVER16X
    }
    #[doc = "Checks if the value of the field is `OVER32X`"]
    #[inline]
    pub fn is_over32x(&self) -> bool {
        *self == OVERSAMPLER::OVER32X
    }
    #[doc = "Checks if the value of the field is `OVER64X`"]
    #[inline]
    pub fn is_over64x(&self) -> bool {
        *self == OVERSAMPLER::OVER64X
    }
    #[doc = "Checks if the value of the field is `OVER128X`"]
    #[inline]
    pub fn is_over128x(&self) -> bool {
        *self == OVERSAMPLER::OVER128X
    }
    #[doc = "Checks if the value of the field is `OVER256X`"]
    #[inline]
    pub fn is_over256x(&self) -> bool {
        *self == OVERSAMPLER::OVER256X
    }
}
#[doc = "Values that can be written to the field `OVERSAMPLE`"]
pub enum OVERSAMPLEW {
    #[doc = "Bypass oversampling"]
    BYPASS,
    #[doc = "Oversample 2x"]
    OVER2X,
    #[doc = "Oversample 4x"]
    OVER4X,
    #[doc = "Oversample 8x"]
    OVER8X,
    #[doc = "Oversample 16x"]
    OVER16X,
    #[doc = "Oversample 32x"]
    OVER32X,
    #[doc = "Oversample 64x"]
    OVER64X,
    #[doc = "Oversample 128x"]
    OVER128X,
    #[doc = "Oversample 256x"]
    OVER256X,
}
impl OVERSAMPLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OVERSAMPLEW::BYPASS => 0,
            OVERSAMPLEW::OVER2X => 1,
            OVERSAMPLEW::OVER4X => 2,
            OVERSAMPLEW::OVER8X => 3,
            OVERSAMPLEW::OVER16X => 4,
            OVERSAMPLEW::OVER32X => 5,
            OVERSAMPLEW::OVER64X => 6,
            OVERSAMPLEW::OVER128X => 7,
            OVERSAMPLEW::OVER256X => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERSAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERSAMPLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERSAMPLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bypass oversampling"]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::BYPASS)
    }
    #[doc = "Oversample 2x"]
    #[inline]
    pub fn over2x(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::OVER2X)
    }
    #[doc = "Oversample 4x"]
    #[inline]
    pub fn over4x(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::OVER4X)
    }
    #[doc = "Oversample 8x"]
    #[inline]
    pub fn over8x(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::OVER8X)
    }
    #[doc = "Oversample 16x"]
    #[inline]
    pub fn over16x(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::OVER16X)
    }
    #[doc = "Oversample 32x"]
    #[inline]
    pub fn over32x(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::OVER32X)
    }
    #[doc = "Oversample 64x"]
    #[inline]
    pub fn over64x(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::OVER64X)
    }
    #[doc = "Oversample 128x"]
    #[inline]
    pub fn over128x(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::OVER128X)
    }
    #[doc = "Oversample 256x"]
    #[inline]
    pub fn over256x(self) -> &'a mut W {
        self.variant(OVERSAMPLEW::OVER256X)
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
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline]
    pub fn oversample(&self) -> OVERSAMPLER {
        OVERSAMPLER::_from({
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
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline]
    pub fn oversample(&mut self) -> _OVERSAMPLEW {
        _OVERSAMPLEW { w: self }
    }
}
