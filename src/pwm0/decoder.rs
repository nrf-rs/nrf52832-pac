#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DECODER {
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
#[doc = "Possible values of the field `LOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOADR {
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    COMMON,
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    GROUPED,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    INDIVIDUAL,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    WAVEFORM,
}
impl LOADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOADR::COMMON => 0,
            LOADR::GROUPED => 1,
            LOADR::INDIVIDUAL => 2,
            LOADR::WAVEFORM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOADR {
        match value {
            0 => LOADR::COMMON,
            1 => LOADR::GROUPED,
            2 => LOADR::INDIVIDUAL,
            3 => LOADR::WAVEFORM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMMON`"]
    #[inline]
    pub fn is_common(&self) -> bool {
        *self == LOADR::COMMON
    }
    #[doc = "Checks if the value of the field is `GROUPED`"]
    #[inline]
    pub fn is_grouped(&self) -> bool {
        *self == LOADR::GROUPED
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL`"]
    #[inline]
    pub fn is_individual(&self) -> bool {
        *self == LOADR::INDIVIDUAL
    }
    #[doc = "Checks if the value of the field is `WAVEFORM`"]
    #[inline]
    pub fn is_wave_form(&self) -> bool {
        *self == LOADR::WAVEFORM
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    REFRESHCOUNT,
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    NEXTSTEP,
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
            MODER::REFRESHCOUNT => false,
            MODER::NEXTSTEP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::REFRESHCOUNT,
            true => MODER::NEXTSTEP,
        }
    }
    #[doc = "Checks if the value of the field is `REFRESHCOUNT`"]
    #[inline]
    pub fn is_refresh_count(&self) -> bool {
        *self == MODER::REFRESHCOUNT
    }
    #[doc = "Checks if the value of the field is `NEXTSTEP`"]
    #[inline]
    pub fn is_next_step(&self) -> bool {
        *self == MODER::NEXTSTEP
    }
}
#[doc = "Values that can be written to the field `LOAD`"]
pub enum LOADW {
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    COMMON,
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    GROUPED,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    INDIVIDUAL,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    WAVEFORM,
}
impl LOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOADW::COMMON => 0,
            LOADW::GROUPED => 1,
            LOADW::INDIVIDUAL => 2,
            LOADW::WAVEFORM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _LOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOADW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    #[inline]
    pub fn common(self) -> &'a mut W {
        self.variant(LOADW::COMMON)
    }
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    #[inline]
    pub fn grouped(self) -> &'a mut W {
        self.variant(LOADW::GROUPED)
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    #[inline]
    pub fn individual(self) -> &'a mut W {
        self.variant(LOADW::INDIVIDUAL)
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    #[inline]
    pub fn wave_form(self) -> &'a mut W {
        self.variant(LOADW::WAVEFORM)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    REFRESHCOUNT,
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    NEXTSTEP,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::REFRESHCOUNT => false,
            MODEW::NEXTSTEP => true,
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
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    #[inline]
    pub fn refresh_count(self) -> &'a mut W {
        self.variant(MODEW::REFRESHCOUNT)
    }
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    #[inline]
    pub fn next_step(self) -> &'a mut W {
        self.variant(MODEW::NEXTSTEP)
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
    #[doc = "Bits 0:1 - How a sequence is read from RAM and spread to the compare register"]
    #[inline]
    pub fn load(&self) -> LOADR {
        LOADR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Selects source for advancing the active sequence"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - How a sequence is read from RAM and spread to the compare register"]
    #[inline]
    pub fn load(&mut self) -> _LOADW {
        _LOADW { w: self }
    }
    #[doc = "Bit 8 - Selects source for advancing the active sequence"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
