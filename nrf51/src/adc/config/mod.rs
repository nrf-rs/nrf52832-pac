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
#[doc = "Possible values of the field `RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESR {
    #[doc = "8bit ADC resolution."]
    _8BIT,
    #[doc = "9bit ADC resolution."]
    _9BIT,
    #[doc = "10bit ADC resolution."]
    _10BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESR::_8BIT => 0,
            RESR::_9BIT => 1,
            RESR::_10BIT => 2,
            RESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESR {
        match value {
            0 => RESR::_8BIT,
            1 => RESR::_9BIT,
            2 => RESR::_10BIT,
            i => RESR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline]
    pub fn is_8bit(&self) -> bool {
        *self == RESR::_8BIT
    }
    #[doc = "Checks if the value of the field is `_9BIT`"]
    #[inline]
    pub fn is_9bit(&self) -> bool {
        *self == RESR::_9BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline]
    pub fn is_10bit(&self) -> bool {
        *self == RESR::_10BIT
    }
}
#[doc = "Possible values of the field `INPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPSELR {
    #[doc = "Analog input specified by PSEL with no prescaling used as input for the conversion."]
    ANALOGINPUTNOPRESCALING,
    #[doc = "Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    ANALOGINPUTTWOTHIRDSPRESCALING,
    #[doc = "Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    ANALOGINPUTONETHIRDPRESCALING,
    #[doc = "Supply voltage with 2/3 prescaling used as input for the conversion."]
    SUPPLYTWOTHIRDSPRESCALING,
    #[doc = "Supply voltage with 1/3 prescaling used as input for the conversion."]
    SUPPLYONETHIRDPRESCALING,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPSELR::ANALOGINPUTNOPRESCALING => 0,
            INPSELR::ANALOGINPUTTWOTHIRDSPRESCALING => 1,
            INPSELR::ANALOGINPUTONETHIRDPRESCALING => 2,
            INPSELR::SUPPLYTWOTHIRDSPRESCALING => 5,
            INPSELR::SUPPLYONETHIRDPRESCALING => 6,
            INPSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPSELR {
        match value {
            0 => INPSELR::ANALOGINPUTNOPRESCALING,
            1 => INPSELR::ANALOGINPUTTWOTHIRDSPRESCALING,
            2 => INPSELR::ANALOGINPUTONETHIRDPRESCALING,
            5 => INPSELR::SUPPLYTWOTHIRDSPRESCALING,
            6 => INPSELR::SUPPLYONETHIRDPRESCALING,
            i => INPSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUTNOPRESCALING`"]
    #[inline]
    pub fn is_analog_input_no_prescaling(&self) -> bool {
        *self == INPSELR::ANALOGINPUTNOPRESCALING
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUTTWOTHIRDSPRESCALING`"]
    #[inline]
    pub fn is_analog_input_two_thirds_prescaling(&self) -> bool {
        *self == INPSELR::ANALOGINPUTTWOTHIRDSPRESCALING
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUTONETHIRDPRESCALING`"]
    #[inline]
    pub fn is_analog_input_one_third_prescaling(&self) -> bool {
        *self == INPSELR::ANALOGINPUTONETHIRDPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYTWOTHIRDSPRESCALING`"]
    #[inline]
    pub fn is_supply_two_thirds_prescaling(&self) -> bool {
        *self == INPSELR::SUPPLYTWOTHIRDSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYONETHIRDPRESCALING`"]
    #[inline]
    pub fn is_supply_one_third_prescaling(&self) -> bool {
        *self == INPSELR::SUPPLYONETHIRDPRESCALING
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Use internal 1.2V bandgap voltage as reference for conversion."]
    VBG,
    #[doc = "Use external source configured by EXTREFSEL as reference for conversion."]
    EXTERNAL,
    #[doc = "Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    SUPPLYONEHALFPRESCALING,
    #[doc = "Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    SUPPLYONETHIRDPRESCALING,
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::VBG => 0,
            REFSELR::EXTERNAL => 1,
            REFSELR::SUPPLYONEHALFPRESCALING => 2,
            REFSELR::SUPPLYONETHIRDPRESCALING => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::VBG,
            1 => REFSELR::EXTERNAL,
            2 => REFSELR::SUPPLYONEHALFPRESCALING,
            3 => REFSELR::SUPPLYONETHIRDPRESCALING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VBG`"]
    #[inline]
    pub fn is_vbg(&self) -> bool {
        *self == REFSELR::VBG
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == REFSELR::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `SUPPLYONEHALFPRESCALING`"]
    #[inline]
    pub fn is_supply_one_half_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYONEHALFPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYONETHIRDPRESCALING`"]
    #[inline]
    pub fn is_supply_one_third_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYONETHIRDPRESCALING
    }
}
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "Analog input pins disabled."]
    DISABLED,
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::DISABLED => 0,
            PSELR::ANALOGINPUT0 => 1,
            PSELR::ANALOGINPUT1 => 2,
            PSELR::ANALOGINPUT2 => 4,
            PSELR::ANALOGINPUT3 => 8,
            PSELR::ANALOGINPUT4 => 16,
            PSELR::ANALOGINPUT5 => 32,
            PSELR::ANALOGINPUT6 => 64,
            PSELR::ANALOGINPUT7 => 128,
            PSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::DISABLED,
            1 => PSELR::ANALOGINPUT0,
            2 => PSELR::ANALOGINPUT1,
            4 => PSELR::ANALOGINPUT2,
            8 => PSELR::ANALOGINPUT3,
            16 => PSELR::ANALOGINPUT4,
            32 => PSELR::ANALOGINPUT5,
            64 => PSELR::ANALOGINPUT6,
            128 => PSELR::ANALOGINPUT7,
            i => PSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PSELR::DISABLED
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
#[doc = "Possible values of the field `EXTREFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTREFSELR {
    #[doc = "Analog external reference inputs disabled."]
    NONE,
    #[doc = "Use analog reference 0 as reference."]
    ANALOGREFERENCE0,
    #[doc = "Use analog reference 1 as reference."]
    ANALOGREFERENCE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTREFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTREFSELR::NONE => 0,
            EXTREFSELR::ANALOGREFERENCE0 => 1,
            EXTREFSELR::ANALOGREFERENCE1 => 2,
            EXTREFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTREFSELR {
        match value {
            0 => EXTREFSELR::NONE,
            1 => EXTREFSELR::ANALOGREFERENCE0,
            2 => EXTREFSELR::ANALOGREFERENCE1,
            i => EXTREFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == EXTREFSELR::NONE
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE0`"]
    #[inline]
    pub fn is_analog_reference0(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE0
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE1`"]
    #[inline]
    pub fn is_analog_reference1(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE1
    }
}
#[doc = "Values that can be written to the field `RES`"]
pub enum RESW {
    #[doc = "8bit ADC resolution."]
    _8BIT,
    #[doc = "9bit ADC resolution."]
    _9BIT,
    #[doc = "10bit ADC resolution."]
    _10BIT,
}
impl RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESW::_8BIT => 0,
            RESW::_9BIT => 1,
            RESW::_10BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESW<'a> {
    w: &'a mut W,
}
impl<'a> _RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8bit ADC resolution."]
    #[inline]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RESW::_8BIT)
    }
    #[doc = "9bit ADC resolution."]
    #[inline]
    pub fn _9bit(self) -> &'a mut W {
        self.variant(RESW::_9BIT)
    }
    #[doc = "10bit ADC resolution."]
    #[inline]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(RESW::_10BIT)
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
#[doc = "Values that can be written to the field `INPSEL`"]
pub enum INPSELW {
    #[doc = "Analog input specified by PSEL with no prescaling used as input for the conversion."]
    ANALOGINPUTNOPRESCALING,
    #[doc = "Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    ANALOGINPUTTWOTHIRDSPRESCALING,
    #[doc = "Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    ANALOGINPUTONETHIRDPRESCALING,
    #[doc = "Supply voltage with 2/3 prescaling used as input for the conversion."]
    SUPPLYTWOTHIRDSPRESCALING,
    #[doc = "Supply voltage with 1/3 prescaling used as input for the conversion."]
    SUPPLYONETHIRDPRESCALING,
}
impl INPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPSELW::ANALOGINPUTNOPRESCALING => 0,
            INPSELW::ANALOGINPUTTWOTHIRDSPRESCALING => 1,
            INPSELW::ANALOGINPUTONETHIRDPRESCALING => 2,
            INPSELW::SUPPLYTWOTHIRDSPRESCALING => 5,
            INPSELW::SUPPLYONETHIRDPRESCALING => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Analog input specified by PSEL with no prescaling used as input for the conversion."]
    #[inline]
    pub fn analog_input_no_prescaling(self) -> &'a mut W {
        self.variant(INPSELW::ANALOGINPUTNOPRESCALING)
    }
    #[doc = "Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    #[inline]
    pub fn analog_input_two_thirds_prescaling(self) -> &'a mut W {
        self.variant(INPSELW::ANALOGINPUTTWOTHIRDSPRESCALING)
    }
    #[doc = "Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    #[inline]
    pub fn analog_input_one_third_prescaling(self) -> &'a mut W {
        self.variant(INPSELW::ANALOGINPUTONETHIRDPRESCALING)
    }
    #[doc = "Supply voltage with 2/3 prescaling used as input for the conversion."]
    #[inline]
    pub fn supply_two_thirds_prescaling(self) -> &'a mut W {
        self.variant(INPSELW::SUPPLYTWOTHIRDSPRESCALING)
    }
    #[doc = "Supply voltage with 1/3 prescaling used as input for the conversion."]
    #[inline]
    pub fn supply_one_third_prescaling(self) -> &'a mut W {
        self.variant(INPSELW::SUPPLYONETHIRDPRESCALING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Use internal 1.2V bandgap voltage as reference for conversion."]
    VBG,
    #[doc = "Use external source configured by EXTREFSEL as reference for conversion."]
    EXTERNAL,
    #[doc = "Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    SUPPLYONEHALFPRESCALING,
    #[doc = "Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    SUPPLYONETHIRDPRESCALING,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::VBG => 0,
            REFSELW::EXTERNAL => 1,
            REFSELW::SUPPLYONEHALFPRESCALING => 2,
            REFSELW::SUPPLYONETHIRDPRESCALING => 3,
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
            self.bits(variant._bits())
        }
    }
    #[doc = "Use internal 1.2V bandgap voltage as reference for conversion."]
    #[inline]
    pub fn vbg(self) -> &'a mut W {
        self.variant(REFSELW::VBG)
    }
    #[doc = "Use external source configured by EXTREFSEL as reference for conversion."]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(REFSELW::EXTERNAL)
    }
    #[doc = "Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    #[inline]
    pub fn supply_one_half_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYONEHALFPRESCALING)
    }
    #[doc = "Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    #[inline]
    pub fn supply_one_third_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYONETHIRDPRESCALING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "Analog input pins disabled."]
    DISABLED,
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
            PSELW::DISABLED => 0,
            PSELW::ANALOGINPUT0 => 1,
            PSELW::ANALOGINPUT1 => 2,
            PSELW::ANALOGINPUT2 => 4,
            PSELW::ANALOGINPUT3 => 8,
            PSELW::ANALOGINPUT4 => 16,
            PSELW::ANALOGINPUT5 => 32,
            PSELW::ANALOGINPUT6 => 64,
            PSELW::ANALOGINPUT7 => 128,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Analog input pins disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PSELW::DISABLED)
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTREFSEL`"]
pub enum EXTREFSELW {
    #[doc = "Analog external reference inputs disabled."]
    NONE,
    #[doc = "Use analog reference 0 as reference."]
    ANALOGREFERENCE0,
    #[doc = "Use analog reference 1 as reference."]
    ANALOGREFERENCE1,
}
impl EXTREFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTREFSELW::NONE => 0,
            EXTREFSELW::ANALOGREFERENCE0 => 1,
            EXTREFSELW::ANALOGREFERENCE1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTREFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTREFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTREFSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Analog external reference inputs disabled."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(EXTREFSELW::NONE)
    }
    #[doc = "Use analog reference 0 as reference."]
    #[inline]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE0)
    }
    #[doc = "Use analog reference 1 as reference."]
    #[inline]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - ADC resolution."]
    #[inline]
    pub fn res(&self) -> RESR {
        RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:4 - ADC input selection."]
    #[inline]
    pub fn inpsel(&self) -> INPSELR {
        INPSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - ADC reference selection."]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - ADC analog pin selection."]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - ADC external reference pin selection."]
    #[inline]
    pub fn extrefsel(&self) -> EXTREFSELR {
        EXTREFSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 24 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - ADC resolution."]
    #[inline]
    pub fn res(&mut self) -> _RESW {
        _RESW { w: self }
    }
    #[doc = "Bits 2:4 - ADC input selection."]
    #[inline]
    pub fn inpsel(&mut self) -> _INPSELW {
        _INPSELW { w: self }
    }
    #[doc = "Bits 5:6 - ADC reference selection."]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bits 8:15 - ADC analog pin selection."]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bits 16:17 - ADC external reference pin selection."]
    #[inline]
    pub fn extrefsel(&mut self) -> _EXTREFSELW {
        _EXTREFSELW { w: self }
    }
}
