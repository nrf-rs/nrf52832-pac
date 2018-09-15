#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POFCON {
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
#[doc = "Possible values of the field `POF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl POFR {
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
            POFR::DISABLED => false,
            POFR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POFR {
        match value {
            false => POFR::DISABLED,
            true => POFR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == POFR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == POFR::ENABLED
    }
}
#[doc = "Possible values of the field `THRESHOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRESHOLDR {
    #[doc = "Set threshold to 2.1Volts."]
    V21,
    #[doc = "Set threshold to 2.3Volts."]
    V23,
    #[doc = "Set threshold to 2.5Volts."]
    V25,
    #[doc = "Set threshold to 2.7Volts."]
    V27,
}
impl THRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            THRESHOLDR::V21 => 0,
            THRESHOLDR::V23 => 1,
            THRESHOLDR::V25 => 2,
            THRESHOLDR::V27 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> THRESHOLDR {
        match value {
            0 => THRESHOLDR::V21,
            1 => THRESHOLDR::V23,
            2 => THRESHOLDR::V25,
            3 => THRESHOLDR::V27,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline]
    pub fn is_v21(&self) -> bool {
        *self == THRESHOLDR::V21
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline]
    pub fn is_v23(&self) -> bool {
        *self == THRESHOLDR::V23
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline]
    pub fn is_v25(&self) -> bool {
        *self == THRESHOLDR::V25
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLDR::V27
    }
}
#[doc = "Values that can be written to the field `POF`"]
pub enum POFW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl POFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POFW::DISABLED => false,
            POFW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POFW<'a> {
    w: &'a mut W,
}
impl<'a> _POFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POFW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POFW::ENABLED)
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
#[doc = "Values that can be written to the field `THRESHOLD`"]
pub enum THRESHOLDW {
    #[doc = "Set threshold to 2.1Volts."]
    V21,
    #[doc = "Set threshold to 2.3Volts."]
    V23,
    #[doc = "Set threshold to 2.5Volts."]
    V25,
    #[doc = "Set threshold to 2.7Volts."]
    V27,
}
impl THRESHOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            THRESHOLDW::V21 => 0,
            THRESHOLDW::V23 => 1,
            THRESHOLDW::V25 => 2,
            THRESHOLDW::V27 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _THRESHOLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THRESHOLDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Set threshold to 2.1Volts."]
    #[inline]
    pub fn v21(self) -> &'a mut W {
        self.variant(THRESHOLDW::V21)
    }
    #[doc = "Set threshold to 2.3Volts."]
    #[inline]
    pub fn v23(self) -> &'a mut W {
        self.variant(THRESHOLDW::V23)
    }
    #[doc = "Set threshold to 2.5Volts."]
    #[inline]
    pub fn v25(self) -> &'a mut W {
        self.variant(THRESHOLDW::V25)
    }
    #[doc = "Set threshold to 2.7Volts."]
    #[inline]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLDW::V27)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Power failure comparator enable."]
    #[inline]
    pub fn pof(&self) -> POFR {
        POFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Set threshold level."]
    #[inline]
    pub fn threshold(&self) -> THRESHOLDR {
        THRESHOLDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Power failure comparator enable."]
    #[inline]
    pub fn pof(&mut self) -> _POFW {
        _POFW { w: self }
    }
    #[doc = "Bits 1:2 - Set threshold level."]
    #[inline]
    pub fn threshold(&mut self) -> _THRESHOLDW {
        _THRESHOLDW { w: self }
    }
}
