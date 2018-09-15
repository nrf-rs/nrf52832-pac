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
#[doc = "Possible values of the field `SLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPR {
    #[doc = "Pause watchdog while the CPU is asleep."]
    PAUSE,
    #[doc = "Do not pause watchdog while the CPU is asleep."]
    RUN,
}
impl SLEEPR {
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
            SLEEPR::PAUSE => false,
            SLEEPR::RUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPR {
        match value {
            false => SLEEPR::PAUSE,
            true => SLEEPR::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline]
    pub fn is_pause(&self) -> bool {
        *self == SLEEPR::PAUSE
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == SLEEPR::RUN
    }
}
#[doc = "Possible values of the field `HALT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTR {
    #[doc = "Pause watchdog while the CPU is halted by the debugger."]
    PAUSE,
    #[doc = "Do not pause watchdog while the CPU is halted by the debugger."]
    RUN,
}
impl HALTR {
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
            HALTR::PAUSE => false,
            HALTR::RUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HALTR {
        match value {
            false => HALTR::PAUSE,
            true => HALTR::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline]
    pub fn is_pause(&self) -> bool {
        *self == HALTR::PAUSE
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == HALTR::RUN
    }
}
#[doc = "Values that can be written to the field `SLEEP`"]
pub enum SLEEPW {
    #[doc = "Pause watchdog while the CPU is asleep."]
    PAUSE,
    #[doc = "Do not pause watchdog while the CPU is asleep."]
    RUN,
}
impl SLEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPW::PAUSE => false,
            SLEEPW::RUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pause watchdog while the CPU is asleep."]
    #[inline]
    pub fn pause(self) -> &'a mut W {
        self.variant(SLEEPW::PAUSE)
    }
    #[doc = "Do not pause watchdog while the CPU is asleep."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(SLEEPW::RUN)
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
#[doc = "Values that can be written to the field `HALT`"]
pub enum HALTW {
    #[doc = "Pause watchdog while the CPU is halted by the debugger."]
    PAUSE,
    #[doc = "Do not pause watchdog while the CPU is halted by the debugger."]
    RUN,
}
impl HALTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HALTW::PAUSE => false,
            HALTW::RUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HALTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pause watchdog while the CPU is halted by the debugger."]
    #[inline]
    pub fn pause(self) -> &'a mut W {
        self.variant(HALTW::PAUSE)
    }
    #[doc = "Do not pause watchdog while the CPU is halted by the debugger."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(HALTW::RUN)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Configure the watchdog to pause or not while the CPU is sleeping."]
    #[inline]
    pub fn sleep(&self) -> SLEEPR {
        SLEEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Configure the watchdog to pause or not while the CPU is halted by the debugger."]
    #[inline]
    pub fn halt(&self) -> HALTR {
        HALTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Configure the watchdog to pause or not while the CPU is sleeping."]
    #[inline]
    pub fn sleep(&mut self) -> _SLEEPW {
        _SLEEPW { w: self }
    }
    #[doc = "Bit 3 - Configure the watchdog to pause or not while the CPU is halted by the debugger."]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
}
