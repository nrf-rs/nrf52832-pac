#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTREFSEL {
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
#[doc = "Possible values of the field `EXTREFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTREFSELR {
    #[doc = "Use AIN0 as external analog reference"]
    ANALOGREFERENCE0,
    #[doc = "Use AIN1 as external analog reference"]
    ANALOGREFERENCE1,
    #[doc = "Use AIN2 as external analog reference"]
    ANALOGREFERENCE2,
    #[doc = "Use AIN3 as external analog reference"]
    ANALOGREFERENCE3,
    #[doc = "Use AIN4 as external analog reference"]
    ANALOGREFERENCE4,
    #[doc = "Use AIN5 as external analog reference"]
    ANALOGREFERENCE5,
    #[doc = "Use AIN6 as external analog reference"]
    ANALOGREFERENCE6,
    #[doc = "Use AIN7 as external analog reference"]
    ANALOGREFERENCE7,
}
impl EXTREFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTREFSELR::ANALOGREFERENCE0 => 0,
            EXTREFSELR::ANALOGREFERENCE1 => 1,
            EXTREFSELR::ANALOGREFERENCE2 => 2,
            EXTREFSELR::ANALOGREFERENCE3 => 3,
            EXTREFSELR::ANALOGREFERENCE4 => 4,
            EXTREFSELR::ANALOGREFERENCE5 => 5,
            EXTREFSELR::ANALOGREFERENCE6 => 6,
            EXTREFSELR::ANALOGREFERENCE7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTREFSELR {
        match value {
            0 => EXTREFSELR::ANALOGREFERENCE0,
            1 => EXTREFSELR::ANALOGREFERENCE1,
            2 => EXTREFSELR::ANALOGREFERENCE2,
            3 => EXTREFSELR::ANALOGREFERENCE3,
            4 => EXTREFSELR::ANALOGREFERENCE4,
            5 => EXTREFSELR::ANALOGREFERENCE5,
            6 => EXTREFSELR::ANALOGREFERENCE6,
            7 => EXTREFSELR::ANALOGREFERENCE7,
            _ => unreachable!(),
        }
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
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE2`"]
    #[inline]
    pub fn is_analog_reference2(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE2
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE3`"]
    #[inline]
    pub fn is_analog_reference3(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE3
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE4`"]
    #[inline]
    pub fn is_analog_reference4(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE4
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE5`"]
    #[inline]
    pub fn is_analog_reference5(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE5
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE6`"]
    #[inline]
    pub fn is_analog_reference6(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE6
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE7`"]
    #[inline]
    pub fn is_analog_reference7(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE7
    }
}
#[doc = "Values that can be written to the field `EXTREFSEL`"]
pub enum EXTREFSELW {
    #[doc = "Use AIN0 as external analog reference"]
    ANALOGREFERENCE0,
    #[doc = "Use AIN1 as external analog reference"]
    ANALOGREFERENCE1,
    #[doc = "Use AIN2 as external analog reference"]
    ANALOGREFERENCE2,
    #[doc = "Use AIN3 as external analog reference"]
    ANALOGREFERENCE3,
    #[doc = "Use AIN4 as external analog reference"]
    ANALOGREFERENCE4,
    #[doc = "Use AIN5 as external analog reference"]
    ANALOGREFERENCE5,
    #[doc = "Use AIN6 as external analog reference"]
    ANALOGREFERENCE6,
    #[doc = "Use AIN7 as external analog reference"]
    ANALOGREFERENCE7,
}
impl EXTREFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTREFSELW::ANALOGREFERENCE0 => 0,
            EXTREFSELW::ANALOGREFERENCE1 => 1,
            EXTREFSELW::ANALOGREFERENCE2 => 2,
            EXTREFSELW::ANALOGREFERENCE3 => 3,
            EXTREFSELW::ANALOGREFERENCE4 => 4,
            EXTREFSELW::ANALOGREFERENCE5 => 5,
            EXTREFSELW::ANALOGREFERENCE6 => 6,
            EXTREFSELW::ANALOGREFERENCE7 => 7,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Use AIN0 as external analog reference"]
    #[inline]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE0)
    }
    #[doc = "Use AIN1 as external analog reference"]
    #[inline]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE1)
    }
    #[doc = "Use AIN2 as external analog reference"]
    #[inline]
    pub fn analog_reference2(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE2)
    }
    #[doc = "Use AIN3 as external analog reference"]
    #[inline]
    pub fn analog_reference3(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE3)
    }
    #[doc = "Use AIN4 as external analog reference"]
    #[inline]
    pub fn analog_reference4(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE4)
    }
    #[doc = "Use AIN5 as external analog reference"]
    #[inline]
    pub fn analog_reference5(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE5)
    }
    #[doc = "Use AIN6 as external analog reference"]
    #[inline]
    pub fn analog_reference6(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE6)
    }
    #[doc = "Use AIN7 as external analog reference"]
    #[inline]
    pub fn analog_reference7(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE7)
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
    #[doc = "Bits 0:2 - External analog reference select"]
    #[inline]
    pub fn extrefsel(&self) -> EXTREFSELR {
        EXTREFSELR::_from({
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
    #[doc = "Bits 0:2 - External analog reference select"]
    #[inline]
    pub fn extrefsel(&mut self) -> _EXTREFSELW {
        _EXTREFSELW { w: self }
    }
}
