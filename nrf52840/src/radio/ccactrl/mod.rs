#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCACTRL {
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
#[doc = "Possible values of the field `CCAMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAMODER {
    #[doc = "Energy above threshold"]
    EDMODE,
    #[doc = "Carrier seen"]
    CARRIERMODE,
    #[doc = "Energy above threshold AND carrier seen"]
    CARRIERANDEDMODE,
    #[doc = "Energy above threshold OR carrier seen"]
    CARRIEROREDMODE,
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    EDMODETEST1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CCAMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CCAMODER::EDMODE => 0,
            CCAMODER::CARRIERMODE => 1,
            CCAMODER::CARRIERANDEDMODE => 2,
            CCAMODER::CARRIEROREDMODE => 3,
            CCAMODER::EDMODETEST1 => 4,
            CCAMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CCAMODER {
        match value {
            0 => CCAMODER::EDMODE,
            1 => CCAMODER::CARRIERMODE,
            2 => CCAMODER::CARRIERANDEDMODE,
            3 => CCAMODER::CARRIEROREDMODE,
            4 => CCAMODER::EDMODETEST1,
            i => CCAMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EDMODE`"]
    #[inline]
    pub fn is_ed_mode(&self) -> bool {
        *self == CCAMODER::EDMODE
    }
    #[doc = "Checks if the value of the field is `CARRIERMODE`"]
    #[inline]
    pub fn is_carrier_mode(&self) -> bool {
        *self == CCAMODER::CARRIERMODE
    }
    #[doc = "Checks if the value of the field is `CARRIERANDEDMODE`"]
    #[inline]
    pub fn is_carrier_and_ed_mode(&self) -> bool {
        *self == CCAMODER::CARRIERANDEDMODE
    }
    #[doc = "Checks if the value of the field is `CARRIEROREDMODE`"]
    #[inline]
    pub fn is_carrier_or_ed_mode(&self) -> bool {
        *self == CCAMODER::CARRIEROREDMODE
    }
    #[doc = "Checks if the value of the field is `EDMODETEST1`"]
    #[inline]
    pub fn is_ed_mode_test1(&self) -> bool {
        *self == CCAMODER::EDMODETEST1
    }
}
#[doc = r" Value of the field"]
pub struct CCAEDTHRESR {
    bits: u8,
}
impl CCAEDTHRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCACORRTHRESR {
    bits: u8,
}
impl CCACORRTHRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCACORRCNTR {
    bits: u8,
}
impl CCACORRCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CCAMODE`"]
pub enum CCAMODEW {
    #[doc = "Energy above threshold"]
    EDMODE,
    #[doc = "Carrier seen"]
    CARRIERMODE,
    #[doc = "Energy above threshold AND carrier seen"]
    CARRIERANDEDMODE,
    #[doc = "Energy above threshold OR carrier seen"]
    CARRIEROREDMODE,
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    EDMODETEST1,
}
impl CCAMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CCAMODEW::EDMODE => 0,
            CCAMODEW::CARRIERMODE => 1,
            CCAMODEW::CARRIERANDEDMODE => 2,
            CCAMODEW::CARRIEROREDMODE => 3,
            CCAMODEW::EDMODETEST1 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCAMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCAMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCAMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Energy above threshold"]
    #[inline]
    pub fn ed_mode(self) -> &'a mut W {
        self.variant(CCAMODEW::EDMODE)
    }
    #[doc = "Carrier seen"]
    #[inline]
    pub fn carrier_mode(self) -> &'a mut W {
        self.variant(CCAMODEW::CARRIERMODE)
    }
    #[doc = "Energy above threshold AND carrier seen"]
    #[inline]
    pub fn carrier_and_ed_mode(self) -> &'a mut W {
        self.variant(CCAMODEW::CARRIERANDEDMODE)
    }
    #[doc = "Energy above threshold OR carrier seen"]
    #[inline]
    pub fn carrier_or_ed_mode(self) -> &'a mut W {
        self.variant(CCAMODEW::CARRIEROREDMODE)
    }
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    #[inline]
    pub fn ed_mode_test1(self) -> &'a mut W {
        self.variant(CCAMODEW::EDMODETEST1)
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
#[doc = r" Proxy"]
pub struct _CCAEDTHRESW<'a> {
    w: &'a mut W,
}
impl<'a> _CCAEDTHRESW<'a> {
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
#[doc = r" Proxy"]
pub struct _CCACORRTHRESW<'a> {
    w: &'a mut W,
}
impl<'a> _CCACORRTHRESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCACORRCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CCACORRCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline]
    pub fn ccamode(&self) -> CCAMODER {
        CCAMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline]
    pub fn ccaedthres(&self) -> CCAEDTHRESR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCAEDTHRESR { bits }
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode and CarrierOrEdMode."]
    #[inline]
    pub fn ccacorrthres(&self) -> CCACORRTHRESR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCACORRTHRESR { bits }
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline]
    pub fn ccacorrcnt(&self) -> CCACORRCNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCACORRCNTR { bits }
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
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline]
    pub fn ccamode(&mut self) -> _CCAMODEW {
        _CCAMODEW { w: self }
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline]
    pub fn ccaedthres(&mut self) -> _CCAEDTHRESW {
        _CCAEDTHRESW { w: self }
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode and CarrierOrEdMode."]
    #[inline]
    pub fn ccacorrthres(&mut self) -> _CCACORRTHRESW {
        _CCACORRTHRESW { w: self }
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline]
    pub fn ccacorrcnt(&mut self) -> _CCACORRCNTW {
        _CCACORRCNTW { w: self }
    }
}
