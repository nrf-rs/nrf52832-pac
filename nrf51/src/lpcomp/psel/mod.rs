#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSEL {
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
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "Use analog input 0 as analog input."]
    ANALOGINPUT0,
    #[doc = "Use analog input 1 as analog input."]
    ANALOGINPUT1,
    #[doc = "Use analog input 2 as analog input."]
    ANALOGINPUT2,
    #[doc = "Use analog input 3 as analog input."]
    ANALOGINPUT3,
    #[doc = "Use analog input 4 as analog input."]
    ANALOGINPUT4,
    #[doc = "Use analog input 5 as analog input."]
    ANALOGINPUT5,
    #[doc = "Use analog input 6 as analog input."]
    ANALOGINPUT6,
    #[doc = "Use analog input 7 as analog input."]
    ANALOGINPUT7,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::ANALOGINPUT0 => 0,
            PSELR::ANALOGINPUT1 => 1,
            PSELR::ANALOGINPUT2 => 2,
            PSELR::ANALOGINPUT3 => 3,
            PSELR::ANALOGINPUT4 => 4,
            PSELR::ANALOGINPUT5 => 5,
            PSELR::ANALOGINPUT6 => 6,
            PSELR::ANALOGINPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::ANALOGINPUT0,
            1 => PSELR::ANALOGINPUT1,
            2 => PSELR::ANALOGINPUT2,
            3 => PSELR::ANALOGINPUT3,
            4 => PSELR::ANALOGINPUT4,
            5 => PSELR::ANALOGINPUT5,
            6 => PSELR::ANALOGINPUT6,
            7 => PSELR::ANALOGINPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT0`"]
    #[inline]
    pub fn is_analog_input0(&self) -> bool {
        *self == PSELR::ANALOGINPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT1`"]
    #[inline]
    pub fn is_analog_input1(&self) -> bool {
        *self == PSELR::ANALOGINPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT2`"]
    #[inline]
    pub fn is_analog_input2(&self) -> bool {
        *self == PSELR::ANALOGINPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT3`"]
    #[inline]
    pub fn is_analog_input3(&self) -> bool {
        *self == PSELR::ANALOGINPUT3
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT4`"]
    #[inline]
    pub fn is_analog_input4(&self) -> bool {
        *self == PSELR::ANALOGINPUT4
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT5`"]
    #[inline]
    pub fn is_analog_input5(&self) -> bool {
        *self == PSELR::ANALOGINPUT5
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT6`"]
    #[inline]
    pub fn is_analog_input6(&self) -> bool {
        *self == PSELR::ANALOGINPUT6
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT7`"]
    #[inline]
    pub fn is_analog_input7(&self) -> bool {
        *self == PSELR::ANALOGINPUT7
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "Use analog input 0 as analog input."]
    ANALOGINPUT0,
    #[doc = "Use analog input 1 as analog input."]
    ANALOGINPUT1,
    #[doc = "Use analog input 2 as analog input."]
    ANALOGINPUT2,
    #[doc = "Use analog input 3 as analog input."]
    ANALOGINPUT3,
    #[doc = "Use analog input 4 as analog input."]
    ANALOGINPUT4,
    #[doc = "Use analog input 5 as analog input."]
    ANALOGINPUT5,
    #[doc = "Use analog input 6 as analog input."]
    ANALOGINPUT6,
    #[doc = "Use analog input 7 as analog input."]
    ANALOGINPUT7,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::ANALOGINPUT0 => 0,
            PSELW::ANALOGINPUT1 => 1,
            PSELW::ANALOGINPUT2 => 2,
            PSELW::ANALOGINPUT3 => 3,
            PSELW::ANALOGINPUT4 => 4,
            PSELW::ANALOGINPUT5 => 5,
            PSELW::ANALOGINPUT6 => 6,
            PSELW::ANALOGINPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Use analog input 0 as analog input."]
    #[inline]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSELW::ANALOGINPUT0)
    }
    #[doc = "Use analog input 1 as analog input."]
    #[inline]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSELW::ANALOGINPUT1)
    }
    #[doc = "Use analog input 2 as analog input."]
    #[inline]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSELW::ANALOGINPUT2)
    }
    #[doc = "Use analog input 3 as analog input."]
    #[inline]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSELW::ANALOGINPUT3)
    }
    #[doc = "Use analog input 4 as analog input."]
    #[inline]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSELW::ANALOGINPUT4)
    }
    #[doc = "Use analog input 5 as analog input."]
    #[inline]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSELW::ANALOGINPUT5)
    }
    #[doc = "Use analog input 6 as analog input."]
    #[inline]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSELW::ANALOGINPUT6)
    }
    #[doc = "Use analog input 7 as analog input."]
    #[inline]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSELW::ANALOGINPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Analog input pin select."]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Analog input pin select."]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
}
