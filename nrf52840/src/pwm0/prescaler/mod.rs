#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESCALER {
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
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "Divide by 1 (16 MHz)"]
    DIV_1,
    #[doc = "Divide by 2 (8 MHz)"]
    DIV_2,
    #[doc = "Divide by 4 (4 MHz)"]
    DIV_4,
    #[doc = "Divide by 8 (2 MHz)"]
    DIV_8,
    #[doc = "Divide by 16 (1 MHz)"]
    DIV_16,
    #[doc = "Divide by 32 (500 kHz)"]
    DIV_32,
    #[doc = "Divide by 64 (250 kHz)"]
    DIV_64,
    #[doc = "Divide by 128 (125 kHz)"]
    DIV_128,
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALERR::DIV_1 => 0,
            PRESCALERR::DIV_2 => 1,
            PRESCALERR::DIV_4 => 2,
            PRESCALERR::DIV_8 => 3,
            PRESCALERR::DIV_16 => 4,
            PRESCALERR::DIV_32 => 5,
            PRESCALERR::DIV_64 => 6,
            PRESCALERR::DIV_128 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALERR {
        match value {
            0 => PRESCALERR::DIV_1,
            1 => PRESCALERR::DIV_2,
            2 => PRESCALERR::DIV_4,
            3 => PRESCALERR::DIV_8,
            4 => PRESCALERR::DIV_16,
            5 => PRESCALERR::DIV_32,
            6 => PRESCALERR::DIV_64,
            7 => PRESCALERR::DIV_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline]
    pub fn is_div_1(&self) -> bool {
        *self == PRESCALERR::DIV_1
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline]
    pub fn is_div_2(&self) -> bool {
        *self == PRESCALERR::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline]
    pub fn is_div_4(&self) -> bool {
        *self == PRESCALERR::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline]
    pub fn is_div_8(&self) -> bool {
        *self == PRESCALERR::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline]
    pub fn is_div_16(&self) -> bool {
        *self == PRESCALERR::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline]
    pub fn is_div_32(&self) -> bool {
        *self == PRESCALERR::DIV_32
    }
    #[doc = "Checks if the value of the field is `DIV_64`"]
    #[inline]
    pub fn is_div_64(&self) -> bool {
        *self == PRESCALERR::DIV_64
    }
    #[doc = "Checks if the value of the field is `DIV_128`"]
    #[inline]
    pub fn is_div_128(&self) -> bool {
        *self == PRESCALERR::DIV_128
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
pub enum PRESCALERW {
    #[doc = "Divide by 1 (16 MHz)"]
    DIV_1,
    #[doc = "Divide by 2 (8 MHz)"]
    DIV_2,
    #[doc = "Divide by 4 (4 MHz)"]
    DIV_4,
    #[doc = "Divide by 8 (2 MHz)"]
    DIV_8,
    #[doc = "Divide by 16 (1 MHz)"]
    DIV_16,
    #[doc = "Divide by 32 (500 kHz)"]
    DIV_32,
    #[doc = "Divide by 64 (250 kHz)"]
    DIV_64,
    #[doc = "Divide by 128 (125 kHz)"]
    DIV_128,
}
impl PRESCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALERW::DIV_1 => 0,
            PRESCALERW::DIV_2 => 1,
            PRESCALERW::DIV_4 => 2,
            PRESCALERW::DIV_8 => 3,
            PRESCALERW::DIV_16 => 4,
            PRESCALERW::DIV_32 => 5,
            PRESCALERW::DIV_64 => 6,
            PRESCALERW::DIV_128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1 (16 MHz)"]
    #[inline]
    pub fn div_1(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV_1)
    }
    #[doc = "Divide by 2 (8 MHz)"]
    #[inline]
    pub fn div_2(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV_2)
    }
    #[doc = "Divide by 4 (4 MHz)"]
    #[inline]
    pub fn div_4(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV_4)
    }
    #[doc = "Divide by 8 (2 MHz)"]
    #[inline]
    pub fn div_8(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV_8)
    }
    #[doc = "Divide by 16 (1 MHz)"]
    #[inline]
    pub fn div_16(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV_16)
    }
    #[doc = "Divide by 32 (500 kHz)"]
    #[inline]
    pub fn div_32(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV_32)
    }
    #[doc = "Divide by 64 (250 kHz)"]
    #[inline]
    pub fn div_64(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV_64)
    }
    #[doc = "Divide by 128 (125 kHz)"]
    #[inline]
    pub fn div_128(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV_128)
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
    #[doc = "Bits 0:2 - Prescaler of PWM_CLK"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from({
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
    #[doc = "Bits 0:2 - Prescaler of PWM_CLK"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
}
