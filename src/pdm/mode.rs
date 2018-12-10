#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE {
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
#[doc = "Possible values of the field `OPERATION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPERATIONR {
    #[doc = "Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    STEREO,
    #[doc = "Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    MONO,
}
impl OPERATIONR {
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
            OPERATIONR::STEREO => false,
            OPERATIONR::MONO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OPERATIONR {
        match value {
            false => OPERATIONR::STEREO,
            true => OPERATIONR::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline]
    pub fn is_stereo(&self) -> bool {
        *self == OPERATIONR::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline]
    pub fn is_mono(&self) -> bool {
        *self == OPERATIONR::MONO
    }
}
#[doc = "Possible values of the field `EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGER {
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    LEFTFALLING,
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    LEFTRISING,
}
impl EDGER {
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
            EDGER::LEFTFALLING => false,
            EDGER::LEFTRISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGER {
        match value {
            false => EDGER::LEFTFALLING,
            true => EDGER::LEFTRISING,
        }
    }
    #[doc = "Checks if the value of the field is `LEFTFALLING`"]
    #[inline]
    pub fn is_left_falling(&self) -> bool {
        *self == EDGER::LEFTFALLING
    }
    #[doc = "Checks if the value of the field is `LEFTRISING`"]
    #[inline]
    pub fn is_left_rising(&self) -> bool {
        *self == EDGER::LEFTRISING
    }
}
#[doc = "Values that can be written to the field `OPERATION`"]
pub enum OPERATIONW {
    #[doc = "Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    STEREO,
    #[doc = "Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    MONO,
}
impl OPERATIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OPERATIONW::STEREO => false,
            OPERATIONW::MONO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPERATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _OPERATIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPERATIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    #[inline]
    pub fn stereo(self) -> &'a mut W {
        self.variant(OPERATIONW::STEREO)
    }
    #[doc = "Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    #[inline]
    pub fn mono(self) -> &'a mut W {
        self.variant(OPERATIONW::MONO)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGE`"]
pub enum EDGEW {
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    LEFTFALLING,
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    LEFTRISING,
}
impl EDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGEW::LEFTFALLING => false,
            EDGEW::LEFTRISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    #[inline]
    pub fn left_falling(self) -> &'a mut W {
        self.variant(EDGEW::LEFTFALLING)
    }
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    #[inline]
    pub fn left_rising(self) -> &'a mut W {
        self.variant(EDGEW::LEFTRISING)
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
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Mono or stereo operation"]
    #[inline]
    pub fn operation(&self) -> OPERATIONR {
        OPERATIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Defines on which PDM_CLK edge Left (or mono) is sampled"]
    #[inline]
    pub fn edge(&self) -> EDGER {
        EDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Mono or stereo operation"]
    #[inline]
    pub fn operation(&mut self) -> _OPERATIONW {
        _OPERATIONW { w: self }
    }
    #[doc = "Bit 1 - Defines on which PDM_CLK edge Left (or mono) is sampled"]
    #[inline]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
}
