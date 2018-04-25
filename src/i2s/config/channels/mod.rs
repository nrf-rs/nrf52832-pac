#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHANNELS {
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
#[doc = "Possible values of the field `CHANNELS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNELSR {
    #[doc = "Stereo."]
    STEREO,
    #[doc = "Left only."]
    LEFT,
    #[doc = "Right only."]
    RIGHT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHANNELSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHANNELSR::STEREO => 0,
            CHANNELSR::LEFT => 1,
            CHANNELSR::RIGHT => 2,
            CHANNELSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHANNELSR {
        match value {
            0 => CHANNELSR::STEREO,
            1 => CHANNELSR::LEFT,
            2 => CHANNELSR::RIGHT,
            i => CHANNELSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline]
    pub fn is_stereo(&self) -> bool {
        *self == CHANNELSR::STEREO
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline]
    pub fn is_left(&self) -> bool {
        *self == CHANNELSR::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline]
    pub fn is_right(&self) -> bool {
        *self == CHANNELSR::RIGHT
    }
}
#[doc = "Values that can be written to the field `CHANNELS`"]
pub enum CHANNELSW {
    #[doc = "Stereo."]
    STEREO,
    #[doc = "Left only."]
    LEFT,
    #[doc = "Right only."]
    RIGHT,
}
impl CHANNELSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHANNELSW::STEREO => 0,
            CHANNELSW::LEFT => 1,
            CHANNELSW::RIGHT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNELSW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNELSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNELSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stereo."]
    #[inline]
    pub fn stereo(self) -> &'a mut W {
        self.variant(CHANNELSW::STEREO)
    }
    #[doc = "Left only."]
    #[inline]
    pub fn left(self) -> &'a mut W {
        self.variant(CHANNELSW::LEFT)
    }
    #[doc = "Right only."]
    #[inline]
    pub fn right(self) -> &'a mut W {
        self.variant(CHANNELSW::RIGHT)
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
    #[doc = "Bits 0:1 - Enable channels."]
    #[inline]
    pub fn channels(&self) -> CHANNELSR {
        CHANNELSR::_from({
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
    #[doc = "Bits 0:1 - Enable channels."]
    #[inline]
    pub fn channels(&mut self) -> _CHANNELSW {
        _CHANNELSW { w: self }
    }
}
