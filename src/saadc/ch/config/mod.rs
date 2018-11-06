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
#[doc = "Possible values of the field `RESP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPR {
    #[doc = "Bypass resistor ladder"]
    BYPASS,
    #[doc = "Pull-down to GND"]
    PULLDOWN,
    #[doc = "Pull-up to VDD"]
    PULLUP,
    #[doc = "Set input at VDD/2"]
    VDD1_2,
}
impl RESPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESPR::BYPASS => 0,
            RESPR::PULLDOWN => 1,
            RESPR::PULLUP => 2,
            RESPR::VDD1_2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESPR {
        match value {
            0 => RESPR::BYPASS,
            1 => RESPR::PULLDOWN,
            2 => RESPR::PULLUP,
            3 => RESPR::VDD1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == RESPR::BYPASS
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline]
    pub fn is_pulldown(&self) -> bool {
        *self == RESPR::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline]
    pub fn is_pullup(&self) -> bool {
        *self == RESPR::PULLUP
    }
    #[doc = "Checks if the value of the field is `VDD1_2`"]
    #[inline]
    pub fn is_vdd1_2(&self) -> bool {
        *self == RESPR::VDD1_2
    }
}
#[doc = "Possible values of the field `RESN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESNR {
    #[doc = "Bypass resistor ladder"]
    BYPASS,
    #[doc = "Pull-down to GND"]
    PULLDOWN,
    #[doc = "Pull-up to VDD"]
    PULLUP,
    #[doc = "Set input at VDD/2"]
    VDD1_2,
}
impl RESNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESNR::BYPASS => 0,
            RESNR::PULLDOWN => 1,
            RESNR::PULLUP => 2,
            RESNR::VDD1_2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESNR {
        match value {
            0 => RESNR::BYPASS,
            1 => RESNR::PULLDOWN,
            2 => RESNR::PULLUP,
            3 => RESNR::VDD1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == RESNR::BYPASS
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline]
    pub fn is_pulldown(&self) -> bool {
        *self == RESNR::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline]
    pub fn is_pullup(&self) -> bool {
        *self == RESNR::PULLUP
    }
    #[doc = "Checks if the value of the field is `VDD1_2`"]
    #[inline]
    pub fn is_vdd1_2(&self) -> bool {
        *self == RESNR::VDD1_2
    }
}
#[doc = "Possible values of the field `GAIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAINR {
    #[doc = "1/6"]
    GAIN1_6,
    #[doc = "1/5"]
    GAIN1_5,
    #[doc = "1/4"]
    GAIN1_4,
    #[doc = "1/3"]
    GAIN1_3,
    #[doc = "1/2"]
    GAIN1_2,
    #[doc = "1"]
    GAIN1,
    #[doc = "2"]
    GAIN2,
    #[doc = "4"]
    GAIN4,
}
impl GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAINR::GAIN1_6 => 0,
            GAINR::GAIN1_5 => 1,
            GAINR::GAIN1_4 => 2,
            GAINR::GAIN1_3 => 3,
            GAINR::GAIN1_2 => 4,
            GAINR::GAIN1 => 5,
            GAINR::GAIN2 => 6,
            GAINR::GAIN4 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAINR {
        match value {
            0 => GAINR::GAIN1_6,
            1 => GAINR::GAIN1_5,
            2 => GAINR::GAIN1_4,
            3 => GAINR::GAIN1_3,
            4 => GAINR::GAIN1_2,
            5 => GAINR::GAIN1,
            6 => GAINR::GAIN2,
            7 => GAINR::GAIN4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GAIN1_6`"]
    #[inline]
    pub fn is_gain1_6(&self) -> bool {
        *self == GAINR::GAIN1_6
    }
    #[doc = "Checks if the value of the field is `GAIN1_5`"]
    #[inline]
    pub fn is_gain1_5(&self) -> bool {
        *self == GAINR::GAIN1_5
    }
    #[doc = "Checks if the value of the field is `GAIN1_4`"]
    #[inline]
    pub fn is_gain1_4(&self) -> bool {
        *self == GAINR::GAIN1_4
    }
    #[doc = "Checks if the value of the field is `GAIN1_3`"]
    #[inline]
    pub fn is_gain1_3(&self) -> bool {
        *self == GAINR::GAIN1_3
    }
    #[doc = "Checks if the value of the field is `GAIN1_2`"]
    #[inline]
    pub fn is_gain1_2(&self) -> bool {
        *self == GAINR::GAIN1_2
    }
    #[doc = "Checks if the value of the field is `GAIN1`"]
    #[inline]
    pub fn is_gain1(&self) -> bool {
        *self == GAINR::GAIN1
    }
    #[doc = "Checks if the value of the field is `GAIN2`"]
    #[inline]
    pub fn is_gain2(&self) -> bool {
        *self == GAINR::GAIN2
    }
    #[doc = "Checks if the value of the field is `GAIN4`"]
    #[inline]
    pub fn is_gain4(&self) -> bool {
        *self == GAINR::GAIN4
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Internal reference (0.6 V)"]
    INTERNAL,
    #[doc = "VDD/4 as reference"]
    VDD1_4,
}
impl REFSELR {
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
            REFSELR::INTERNAL => false,
            REFSELR::VDD1_4 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFSELR {
        match value {
            false => REFSELR::INTERNAL,
            true => REFSELR::VDD1_4,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline]
    pub fn is_internal(&self) -> bool {
        *self == REFSELR::INTERNAL
    }
    #[doc = "Checks if the value of the field is `VDD1_4`"]
    #[inline]
    pub fn is_vdd1_4(&self) -> bool {
        *self == REFSELR::VDD1_4
    }
}
#[doc = "Possible values of the field `TACQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACQR {
    #[doc = "3 us"]
    _3US,
    #[doc = "5 us"]
    _5US,
    #[doc = "10 us"]
    _10US,
    #[doc = "15 us"]
    _15US,
    #[doc = "20 us"]
    _20US,
    #[doc = "40 us"]
    _40US,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TACQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TACQR::_3US => 0,
            TACQR::_5US => 1,
            TACQR::_10US => 2,
            TACQR::_15US => 3,
            TACQR::_20US => 4,
            TACQR::_40US => 5,
            TACQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TACQR {
        match value {
            0 => TACQR::_3US,
            1 => TACQR::_5US,
            2 => TACQR::_10US,
            3 => TACQR::_15US,
            4 => TACQR::_20US,
            5 => TACQR::_40US,
            i => TACQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_3US`"]
    #[inline]
    pub fn is_3us(&self) -> bool {
        *self == TACQR::_3US
    }
    #[doc = "Checks if the value of the field is `_5US`"]
    #[inline]
    pub fn is_5us(&self) -> bool {
        *self == TACQR::_5US
    }
    #[doc = "Checks if the value of the field is `_10US`"]
    #[inline]
    pub fn is_10us(&self) -> bool {
        *self == TACQR::_10US
    }
    #[doc = "Checks if the value of the field is `_15US`"]
    #[inline]
    pub fn is_15us(&self) -> bool {
        *self == TACQR::_15US
    }
    #[doc = "Checks if the value of the field is `_20US`"]
    #[inline]
    pub fn is_20us(&self) -> bool {
        *self == TACQR::_20US
    }
    #[doc = "Checks if the value of the field is `_40US`"]
    #[inline]
    pub fn is_40us(&self) -> bool {
        *self == TACQR::_40US
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
    SE,
    #[doc = "Differential"]
    DIFF,
}
impl MODER {
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
            MODER::SE => false,
            MODER::DIFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::SE,
            true => MODER::DIFF,
        }
    }
    #[doc = "Checks if the value of the field is `SE`"]
    #[inline]
    pub fn is_se(&self) -> bool {
        *self == MODER::SE
    }
    #[doc = "Checks if the value of the field is `DIFF`"]
    #[inline]
    pub fn is_diff(&self) -> bool {
        *self == MODER::DIFF
    }
}
#[doc = "Possible values of the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTR {
    #[doc = "Burst mode is disabled (normal operation)"]
    DISABLED,
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    ENABLED,
}
impl BURSTR {
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
            BURSTR::DISABLED => false,
            BURSTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTR {
        match value {
            false => BURSTR::DISABLED,
            true => BURSTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BURSTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BURSTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RESP`"]
pub enum RESPW {
    #[doc = "Bypass resistor ladder"]
    BYPASS,
    #[doc = "Pull-down to GND"]
    PULLDOWN,
    #[doc = "Pull-up to VDD"]
    PULLUP,
    #[doc = "Set input at VDD/2"]
    VDD1_2,
}
impl RESPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESPW::BYPASS => 0,
            RESPW::PULLDOWN => 1,
            RESPW::PULLUP => 2,
            RESPW::VDD1_2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESPW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass resistor ladder"]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(RESPW::BYPASS)
    }
    #[doc = "Pull-down to GND"]
    #[inline]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(RESPW::PULLDOWN)
    }
    #[doc = "Pull-up to VDD"]
    #[inline]
    pub fn pullup(self) -> &'a mut W {
        self.variant(RESPW::PULLUP)
    }
    #[doc = "Set input at VDD/2"]
    #[inline]
    pub fn vdd1_2(self) -> &'a mut W {
        self.variant(RESPW::VDD1_2)
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
#[doc = "Values that can be written to the field `RESN`"]
pub enum RESNW {
    #[doc = "Bypass resistor ladder"]
    BYPASS,
    #[doc = "Pull-down to GND"]
    PULLDOWN,
    #[doc = "Pull-up to VDD"]
    PULLUP,
    #[doc = "Set input at VDD/2"]
    VDD1_2,
}
impl RESNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESNW::BYPASS => 0,
            RESNW::PULLDOWN => 1,
            RESNW::PULLUP => 2,
            RESNW::VDD1_2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESNW<'a> {
    w: &'a mut W,
}
impl<'a> _RESNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass resistor ladder"]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(RESNW::BYPASS)
    }
    #[doc = "Pull-down to GND"]
    #[inline]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(RESNW::PULLDOWN)
    }
    #[doc = "Pull-up to VDD"]
    #[inline]
    pub fn pullup(self) -> &'a mut W {
        self.variant(RESNW::PULLUP)
    }
    #[doc = "Set input at VDD/2"]
    #[inline]
    pub fn vdd1_2(self) -> &'a mut W {
        self.variant(RESNW::VDD1_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GAIN`"]
pub enum GAINW {
    #[doc = "1/6"]
    GAIN1_6,
    #[doc = "1/5"]
    GAIN1_5,
    #[doc = "1/4"]
    GAIN1_4,
    #[doc = "1/3"]
    GAIN1_3,
    #[doc = "1/2"]
    GAIN1_2,
    #[doc = "1"]
    GAIN1,
    #[doc = "2"]
    GAIN2,
    #[doc = "4"]
    GAIN4,
}
impl GAINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAINW::GAIN1_6 => 0,
            GAINW::GAIN1_5 => 1,
            GAINW::GAIN1_4 => 2,
            GAINW::GAIN1_3 => 3,
            GAINW::GAIN1_2 => 4,
            GAINW::GAIN1 => 5,
            GAINW::GAIN2 => 6,
            GAINW::GAIN4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAINW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1/6"]
    #[inline]
    pub fn gain1_6(self) -> &'a mut W {
        self.variant(GAINW::GAIN1_6)
    }
    #[doc = "1/5"]
    #[inline]
    pub fn gain1_5(self) -> &'a mut W {
        self.variant(GAINW::GAIN1_5)
    }
    #[doc = "1/4"]
    #[inline]
    pub fn gain1_4(self) -> &'a mut W {
        self.variant(GAINW::GAIN1_4)
    }
    #[doc = "1/3"]
    #[inline]
    pub fn gain1_3(self) -> &'a mut W {
        self.variant(GAINW::GAIN1_3)
    }
    #[doc = "1/2"]
    #[inline]
    pub fn gain1_2(self) -> &'a mut W {
        self.variant(GAINW::GAIN1_2)
    }
    #[doc = "1"]
    #[inline]
    pub fn gain1(self) -> &'a mut W {
        self.variant(GAINW::GAIN1)
    }
    #[doc = "2"]
    #[inline]
    pub fn gain2(self) -> &'a mut W {
        self.variant(GAINW::GAIN2)
    }
    #[doc = "4"]
    #[inline]
    pub fn gain4(self) -> &'a mut W {
        self.variant(GAINW::GAIN4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Internal reference (0.6 V)"]
    INTERNAL,
    #[doc = "VDD/4 as reference"]
    VDD1_4,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFSELW::INTERNAL => false,
            REFSELW::VDD1_4 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal reference (0.6 V)"]
    #[inline]
    pub fn internal(self) -> &'a mut W {
        self.variant(REFSELW::INTERNAL)
    }
    #[doc = "VDD/4 as reference"]
    #[inline]
    pub fn vdd1_4(self) -> &'a mut W {
        self.variant(REFSELW::VDD1_4)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TACQ`"]
pub enum TACQW {
    #[doc = "3 us"]
    _3US,
    #[doc = "5 us"]
    _5US,
    #[doc = "10 us"]
    _10US,
    #[doc = "15 us"]
    _15US,
    #[doc = "20 us"]
    _20US,
    #[doc = "40 us"]
    _40US,
}
impl TACQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TACQW::_3US => 0,
            TACQW::_5US => 1,
            TACQW::_10US => 2,
            TACQW::_15US => 3,
            TACQW::_20US => 4,
            TACQW::_40US => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TACQW<'a> {
    w: &'a mut W,
}
impl<'a> _TACQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TACQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "3 us"]
    #[inline]
    pub fn _3us(self) -> &'a mut W {
        self.variant(TACQW::_3US)
    }
    #[doc = "5 us"]
    #[inline]
    pub fn _5us(self) -> &'a mut W {
        self.variant(TACQW::_5US)
    }
    #[doc = "10 us"]
    #[inline]
    pub fn _10us(self) -> &'a mut W {
        self.variant(TACQW::_10US)
    }
    #[doc = "15 us"]
    #[inline]
    pub fn _15us(self) -> &'a mut W {
        self.variant(TACQW::_15US)
    }
    #[doc = "20 us"]
    #[inline]
    pub fn _20us(self) -> &'a mut W {
        self.variant(TACQW::_20US)
    }
    #[doc = "40 us"]
    #[inline]
    pub fn _40us(self) -> &'a mut W {
        self.variant(TACQW::_40US)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
    SE,
    #[doc = "Differential"]
    DIFF,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::SE => false,
            MODEW::DIFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
    #[inline]
    pub fn se(self) -> &'a mut W {
        self.variant(MODEW::SE)
    }
    #[doc = "Differential"]
    #[inline]
    pub fn diff(self) -> &'a mut W {
        self.variant(MODEW::DIFF)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURST`"]
pub enum BURSTW {
    #[doc = "Burst mode is disabled (normal operation)"]
    DISABLED,
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    ENABLED,
}
impl BURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTW::DISABLED => false,
            BURSTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Burst mode is disabled (normal operation)"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BURSTW::DISABLED)
    }
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BURSTW::ENABLED)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - Positive channel resistor control"]
    #[inline]
    pub fn resp(&self) -> RESPR {
        RESPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Negative channel resistor control"]
    #[inline]
    pub fn resn(&self) -> RESNR {
        RESNR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Gain control"]
    #[inline]
    pub fn gain(&self) -> GAINR {
        GAINR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Reference control"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Acquisition time, the time the ADC uses to sample the input voltage"]
    #[inline]
    pub fn tacq(&self) -> TACQR {
        TACQR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Enable differential mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable burst mode"]
    #[inline]
    pub fn burst(&self) -> BURSTR {
        BURSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 131072 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Positive channel resistor control"]
    #[inline]
    pub fn resp(&mut self) -> _RESPW {
        _RESPW { w: self }
    }
    #[doc = "Bits 4:5 - Negative channel resistor control"]
    #[inline]
    pub fn resn(&mut self) -> _RESNW {
        _RESNW { w: self }
    }
    #[doc = "Bits 8:10 - Gain control"]
    #[inline]
    pub fn gain(&mut self) -> _GAINW {
        _GAINW { w: self }
    }
    #[doc = "Bit 12 - Reference control"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bits 16:18 - Acquisition time, the time the ADC uses to sample the input voltage"]
    #[inline]
    pub fn tacq(&mut self) -> _TACQW {
        _TACQW { w: self }
    }
    #[doc = "Bit 20 - Enable differential mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 24 - Enable burst mode"]
    #[inline]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
}
