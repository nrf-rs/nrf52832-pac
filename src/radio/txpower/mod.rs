#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXPOWER {
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
#[doc = "Possible values of the field `TXPOWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPOWERR {
    #[doc = "+4 dBm"]
    POS4DBM,
    #[doc = "+3 dBm"]
    POS3DBM,
    #[doc = "0 dBm"]
    _0DBM,
    #[doc = "-4 dBm"]
    NEG4DBM,
    #[doc = "-8 dBm"]
    NEG8DBM,
    #[doc = "-12 dBm"]
    NEG12DBM,
    #[doc = "-16 dBm"]
    NEG16DBM,
    #[doc = "-20 dBm"]
    NEG20DBM,
    #[doc = "-40 dBm"]
    NEG40DBM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXPOWERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXPOWERR::POS4DBM => 4,
            TXPOWERR::POS3DBM => 3,
            TXPOWERR::_0DBM => 0,
            TXPOWERR::NEG4DBM => 252,
            TXPOWERR::NEG8DBM => 248,
            TXPOWERR::NEG12DBM => 244,
            TXPOWERR::NEG16DBM => 240,
            TXPOWERR::NEG20DBM => 236,
            TXPOWERR::NEG40DBM => 216,
            TXPOWERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXPOWERR {
        match value {
            4 => TXPOWERR::POS4DBM,
            3 => TXPOWERR::POS3DBM,
            0 => TXPOWERR::_0DBM,
            252 => TXPOWERR::NEG4DBM,
            248 => TXPOWERR::NEG8DBM,
            244 => TXPOWERR::NEG12DBM,
            240 => TXPOWERR::NEG16DBM,
            236 => TXPOWERR::NEG20DBM,
            216 => TXPOWERR::NEG40DBM,
            i => TXPOWERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POS4DBM`"]
    #[inline]
    pub fn is_pos4d_bm(&self) -> bool {
        *self == TXPOWERR::POS4DBM
    }
    #[doc = "Checks if the value of the field is `POS3DBM`"]
    #[inline]
    pub fn is_pos3d_bm(&self) -> bool {
        *self == TXPOWERR::POS3DBM
    }
    #[doc = "Checks if the value of the field is `_0DBM`"]
    #[inline]
    pub fn is_0d_bm(&self) -> bool {
        *self == TXPOWERR::_0DBM
    }
    #[doc = "Checks if the value of the field is `NEG4DBM`"]
    #[inline]
    pub fn is_neg4d_bm(&self) -> bool {
        *self == TXPOWERR::NEG4DBM
    }
    #[doc = "Checks if the value of the field is `NEG8DBM`"]
    #[inline]
    pub fn is_neg8d_bm(&self) -> bool {
        *self == TXPOWERR::NEG8DBM
    }
    #[doc = "Checks if the value of the field is `NEG12DBM`"]
    #[inline]
    pub fn is_neg12d_bm(&self) -> bool {
        *self == TXPOWERR::NEG12DBM
    }
    #[doc = "Checks if the value of the field is `NEG16DBM`"]
    #[inline]
    pub fn is_neg16d_bm(&self) -> bool {
        *self == TXPOWERR::NEG16DBM
    }
    #[doc = "Checks if the value of the field is `NEG20DBM`"]
    #[inline]
    pub fn is_neg20d_bm(&self) -> bool {
        *self == TXPOWERR::NEG20DBM
    }
    #[doc = "Checks if the value of the field is `NEG40DBM`"]
    #[inline]
    pub fn is_neg40d_bm(&self) -> bool {
        *self == TXPOWERR::NEG40DBM
    }
}
#[doc = "Values that can be written to the field `TXPOWER`"]
pub enum TXPOWERW {
    #[doc = "+4 dBm"]
    POS4DBM,
    #[doc = "+3 dBm"]
    POS3DBM,
    #[doc = "0 dBm"]
    _0DBM,
    #[doc = "-4 dBm"]
    NEG4DBM,
    #[doc = "-8 dBm"]
    NEG8DBM,
    #[doc = "-12 dBm"]
    NEG12DBM,
    #[doc = "-16 dBm"]
    NEG16DBM,
    #[doc = "-20 dBm"]
    NEG20DBM,
    #[doc = "-40 dBm"]
    NEG40DBM,
}
impl TXPOWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXPOWERW::POS4DBM => 4,
            TXPOWERW::POS3DBM => 3,
            TXPOWERW::_0DBM => 0,
            TXPOWERW::NEG4DBM => 252,
            TXPOWERW::NEG8DBM => 248,
            TXPOWERW::NEG12DBM => 244,
            TXPOWERW::NEG16DBM => 240,
            TXPOWERW::NEG20DBM => 236,
            TXPOWERW::NEG40DBM => 216,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPOWERW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPOWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPOWERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "+4 dBm"]
    #[inline]
    pub fn pos4d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::POS4DBM)
    }
    #[doc = "+3 dBm"]
    #[inline]
    pub fn pos3d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::POS3DBM)
    }
    #[doc = "0 dBm"]
    #[inline]
    pub fn _0d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::_0DBM)
    }
    #[doc = "-4 dBm"]
    #[inline]
    pub fn neg4d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::NEG4DBM)
    }
    #[doc = "-8 dBm"]
    #[inline]
    pub fn neg8d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::NEG8DBM)
    }
    #[doc = "-12 dBm"]
    #[inline]
    pub fn neg12d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::NEG12DBM)
    }
    #[doc = "-16 dBm"]
    #[inline]
    pub fn neg16d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::NEG16DBM)
    }
    #[doc = "-20 dBm"]
    #[inline]
    pub fn neg20d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::NEG20DBM)
    }
    #[doc = "-40 dBm"]
    #[inline]
    pub fn neg40d_bm(self) -> &'a mut W {
        self.variant(TXPOWERW::NEG40DBM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - RADIO output power."]
    #[inline]
    pub fn txpower(&self) -> TXPOWERR {
        TXPOWERR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - RADIO output power."]
    #[inline]
    pub fn txpower(&mut self) -> _TXPOWERW {
        _TXPOWERW { w: self }
    }
}
