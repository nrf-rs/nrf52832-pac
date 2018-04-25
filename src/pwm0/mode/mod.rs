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
#[doc = "Possible values of the field `UPDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDOWNR {
    #[doc = "Up counter - edge aligned PWM duty-cycle"]
    UP,
    #[doc = "Up and down counter - center aligned PWM duty cycle"]
    UPANDDOWN,
}
impl UPDOWNR {
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
            UPDOWNR::UP => false,
            UPDOWNR::UPANDDOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPDOWNR {
        match value {
            false => UPDOWNR::UP,
            true => UPDOWNR::UPANDDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == UPDOWNR::UP
    }
    #[doc = "Checks if the value of the field is `UPANDDOWN`"]
    #[inline]
    pub fn is_up_and_down(&self) -> bool {
        *self == UPDOWNR::UPANDDOWN
    }
}
#[doc = "Values that can be written to the field `UPDOWN`"]
pub enum UPDOWNW {
    #[doc = "Up counter - edge aligned PWM duty-cycle"]
    UP,
    #[doc = "Up and down counter - center aligned PWM duty cycle"]
    UPANDDOWN,
}
impl UPDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPDOWNW::UP => false,
            UPDOWNW::UPANDDOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Up counter - edge aligned PWM duty-cycle"]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(UPDOWNW::UP)
    }
    #[doc = "Up and down counter - center aligned PWM duty cycle"]
    #[inline]
    pub fn up_and_down(self) -> &'a mut W {
        self.variant(UPDOWNW::UPANDDOWN)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Selects up or up and down as wave counter mode"]
    #[inline]
    pub fn updown(&self) -> UPDOWNR {
        UPDOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Selects up or up and down as wave counter mode"]
    #[inline]
    pub fn updown(&mut self) -> _UPDOWNW {
        _UPDOWNW { w: self }
    }
}
