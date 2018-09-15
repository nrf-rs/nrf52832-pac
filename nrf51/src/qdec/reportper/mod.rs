#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REPORTPER {
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
#[doc = "Possible values of the field `REPORTPER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTPERR {
    #[doc = "10 samples per report."]
    _10SMPL,
    #[doc = "40 samples per report."]
    _40SMPL,
    #[doc = "80 samples per report."]
    _80SMPL,
    #[doc = "120 samples per report."]
    _120SMPL,
    #[doc = "160 samples per report."]
    _160SMPL,
    #[doc = "200 samples per report."]
    _200SMPL,
    #[doc = "240 samples per report."]
    _240SMPL,
    #[doc = "280 samples per report."]
    _280SMPL,
}
impl REPORTPERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REPORTPERR::_10SMPL => 0,
            REPORTPERR::_40SMPL => 1,
            REPORTPERR::_80SMPL => 2,
            REPORTPERR::_120SMPL => 3,
            REPORTPERR::_160SMPL => 4,
            REPORTPERR::_200SMPL => 5,
            REPORTPERR::_240SMPL => 6,
            REPORTPERR::_280SMPL => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REPORTPERR {
        match value {
            0 => REPORTPERR::_10SMPL,
            1 => REPORTPERR::_40SMPL,
            2 => REPORTPERR::_80SMPL,
            3 => REPORTPERR::_120SMPL,
            4 => REPORTPERR::_160SMPL,
            5 => REPORTPERR::_200SMPL,
            6 => REPORTPERR::_240SMPL,
            7 => REPORTPERR::_280SMPL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_10SMPL`"]
    #[inline]
    pub fn is_10smpl(&self) -> bool {
        *self == REPORTPERR::_10SMPL
    }
    #[doc = "Checks if the value of the field is `_40SMPL`"]
    #[inline]
    pub fn is_40smpl(&self) -> bool {
        *self == REPORTPERR::_40SMPL
    }
    #[doc = "Checks if the value of the field is `_80SMPL`"]
    #[inline]
    pub fn is_80smpl(&self) -> bool {
        *self == REPORTPERR::_80SMPL
    }
    #[doc = "Checks if the value of the field is `_120SMPL`"]
    #[inline]
    pub fn is_120smpl(&self) -> bool {
        *self == REPORTPERR::_120SMPL
    }
    #[doc = "Checks if the value of the field is `_160SMPL`"]
    #[inline]
    pub fn is_160smpl(&self) -> bool {
        *self == REPORTPERR::_160SMPL
    }
    #[doc = "Checks if the value of the field is `_200SMPL`"]
    #[inline]
    pub fn is_200smpl(&self) -> bool {
        *self == REPORTPERR::_200SMPL
    }
    #[doc = "Checks if the value of the field is `_240SMPL`"]
    #[inline]
    pub fn is_240smpl(&self) -> bool {
        *self == REPORTPERR::_240SMPL
    }
    #[doc = "Checks if the value of the field is `_280SMPL`"]
    #[inline]
    pub fn is_280smpl(&self) -> bool {
        *self == REPORTPERR::_280SMPL
    }
}
#[doc = "Values that can be written to the field `REPORTPER`"]
pub enum REPORTPERW {
    #[doc = "10 samples per report."]
    _10SMPL,
    #[doc = "40 samples per report."]
    _40SMPL,
    #[doc = "80 samples per report."]
    _80SMPL,
    #[doc = "120 samples per report."]
    _120SMPL,
    #[doc = "160 samples per report."]
    _160SMPL,
    #[doc = "200 samples per report."]
    _200SMPL,
    #[doc = "240 samples per report."]
    _240SMPL,
    #[doc = "280 samples per report."]
    _280SMPL,
}
impl REPORTPERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REPORTPERW::_10SMPL => 0,
            REPORTPERW::_40SMPL => 1,
            REPORTPERW::_80SMPL => 2,
            REPORTPERW::_120SMPL => 3,
            REPORTPERW::_160SMPL => 4,
            REPORTPERW::_200SMPL => 5,
            REPORTPERW::_240SMPL => 6,
            REPORTPERW::_280SMPL => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REPORTPERW<'a> {
    w: &'a mut W,
}
impl<'a> _REPORTPERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REPORTPERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "10 samples per report."]
    #[inline]
    pub fn _10smpl(self) -> &'a mut W {
        self.variant(REPORTPERW::_10SMPL)
    }
    #[doc = "40 samples per report."]
    #[inline]
    pub fn _40smpl(self) -> &'a mut W {
        self.variant(REPORTPERW::_40SMPL)
    }
    #[doc = "80 samples per report."]
    #[inline]
    pub fn _80smpl(self) -> &'a mut W {
        self.variant(REPORTPERW::_80SMPL)
    }
    #[doc = "120 samples per report."]
    #[inline]
    pub fn _120smpl(self) -> &'a mut W {
        self.variant(REPORTPERW::_120SMPL)
    }
    #[doc = "160 samples per report."]
    #[inline]
    pub fn _160smpl(self) -> &'a mut W {
        self.variant(REPORTPERW::_160SMPL)
    }
    #[doc = "200 samples per report."]
    #[inline]
    pub fn _200smpl(self) -> &'a mut W {
        self.variant(REPORTPERW::_200SMPL)
    }
    #[doc = "240 samples per report."]
    #[inline]
    pub fn _240smpl(self) -> &'a mut W {
        self.variant(REPORTPERW::_240SMPL)
    }
    #[doc = "280 samples per report."]
    #[inline]
    pub fn _280smpl(self) -> &'a mut W {
        self.variant(REPORTPERW::_280SMPL)
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
    #[doc = "Bits 0:2 - Number of samples to generate an EVENT_REPORTRDY."]
    #[inline]
    pub fn reportper(&self) -> REPORTPERR {
        REPORTPERR::_from({
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
    #[doc = "Bits 0:2 - Number of samples to generate an EVENT_REPORTRDY."]
    #[inline]
    pub fn reportper(&mut self) -> _REPORTPERW {
        _REPORTPERW { w: self }
    }
}
