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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Channel configure in event mode."]
    EVENT,
    #[doc = "Channel configure in task mode."]
    TASK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::DISABLED => 0,
            MODER::EVENT => 1,
            MODER::TASK => 3,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::DISABLED,
            1 => MODER::EVENT,
            3 => MODER::TASK,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == MODER::EVENT
    }
    #[doc = "Checks if the value of the field is `TASK`"]
    #[inline]
    pub fn is_task(&self) -> bool {
        *self == MODER::TASK
    }
}
#[doc = r" Value of the field"]
pub struct PSELR {
    bits: u8,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `POLARITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLARITYR {
    #[doc = "No task or event."]
    NONE,
    #[doc = "Low to high."]
    LOTOHI,
    #[doc = "High to low."]
    HITOLO,
    #[doc = "Toggle."]
    TOGGLE,
}
impl POLARITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POLARITYR::NONE => 0,
            POLARITYR::LOTOHI => 1,
            POLARITYR::HITOLO => 2,
            POLARITYR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POLARITYR {
        match value {
            0 => POLARITYR::NONE,
            1 => POLARITYR::LOTOHI,
            2 => POLARITYR::HITOLO,
            3 => POLARITYR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == POLARITYR::NONE
    }
    #[doc = "Checks if the value of the field is `LOTOHI`"]
    #[inline]
    pub fn is_lo_to_hi(&self) -> bool {
        *self == POLARITYR::LOTOHI
    }
    #[doc = "Checks if the value of the field is `HITOLO`"]
    #[inline]
    pub fn is_hi_to_lo(&self) -> bool {
        *self == POLARITYR::HITOLO
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == POLARITYR::TOGGLE
    }
}
#[doc = "Possible values of the field `OUTINIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTINITR {
    #[doc = "Initial low output when in task mode."]
    LOW,
    #[doc = "Initial high output when in task mode."]
    HIGH,
}
impl OUTINITR {
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
            OUTINITR::LOW => false,
            OUTINITR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTINITR {
        match value {
            false => OUTINITR::LOW,
            true => OUTINITR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == OUTINITR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == OUTINITR::HIGH
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Channel configure in event mode."]
    EVENT,
    #[doc = "Channel configure in task mode."]
    TASK,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::DISABLED => 0,
            MODEW::EVENT => 1,
            MODEW::TASK => 3,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODEW::DISABLED)
    }
    #[doc = "Channel configure in event mode."]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(MODEW::EVENT)
    }
    #[doc = "Channel configure in task mode."]
    #[inline]
    pub fn task(self) -> &'a mut W {
        self.variant(MODEW::TASK)
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
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POLARITY`"]
pub enum POLARITYW {
    #[doc = "No task or event."]
    NONE,
    #[doc = "Low to high."]
    LOTOHI,
    #[doc = "High to low."]
    HITOLO,
    #[doc = "Toggle."]
    TOGGLE,
}
impl POLARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POLARITYW::NONE => 0,
            POLARITYW::LOTOHI => 1,
            POLARITYW::HITOLO => 2,
            POLARITYW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POLARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _POLARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POLARITYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No task or event."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(POLARITYW::NONE)
    }
    #[doc = "Low to high."]
    #[inline]
    pub fn lo_to_hi(self) -> &'a mut W {
        self.variant(POLARITYW::LOTOHI)
    }
    #[doc = "High to low."]
    #[inline]
    pub fn hi_to_lo(self) -> &'a mut W {
        self.variant(POLARITYW::HITOLO)
    }
    #[doc = "Toggle."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(POLARITYW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OUTINIT`"]
pub enum OUTINITW {
    #[doc = "Initial low output when in task mode."]
    LOW,
    #[doc = "Initial high output when in task mode."]
    HIGH,
}
impl OUTINITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUTINITW::LOW => false,
            OUTINITW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTINITW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTINITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTINITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Initial low output when in task mode."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(OUTINITW::LOW)
    }
    #[doc = "Initial high output when in task mode."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(OUTINITW::HIGH)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - Pin select."]
    #[inline]
    pub fn psel(&self) -> PSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PSELR { bits }
    }
    #[doc = "Bits 16:17 - Effects on output when in Task mode, or events on input that generates an event."]
    #[inline]
    pub fn polarity(&self) -> POLARITYR {
        POLARITYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Initial value of the output when the GPIOTE channel is configured as a Task."]
    #[inline]
    pub fn outinit(&self) -> OUTINITR {
        OUTINITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 8:12 - Pin select."]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bits 16:17 - Effects on output when in Task mode, or events on input that generates an event."]
    #[inline]
    pub fn polarity(&mut self) -> _POLARITYW {
        _POLARITYW { w: self }
    }
    #[doc = "Bit 20 - Initial value of the output when the GPIOTE channel is configured as a Task."]
    #[inline]
    pub fn outinit(&mut self) -> _OUTINITW {
        _OUTINITW { w: self }
    }
}
