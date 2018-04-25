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
    #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    INT1V2,
    #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    INT1V8,
    #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    INT2V4,
    #[doc = "VREF = VDD"]
    VDD,
    #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    AREF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::INT1V2 => 0,
            REFSELR::INT1V8 => 1,
            REFSELR::INT2V4 => 2,
            REFSELR::VDD => 4,
            REFSELR::AREF => 7,
            REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::INT1V2,
            1 => REFSELR::INT1V8,
            2 => REFSELR::INT2V4,
            4 => REFSELR::VDD,
            7 => REFSELR::AREF,
            i => REFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INT1V2`"]
    #[inline]
    pub fn is_int1v2(&self) -> bool {
        *self == REFSELR::INT1V2
    }
    #[doc = "Checks if the value of the field is `INT1V8`"]
    #[inline]
    pub fn is_int1v8(&self) -> bool {
        *self == REFSELR::INT1V8
    }
    #[doc = "Checks if the value of the field is `INT2V4`"]
    #[inline]
    pub fn is_int2v4(&self) -> bool {
        *self == REFSELR::INT2V4
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline]
    pub fn is_vdd(&self) -> bool {
        *self == REFSELR::VDD
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline]
    pub fn is_aref(&self) -> bool {
        *self == REFSELR::AREF
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    INT1V2,
    #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    INT1V8,
    #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    INT2V4,
    #[doc = "VREF = VDD"]
    VDD,
    #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    AREF,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::INT1V2 => 0,
            REFSELW::INT1V8 => 1,
            REFSELW::INT2V4 => 2,
            REFSELW::VDD => 4,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    #[inline]
    pub fn int1v2(self) -> &'a mut W {
        self.variant(REFSELW::INT1V2)
    }
    #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline]
    pub fn int1v8(self) -> &'a mut W {
        self.variant(REFSELW::INT1V8)
    }
    #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline]
    pub fn int2v4(self) -> &'a mut W {
        self.variant(REFSELW::INT2V4)
    }
    #[doc = "VREF = VDD"]
    #[inline]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSELW::VDD)
    }
    #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    #[inline]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSELW::AREF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:2 - Reference select"]
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
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Reference select"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
}
