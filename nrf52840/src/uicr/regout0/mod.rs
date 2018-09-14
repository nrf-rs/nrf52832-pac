#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REGOUT0 {
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
#[doc = "Possible values of the field `VOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOUTR {
    #[doc = "1.8 V"]
    _1V8,
    #[doc = "2.1 V"]
    _2V1,
    #[doc = "2.4 V"]
    _2V4,
    #[doc = "2.7 V"]
    _2V7,
    #[doc = "3.0 V"]
    _3V0,
    #[doc = "3.3 V"]
    _3V3,
    #[doc = "Default voltage: 1.8 V"]
    DEFAULT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VOUTR::_1V8 => 0,
            VOUTR::_2V1 => 1,
            VOUTR::_2V4 => 2,
            VOUTR::_2V7 => 3,
            VOUTR::_3V0 => 4,
            VOUTR::_3V3 => 5,
            VOUTR::DEFAULT => 7,
            VOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VOUTR {
        match value {
            0 => VOUTR::_1V8,
            1 => VOUTR::_2V1,
            2 => VOUTR::_2V4,
            3 => VOUTR::_2V7,
            4 => VOUTR::_3V0,
            5 => VOUTR::_3V3,
            7 => VOUTR::DEFAULT,
            i => VOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline]
    pub fn is_1v8(&self) -> bool {
        *self == VOUTR::_1V8
    }
    #[doc = "Checks if the value of the field is `_2V1`"]
    #[inline]
    pub fn is_2v1(&self) -> bool {
        *self == VOUTR::_2V1
    }
    #[doc = "Checks if the value of the field is `_2V4`"]
    #[inline]
    pub fn is_2v4(&self) -> bool {
        *self == VOUTR::_2V4
    }
    #[doc = "Checks if the value of the field is `_2V7`"]
    #[inline]
    pub fn is_2v7(&self) -> bool {
        *self == VOUTR::_2V7
    }
    #[doc = "Checks if the value of the field is `_3V0`"]
    #[inline]
    pub fn is_3v0(&self) -> bool {
        *self == VOUTR::_3V0
    }
    #[doc = "Checks if the value of the field is `_3V3`"]
    #[inline]
    pub fn is_3v3(&self) -> bool {
        *self == VOUTR::_3V3
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == VOUTR::DEFAULT
    }
}
#[doc = "Values that can be written to the field `VOUT`"]
pub enum VOUTW {
    #[doc = "1.8 V"]
    _1V8,
    #[doc = "2.1 V"]
    _2V1,
    #[doc = "2.4 V"]
    _2V4,
    #[doc = "2.7 V"]
    _2V7,
    #[doc = "3.0 V"]
    _3V0,
    #[doc = "3.3 V"]
    _3V3,
    #[doc = "Default voltage: 1.8 V"]
    DEFAULT,
}
impl VOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VOUTW::_1V8 => 0,
            VOUTW::_2V1 => 1,
            VOUTW::_2V4 => 2,
            VOUTW::_2V7 => 3,
            VOUTW::_3V0 => 4,
            VOUTW::_3V3 => 5,
            VOUTW::DEFAULT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _VOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1.8 V"]
    #[inline]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(VOUTW::_1V8)
    }
    #[doc = "2.1 V"]
    #[inline]
    pub fn _2v1(self) -> &'a mut W {
        self.variant(VOUTW::_2V1)
    }
    #[doc = "2.4 V"]
    #[inline]
    pub fn _2v4(self) -> &'a mut W {
        self.variant(VOUTW::_2V4)
    }
    #[doc = "2.7 V"]
    #[inline]
    pub fn _2v7(self) -> &'a mut W {
        self.variant(VOUTW::_2V7)
    }
    #[doc = "3.0 V"]
    #[inline]
    pub fn _3v0(self) -> &'a mut W {
        self.variant(VOUTW::_3V0)
    }
    #[doc = "3.3 V"]
    #[inline]
    pub fn _3v3(self) -> &'a mut W {
        self.variant(VOUTW::_3V3)
    }
    #[doc = "Default voltage: 1.8 V"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(VOUTW::DEFAULT)
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
    #[doc = "Bits 0:2 - Output voltage from of REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VEXDIF."]
    #[inline]
    pub fn vout(&self) -> VOUTR {
        VOUTR::_from({
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
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Output voltage from of REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VEXDIF."]
    #[inline]
    pub fn vout(&mut self) -> _VOUTW {
        _VOUTW { w: self }
    }
}
