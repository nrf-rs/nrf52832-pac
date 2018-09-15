#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REFSEL {
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
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Use supply with a 1/8 prescaler as reference."]
    SUPPLYONEEIGHTHPRESCALING,
    #[doc = "Use supply with a 2/8 prescaler as reference."]
    SUPPLYTWOEIGHTHSPRESCALING,
    #[doc = "Use supply with a 3/8 prescaler as reference."]
    SUPPLYTHREEEIGHTHSPRESCALING,
    #[doc = "Use supply with a 4/8 prescaler as reference."]
    SUPPLYFOUREIGHTHSPRESCALING,
    #[doc = "Use supply with a 5/8 prescaler as reference."]
    SUPPLYFIVEEIGHTHSPRESCALING,
    #[doc = "Use supply with a 6/8 prescaler as reference."]
    SUPPLYSIXEIGHTHSPRESCALING,
    #[doc = "Use supply with a 7/8 prescaler as reference."]
    SUPPLYSEVENEIGHTHSPRESCALING,
    #[doc = "Use external analog reference as reference."]
    AREF,
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::SUPPLYONEEIGHTHPRESCALING => 0,
            REFSELR::SUPPLYTWOEIGHTHSPRESCALING => 1,
            REFSELR::SUPPLYTHREEEIGHTHSPRESCALING => 2,
            REFSELR::SUPPLYFOUREIGHTHSPRESCALING => 3,
            REFSELR::SUPPLYFIVEEIGHTHSPRESCALING => 4,
            REFSELR::SUPPLYSIXEIGHTHSPRESCALING => 5,
            REFSELR::SUPPLYSEVENEIGHTHSPRESCALING => 6,
            REFSELR::AREF => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::SUPPLYONEEIGHTHPRESCALING,
            1 => REFSELR::SUPPLYTWOEIGHTHSPRESCALING,
            2 => REFSELR::SUPPLYTHREEEIGHTHSPRESCALING,
            3 => REFSELR::SUPPLYFOUREIGHTHSPRESCALING,
            4 => REFSELR::SUPPLYFIVEEIGHTHSPRESCALING,
            5 => REFSELR::SUPPLYSIXEIGHTHSPRESCALING,
            6 => REFSELR::SUPPLYSEVENEIGHTHSPRESCALING,
            7 => REFSELR::AREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SUPPLYONEEIGHTHPRESCALING`"]
    #[inline]
    pub fn is_supply_one_eighth_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYONEEIGHTHPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYTWOEIGHTHSPRESCALING`"]
    #[inline]
    pub fn is_supply_two_eighths_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYTWOEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYTHREEEIGHTHSPRESCALING`"]
    #[inline]
    pub fn is_supply_three_eighths_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYTHREEEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYFOUREIGHTHSPRESCALING`"]
    #[inline]
    pub fn is_supply_four_eighths_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYFOUREIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYFIVEEIGHTHSPRESCALING`"]
    #[inline]
    pub fn is_supply_five_eighths_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYFIVEEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYSIXEIGHTHSPRESCALING`"]
    #[inline]
    pub fn is_supply_six_eighths_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYSIXEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYSEVENEIGHTHSPRESCALING`"]
    #[inline]
    pub fn is_supply_seven_eighths_prescaling(&self) -> bool {
        *self == REFSELR::SUPPLYSEVENEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline]
    pub fn is_aref(&self) -> bool {
        *self == REFSELR::AREF
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Use supply with a 1/8 prescaler as reference."]
    SUPPLYONEEIGHTHPRESCALING,
    #[doc = "Use supply with a 2/8 prescaler as reference."]
    SUPPLYTWOEIGHTHSPRESCALING,
    #[doc = "Use supply with a 3/8 prescaler as reference."]
    SUPPLYTHREEEIGHTHSPRESCALING,
    #[doc = "Use supply with a 4/8 prescaler as reference."]
    SUPPLYFOUREIGHTHSPRESCALING,
    #[doc = "Use supply with a 5/8 prescaler as reference."]
    SUPPLYFIVEEIGHTHSPRESCALING,
    #[doc = "Use supply with a 6/8 prescaler as reference."]
    SUPPLYSIXEIGHTHSPRESCALING,
    #[doc = "Use supply with a 7/8 prescaler as reference."]
    SUPPLYSEVENEIGHTHSPRESCALING,
    #[doc = "Use external analog reference as reference."]
    AREF,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::SUPPLYONEEIGHTHPRESCALING => 0,
            REFSELW::SUPPLYTWOEIGHTHSPRESCALING => 1,
            REFSELW::SUPPLYTHREEEIGHTHSPRESCALING => 2,
            REFSELW::SUPPLYFOUREIGHTHSPRESCALING => 3,
            REFSELW::SUPPLYFIVEEIGHTHSPRESCALING => 4,
            REFSELW::SUPPLYSIXEIGHTHSPRESCALING => 5,
            REFSELW::SUPPLYSEVENEIGHTHSPRESCALING => 6,
            REFSELW::AREF => 7,
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
    #[doc = "Use supply with a 1/8 prescaler as reference."]
    #[inline]
    pub fn supply_one_eighth_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYONEEIGHTHPRESCALING)
    }
    #[doc = "Use supply with a 2/8 prescaler as reference."]
    #[inline]
    pub fn supply_two_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYTWOEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 3/8 prescaler as reference."]
    #[inline]
    pub fn supply_three_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYTHREEEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 4/8 prescaler as reference."]
    #[inline]
    pub fn supply_four_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYFOUREIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 5/8 prescaler as reference."]
    #[inline]
    pub fn supply_five_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYFIVEEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 6/8 prescaler as reference."]
    #[inline]
    pub fn supply_six_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYSIXEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 7/8 prescaler as reference."]
    #[inline]
    pub fn supply_seven_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSELW::SUPPLYSEVENEIGHTHSPRESCALING)
    }
    #[doc = "Use external analog reference as reference."]
    #[inline]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSELW::AREF)
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
    #[doc = "Bits 0:2 - Reference select."]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
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
    #[doc = "Bits 0:2 - Reference select."]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
}
