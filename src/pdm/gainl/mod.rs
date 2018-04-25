#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GAINL {
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
#[doc = "Possible values of the field `GAINL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAINLR {
    #[doc = "-20dB gain adjustment (minimum)"]
    MINGAIN,
    #[doc = "0dB gain adjustment ('2500 RMS' requirement)"]
    DEFAULTGAIN,
    #[doc = "+20dB gain adjustment (maximum)"]
    MAXGAIN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GAINLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAINLR::MINGAIN => 0,
            GAINLR::DEFAULTGAIN => 40,
            GAINLR::MAXGAIN => 80,
            GAINLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAINLR {
        match value {
            0 => GAINLR::MINGAIN,
            40 => GAINLR::DEFAULTGAIN,
            80 => GAINLR::MAXGAIN,
            i => GAINLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MINGAIN`"]
    #[inline]
    pub fn is_min_gain(&self) -> bool {
        *self == GAINLR::MINGAIN
    }
    #[doc = "Checks if the value of the field is `DEFAULTGAIN`"]
    #[inline]
    pub fn is_default_gain(&self) -> bool {
        *self == GAINLR::DEFAULTGAIN
    }
    #[doc = "Checks if the value of the field is `MAXGAIN`"]
    #[inline]
    pub fn is_max_gain(&self) -> bool {
        *self == GAINLR::MAXGAIN
    }
}
#[doc = "Values that can be written to the field `GAINL`"]
pub enum GAINLW {
    #[doc = "-20dB gain adjustment (minimum)"]
    MINGAIN,
    #[doc = "0dB gain adjustment ('2500 RMS' requirement)"]
    DEFAULTGAIN,
    #[doc = "+20dB gain adjustment (maximum)"]
    MAXGAIN,
}
impl GAINLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAINLW::MINGAIN => 0,
            GAINLW::DEFAULTGAIN => 40,
            GAINLW::MAXGAIN => 80,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAINLW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAINLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "-20dB gain adjustment (minimum)"]
    #[inline]
    pub fn min_gain(self) -> &'a mut W {
        self.variant(GAINLW::MINGAIN)
    }
    #[doc = "0dB gain adjustment ('2500 RMS' requirement)"]
    #[inline]
    pub fn default_gain(self) -> &'a mut W {
        self.variant(GAINLW::DEFAULTGAIN)
    }
    #[doc = "+20dB gain adjustment (maximum)"]
    #[inline]
    pub fn max_gain(self) -> &'a mut W {
        self.variant(GAINLW::MAXGAIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
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
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline]
    pub fn gainl(&self) -> GAINLR {
        GAINLR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 40 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline]
    pub fn gainl(&mut self) -> _GAINLW {
        _GAINLW { w: self }
    }
}
