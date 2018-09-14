#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSELN {
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
#[doc = "Possible values of the field `PSELN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELNR {
    #[doc = "Not connected"]
    NC,
    #[doc = "AIN0"]
    ANALOGINPUT0,
    #[doc = "AIN1"]
    ANALOGINPUT1,
    #[doc = "AIN2"]
    ANALOGINPUT2,
    #[doc = "AIN3"]
    ANALOGINPUT3,
    #[doc = "AIN4"]
    ANALOGINPUT4,
    #[doc = "AIN5"]
    ANALOGINPUT5,
    #[doc = "AIN6"]
    ANALOGINPUT6,
    #[doc = "AIN7"]
    ANALOGINPUT7,
    #[doc = "VDD"]
    VDD,
    #[doc = "VDDH/5"]
    VDDHDIV5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSELNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELNR::NC => 0,
            PSELNR::ANALOGINPUT0 => 1,
            PSELNR::ANALOGINPUT1 => 2,
            PSELNR::ANALOGINPUT2 => 3,
            PSELNR::ANALOGINPUT3 => 4,
            PSELNR::ANALOGINPUT4 => 5,
            PSELNR::ANALOGINPUT5 => 6,
            PSELNR::ANALOGINPUT6 => 7,
            PSELNR::ANALOGINPUT7 => 8,
            PSELNR::VDD => 9,
            PSELNR::VDDHDIV5 => 13,
            PSELNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELNR {
        match value {
            0 => PSELNR::NC,
            1 => PSELNR::ANALOGINPUT0,
            2 => PSELNR::ANALOGINPUT1,
            3 => PSELNR::ANALOGINPUT2,
            4 => PSELNR::ANALOGINPUT3,
            5 => PSELNR::ANALOGINPUT4,
            6 => PSELNR::ANALOGINPUT5,
            7 => PSELNR::ANALOGINPUT6,
            8 => PSELNR::ANALOGINPUT7,
            9 => PSELNR::VDD,
            13 => PSELNR::VDDHDIV5,
            i => PSELNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == PSELNR::NC
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT0`"]
    #[inline]
    pub fn is_analog_input0(&self) -> bool {
        *self == PSELNR::ANALOGINPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT1`"]
    #[inline]
    pub fn is_analog_input1(&self) -> bool {
        *self == PSELNR::ANALOGINPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT2`"]
    #[inline]
    pub fn is_analog_input2(&self) -> bool {
        *self == PSELNR::ANALOGINPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT3`"]
    #[inline]
    pub fn is_analog_input3(&self) -> bool {
        *self == PSELNR::ANALOGINPUT3
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT4`"]
    #[inline]
    pub fn is_analog_input4(&self) -> bool {
        *self == PSELNR::ANALOGINPUT4
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT5`"]
    #[inline]
    pub fn is_analog_input5(&self) -> bool {
        *self == PSELNR::ANALOGINPUT5
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT6`"]
    #[inline]
    pub fn is_analog_input6(&self) -> bool {
        *self == PSELNR::ANALOGINPUT6
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT7`"]
    #[inline]
    pub fn is_analog_input7(&self) -> bool {
        *self == PSELNR::ANALOGINPUT7
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline]
    pub fn is_vdd(&self) -> bool {
        *self == PSELNR::VDD
    }
    #[doc = "Checks if the value of the field is `VDDHDIV5`"]
    #[inline]
    pub fn is_vddhdiv5(&self) -> bool {
        *self == PSELNR::VDDHDIV5
    }
}
#[doc = "Values that can be written to the field `PSELN`"]
pub enum PSELNW {
    #[doc = "Not connected"]
    NC,
    #[doc = "AIN0"]
    ANALOGINPUT0,
    #[doc = "AIN1"]
    ANALOGINPUT1,
    #[doc = "AIN2"]
    ANALOGINPUT2,
    #[doc = "AIN3"]
    ANALOGINPUT3,
    #[doc = "AIN4"]
    ANALOGINPUT4,
    #[doc = "AIN5"]
    ANALOGINPUT5,
    #[doc = "AIN6"]
    ANALOGINPUT6,
    #[doc = "AIN7"]
    ANALOGINPUT7,
    #[doc = "VDD"]
    VDD,
    #[doc = "VDDH/5"]
    VDDHDIV5,
}
impl PSELNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELNW::NC => 0,
            PSELNW::ANALOGINPUT0 => 1,
            PSELNW::ANALOGINPUT1 => 2,
            PSELNW::ANALOGINPUT2 => 3,
            PSELNW::ANALOGINPUT3 => 4,
            PSELNW::ANALOGINPUT4 => 5,
            PSELNW::ANALOGINPUT5 => 6,
            PSELNW::ANALOGINPUT6 => 7,
            PSELNW::ANALOGINPUT7 => 8,
            PSELNW::VDD => 9,
            PSELNW::VDDHDIV5 => 13,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELNW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not connected"]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(PSELNW::NC)
    }
    #[doc = "AIN0"]
    #[inline]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSELNW::ANALOGINPUT0)
    }
    #[doc = "AIN1"]
    #[inline]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSELNW::ANALOGINPUT1)
    }
    #[doc = "AIN2"]
    #[inline]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSELNW::ANALOGINPUT2)
    }
    #[doc = "AIN3"]
    #[inline]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSELNW::ANALOGINPUT3)
    }
    #[doc = "AIN4"]
    #[inline]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSELNW::ANALOGINPUT4)
    }
    #[doc = "AIN5"]
    #[inline]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSELNW::ANALOGINPUT5)
    }
    #[doc = "AIN6"]
    #[inline]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSELNW::ANALOGINPUT6)
    }
    #[doc = "AIN7"]
    #[inline]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSELNW::ANALOGINPUT7)
    }
    #[doc = "VDD"]
    #[inline]
    pub fn vdd(self) -> &'a mut W {
        self.variant(PSELNW::VDD)
    }
    #[doc = "VDDH/5"]
    #[inline]
    pub fn vddhdiv5(self) -> &'a mut W {
        self.variant(PSELNW::VDDHDIV5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:4 - Analog negative input, enables differential channel"]
    #[inline]
    pub fn pseln(&self) -> PSELNR {
        PSELNR::_from({
            const MASK: u8 = 31;
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
    #[doc = "Bits 0:4 - Analog negative input, enables differential channel"]
    #[inline]
    pub fn pseln(&mut self) -> _PSELNW {
        _PSELNW { w: self }
    }
}
