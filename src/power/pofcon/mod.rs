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
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
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
    #[doc = "Set threshold to 1.7 V"]
    V17,
    #[doc = "Set threshold to 1.8 V"]
    V18,
    #[doc = "Set threshold to 1.9 V"]
    V19,
    #[doc = "Set threshold to 2.0 V"]
    V20,
    #[doc = "Set threshold to 2.1 V"]
    V21,
    #[doc = "Set threshold to 2.2 V"]
    V22,
    #[doc = "Set threshold to 2.3 V"]
    V23,
    #[doc = "Set threshold to 2.4 V"]
    V24,
    #[doc = "Set threshold to 2.5 V"]
    V25,
    #[doc = "Set threshold to 2.6 V"]
    V26,
    #[doc = "Set threshold to 2.7 V"]
    V27,
    #[doc = "Set threshold to 2.8 V"]
    V28,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl THRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            THRESHOLDR::V17 => 4,
            THRESHOLDR::V18 => 5,
            THRESHOLDR::V19 => 6,
            THRESHOLDR::V20 => 7,
            THRESHOLDR::V21 => 8,
            THRESHOLDR::V22 => 9,
            THRESHOLDR::V23 => 10,
            THRESHOLDR::V24 => 11,
            THRESHOLDR::V25 => 12,
            THRESHOLDR::V26 => 13,
            THRESHOLDR::V27 => 14,
            THRESHOLDR::V28 => 15,
            THRESHOLDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> THRESHOLDR {
        match value {
            4 => THRESHOLDR::V17,
            5 => THRESHOLDR::V18,
            6 => THRESHOLDR::V19,
            7 => THRESHOLDR::V20,
            8 => THRESHOLDR::V21,
            9 => THRESHOLDR::V22,
            10 => THRESHOLDR::V23,
            11 => THRESHOLDR::V24,
            12 => THRESHOLDR::V25,
            13 => THRESHOLDR::V26,
            14 => THRESHOLDR::V27,
            15 => THRESHOLDR::V28,
            i => THRESHOLDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `V17`"]
    #[inline]
    pub fn is_v17(&self) -> bool {
        *self == THRESHOLDR::V17
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline]
    pub fn is_v18(&self) -> bool {
        *self == THRESHOLDR::V18
    }
    #[doc = "Checks if the value of the field is `V19`"]
    #[inline]
    pub fn is_v19(&self) -> bool {
        *self == THRESHOLDR::V19
    }
    #[doc = "Checks if the value of the field is `V20`"]
    #[inline]
    pub fn is_v20(&self) -> bool {
        *self == THRESHOLDR::V20
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline]
    pub fn is_v21(&self) -> bool {
        *self == THRESHOLDR::V21
    }
    #[doc = "Checks if the value of the field is `V22`"]
    #[inline]
    pub fn is_v22(&self) -> bool {
        *self == THRESHOLDR::V22
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline]
    pub fn is_v23(&self) -> bool {
        *self == THRESHOLDR::V23
    }
    #[doc = "Checks if the value of the field is `V24`"]
    #[inline]
    pub fn is_v24(&self) -> bool {
        *self == THRESHOLDR::V24
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline]
    pub fn is_v25(&self) -> bool {
        *self == THRESHOLDR::V25
    }
    #[doc = "Checks if the value of the field is `V26`"]
    #[inline]
    pub fn is_v26(&self) -> bool {
        *self == THRESHOLDR::V26
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLDR::V27
    }
    #[doc = "Checks if the value of the field is `V28`"]
    #[inline]
    pub fn is_v28(&self) -> bool {
        *self == THRESHOLDR::V28
    }
}
#[doc = "Values that can be written to the field `POF`"]
pub enum POFW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
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
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POFW::DISABLED)
    }
    #[doc = "Enable"]
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
    #[doc = "Set threshold to 1.7 V"]
    V17,
    #[doc = "Set threshold to 1.8 V"]
    V18,
    #[doc = "Set threshold to 1.9 V"]
    V19,
    #[doc = "Set threshold to 2.0 V"]
    V20,
    #[doc = "Set threshold to 2.1 V"]
    V21,
    #[doc = "Set threshold to 2.2 V"]
    V22,
    #[doc = "Set threshold to 2.3 V"]
    V23,
    #[doc = "Set threshold to 2.4 V"]
    V24,
    #[doc = "Set threshold to 2.5 V"]
    V25,
    #[doc = "Set threshold to 2.6 V"]
    V26,
    #[doc = "Set threshold to 2.7 V"]
    V27,
    #[doc = "Set threshold to 2.8 V"]
    V28,
}
impl THRESHOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            THRESHOLDW::V17 => 4,
            THRESHOLDW::V18 => 5,
            THRESHOLDW::V19 => 6,
            THRESHOLDW::V20 => 7,
            THRESHOLDW::V21 => 8,
            THRESHOLDW::V22 => 9,
            THRESHOLDW::V23 => 10,
            THRESHOLDW::V24 => 11,
            THRESHOLDW::V25 => 12,
            THRESHOLDW::V26 => 13,
            THRESHOLDW::V27 => 14,
            THRESHOLDW::V28 => 15,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set threshold to 1.7 V"]
    #[inline]
    pub fn v17(self) -> &'a mut W {
        self.variant(THRESHOLDW::V17)
    }
    #[doc = "Set threshold to 1.8 V"]
    #[inline]
    pub fn v18(self) -> &'a mut W {
        self.variant(THRESHOLDW::V18)
    }
    #[doc = "Set threshold to 1.9 V"]
    #[inline]
    pub fn v19(self) -> &'a mut W {
        self.variant(THRESHOLDW::V19)
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline]
    pub fn v20(self) -> &'a mut W {
        self.variant(THRESHOLDW::V20)
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline]
    pub fn v21(self) -> &'a mut W {
        self.variant(THRESHOLDW::V21)
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline]
    pub fn v22(self) -> &'a mut W {
        self.variant(THRESHOLDW::V22)
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline]
    pub fn v23(self) -> &'a mut W {
        self.variant(THRESHOLDW::V23)
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline]
    pub fn v24(self) -> &'a mut W {
        self.variant(THRESHOLDW::V24)
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline]
    pub fn v25(self) -> &'a mut W {
        self.variant(THRESHOLDW::V25)
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline]
    pub fn v26(self) -> &'a mut W {
        self.variant(THRESHOLDW::V26)
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLDW::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline]
    pub fn v28(self) -> &'a mut W {
        self.variant(THRESHOLDW::V28)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 0 - Enable or disable power failure comparator"]
    #[inline]
    pub fn pof(&self) -> POFR {
        POFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:4 - Power failure comparator threshold setting"]
    #[inline]
    pub fn threshold(&self) -> THRESHOLDR {
        THRESHOLDR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bit 0 - Enable or disable power failure comparator"]
    #[inline]
    pub fn pof(&mut self) -> _POFW {
        _POFW { w: self }
    }
    #[doc = "Bits 1:4 - Power failure comparator threshold setting"]
    #[inline]
    pub fn threshold(&mut self) -> _THRESHOLDW {
        _THRESHOLDW { w: self }
    }
}
