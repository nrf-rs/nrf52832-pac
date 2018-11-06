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
    #[doc = "VDD * 1/8 selected as reference"]
    REF1_8VDD,
    #[doc = "VDD * 2/8 selected as reference"]
    REF2_8VDD,
    #[doc = "VDD * 3/8 selected as reference"]
    REF3_8VDD,
    #[doc = "VDD * 4/8 selected as reference"]
    REF4_8VDD,
    #[doc = "VDD * 5/8 selected as reference"]
    REF5_8VDD,
    #[doc = "VDD * 6/8 selected as reference"]
    REF6_8VDD,
    #[doc = "VDD * 7/8 selected as reference"]
    REF7_8VDD,
    #[doc = "External analog reference selected"]
    AREF,
    #[doc = "VDD * 1/16 selected as reference"]
    REF1_16VDD,
    #[doc = "VDD * 3/16 selected as reference"]
    REF3_16VDD,
    #[doc = "VDD * 5/16 selected as reference"]
    REF5_16VDD,
    #[doc = "VDD * 7/16 selected as reference"]
    REF7_16VDD,
    #[doc = "VDD * 9/16 selected as reference"]
    REF9_16VDD,
    #[doc = "VDD * 11/16 selected as reference"]
    REF11_16VDD,
    #[doc = "VDD * 13/16 selected as reference"]
    REF13_16VDD,
    #[doc = "VDD * 15/16 selected as reference"]
    REF15_16VDD,
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::REF1_8VDD => 0,
            REFSELR::REF2_8VDD => 1,
            REFSELR::REF3_8VDD => 2,
            REFSELR::REF4_8VDD => 3,
            REFSELR::REF5_8VDD => 4,
            REFSELR::REF6_8VDD => 5,
            REFSELR::REF7_8VDD => 6,
            REFSELR::AREF => 7,
            REFSELR::REF1_16VDD => 8,
            REFSELR::REF3_16VDD => 9,
            REFSELR::REF5_16VDD => 10,
            REFSELR::REF7_16VDD => 11,
            REFSELR::REF9_16VDD => 12,
            REFSELR::REF11_16VDD => 13,
            REFSELR::REF13_16VDD => 14,
            REFSELR::REF15_16VDD => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::REF1_8VDD,
            1 => REFSELR::REF2_8VDD,
            2 => REFSELR::REF3_8VDD,
            3 => REFSELR::REF4_8VDD,
            4 => REFSELR::REF5_8VDD,
            5 => REFSELR::REF6_8VDD,
            6 => REFSELR::REF7_8VDD,
            7 => REFSELR::AREF,
            8 => REFSELR::REF1_16VDD,
            9 => REFSELR::REF3_16VDD,
            10 => REFSELR::REF5_16VDD,
            11 => REFSELR::REF7_16VDD,
            12 => REFSELR::REF9_16VDD,
            13 => REFSELR::REF11_16VDD,
            14 => REFSELR::REF13_16VDD,
            15 => REFSELR::REF15_16VDD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REF1_8VDD`"]
    #[inline]
    pub fn is_ref1_8vdd(&self) -> bool {
        *self == REFSELR::REF1_8VDD
    }
    #[doc = "Checks if the value of the field is `REF2_8VDD`"]
    #[inline]
    pub fn is_ref2_8vdd(&self) -> bool {
        *self == REFSELR::REF2_8VDD
    }
    #[doc = "Checks if the value of the field is `REF3_8VDD`"]
    #[inline]
    pub fn is_ref3_8vdd(&self) -> bool {
        *self == REFSELR::REF3_8VDD
    }
    #[doc = "Checks if the value of the field is `REF4_8VDD`"]
    #[inline]
    pub fn is_ref4_8vdd(&self) -> bool {
        *self == REFSELR::REF4_8VDD
    }
    #[doc = "Checks if the value of the field is `REF5_8VDD`"]
    #[inline]
    pub fn is_ref5_8vdd(&self) -> bool {
        *self == REFSELR::REF5_8VDD
    }
    #[doc = "Checks if the value of the field is `REF6_8VDD`"]
    #[inline]
    pub fn is_ref6_8vdd(&self) -> bool {
        *self == REFSELR::REF6_8VDD
    }
    #[doc = "Checks if the value of the field is `REF7_8VDD`"]
    #[inline]
    pub fn is_ref7_8vdd(&self) -> bool {
        *self == REFSELR::REF7_8VDD
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline]
    pub fn is_aref(&self) -> bool {
        *self == REFSELR::AREF
    }
    #[doc = "Checks if the value of the field is `REF1_16VDD`"]
    #[inline]
    pub fn is_ref1_16vdd(&self) -> bool {
        *self == REFSELR::REF1_16VDD
    }
    #[doc = "Checks if the value of the field is `REF3_16VDD`"]
    #[inline]
    pub fn is_ref3_16vdd(&self) -> bool {
        *self == REFSELR::REF3_16VDD
    }
    #[doc = "Checks if the value of the field is `REF5_16VDD`"]
    #[inline]
    pub fn is_ref5_16vdd(&self) -> bool {
        *self == REFSELR::REF5_16VDD
    }
    #[doc = "Checks if the value of the field is `REF7_16VDD`"]
    #[inline]
    pub fn is_ref7_16vdd(&self) -> bool {
        *self == REFSELR::REF7_16VDD
    }
    #[doc = "Checks if the value of the field is `REF9_16VDD`"]
    #[inline]
    pub fn is_ref9_16vdd(&self) -> bool {
        *self == REFSELR::REF9_16VDD
    }
    #[doc = "Checks if the value of the field is `REF11_16VDD`"]
    #[inline]
    pub fn is_ref11_16vdd(&self) -> bool {
        *self == REFSELR::REF11_16VDD
    }
    #[doc = "Checks if the value of the field is `REF13_16VDD`"]
    #[inline]
    pub fn is_ref13_16vdd(&self) -> bool {
        *self == REFSELR::REF13_16VDD
    }
    #[doc = "Checks if the value of the field is `REF15_16VDD`"]
    #[inline]
    pub fn is_ref15_16vdd(&self) -> bool {
        *self == REFSELR::REF15_16VDD
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "VDD * 1/8 selected as reference"]
    REF1_8VDD,
    #[doc = "VDD * 2/8 selected as reference"]
    REF2_8VDD,
    #[doc = "VDD * 3/8 selected as reference"]
    REF3_8VDD,
    #[doc = "VDD * 4/8 selected as reference"]
    REF4_8VDD,
    #[doc = "VDD * 5/8 selected as reference"]
    REF5_8VDD,
    #[doc = "VDD * 6/8 selected as reference"]
    REF6_8VDD,
    #[doc = "VDD * 7/8 selected as reference"]
    REF7_8VDD,
    #[doc = "External analog reference selected"]
    AREF,
    #[doc = "VDD * 1/16 selected as reference"]
    REF1_16VDD,
    #[doc = "VDD * 3/16 selected as reference"]
    REF3_16VDD,
    #[doc = "VDD * 5/16 selected as reference"]
    REF5_16VDD,
    #[doc = "VDD * 7/16 selected as reference"]
    REF7_16VDD,
    #[doc = "VDD * 9/16 selected as reference"]
    REF9_16VDD,
    #[doc = "VDD * 11/16 selected as reference"]
    REF11_16VDD,
    #[doc = "VDD * 13/16 selected as reference"]
    REF13_16VDD,
    #[doc = "VDD * 15/16 selected as reference"]
    REF15_16VDD,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::REF1_8VDD => 0,
            REFSELW::REF2_8VDD => 1,
            REFSELW::REF3_8VDD => 2,
            REFSELW::REF4_8VDD => 3,
            REFSELW::REF5_8VDD => 4,
            REFSELW::REF6_8VDD => 5,
            REFSELW::REF7_8VDD => 6,
            REFSELW::AREF => 7,
            REFSELW::REF1_16VDD => 8,
            REFSELW::REF3_16VDD => 9,
            REFSELW::REF5_16VDD => 10,
            REFSELW::REF7_16VDD => 11,
            REFSELW::REF9_16VDD => 12,
            REFSELW::REF11_16VDD => 13,
            REFSELW::REF13_16VDD => 14,
            REFSELW::REF15_16VDD => 15,
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
    #[doc = "VDD * 1/8 selected as reference"]
    #[inline]
    pub fn ref1_8vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF1_8VDD)
    }
    #[doc = "VDD * 2/8 selected as reference"]
    #[inline]
    pub fn ref2_8vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF2_8VDD)
    }
    #[doc = "VDD * 3/8 selected as reference"]
    #[inline]
    pub fn ref3_8vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF3_8VDD)
    }
    #[doc = "VDD * 4/8 selected as reference"]
    #[inline]
    pub fn ref4_8vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF4_8VDD)
    }
    #[doc = "VDD * 5/8 selected as reference"]
    #[inline]
    pub fn ref5_8vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF5_8VDD)
    }
    #[doc = "VDD * 6/8 selected as reference"]
    #[inline]
    pub fn ref6_8vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF6_8VDD)
    }
    #[doc = "VDD * 7/8 selected as reference"]
    #[inline]
    pub fn ref7_8vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF7_8VDD)
    }
    #[doc = "External analog reference selected"]
    #[inline]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSELW::AREF)
    }
    #[doc = "VDD * 1/16 selected as reference"]
    #[inline]
    pub fn ref1_16vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF1_16VDD)
    }
    #[doc = "VDD * 3/16 selected as reference"]
    #[inline]
    pub fn ref3_16vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF3_16VDD)
    }
    #[doc = "VDD * 5/16 selected as reference"]
    #[inline]
    pub fn ref5_16vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF5_16VDD)
    }
    #[doc = "VDD * 7/16 selected as reference"]
    #[inline]
    pub fn ref7_16vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF7_16VDD)
    }
    #[doc = "VDD * 9/16 selected as reference"]
    #[inline]
    pub fn ref9_16vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF9_16VDD)
    }
    #[doc = "VDD * 11/16 selected as reference"]
    #[inline]
    pub fn ref11_16vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF11_16VDD)
    }
    #[doc = "VDD * 13/16 selected as reference"]
    #[inline]
    pub fn ref13_16vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF13_16VDD)
    }
    #[doc = "VDD * 15/16 selected as reference"]
    #[inline]
    pub fn ref15_16vdd(self) -> &'a mut W {
        self.variant(REFSELW::REF15_16VDD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Reference select"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Reference select"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
}
