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
#[doc = r" Value of the field"]
pub struct FREQUENCYR {
    bits: u8,
}
impl FREQUENCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAPR {
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    DEFAULT,
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    LOW,
}
impl MAPR {
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
            MAPR::DEFAULT => false,
            MAPR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAPR {
        match value {
            false => MAPR::DEFAULT,
            true => MAPR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == MAPR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == MAPR::LOW
    }
}
#[doc = r" Proxy"]
pub struct _FREQUENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQUENCYW<'a> {
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
#[doc = "Values that can be written to the field `MAP`"]
pub enum MAPW {
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    DEFAULT,
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    LOW,
}
impl MAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAPW::DEFAULT => false,
            MAPW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAPW<'a> {
    w: &'a mut W,
}
impl<'a> _MAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(MAPW::DEFAULT)
    }
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(MAPW::LOW)
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline]
    pub fn frequency(&self) -> FREQUENCYR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FREQUENCYR { bits }
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline]
    pub fn map(&self) -> MAPR {
        MAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline]
    pub fn frequency(&mut self) -> _FREQUENCYW {
        _FREQUENCYW { w: self }
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline]
    pub fn map(&mut self) -> _MAPW {
        _MAPW { w: self }
    }
}
