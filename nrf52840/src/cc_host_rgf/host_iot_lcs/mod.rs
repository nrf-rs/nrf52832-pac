#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HOST_IOT_LCS {
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
#[doc = "Possible values of the field `LCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCSR {
    #[doc = "CC310 operates in debug mode"]
    DEBUG,
    #[doc = "CC310 operates in secure mode"]
    SECURE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCSR::DEBUG => 0,
            LCSR::SECURE => 2,
            LCSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCSR {
        match value {
            0 => LCSR::DEBUG,
            2 => LCSR::SECURE,
            i => LCSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEBUG`"]
    #[inline]
    pub fn is_debug(&self) -> bool {
        *self == LCSR::DEBUG
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == LCSR::SECURE
    }
}
#[doc = "Possible values of the field `LCS_IS_VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCS_IS_VALIDR {
    #[doc = "A valid LCS is not yet retained in the CRYPTOCELL AO power domain"]
    INVALID,
    #[doc = "A valid LCS is successfully retained in the CRYPTOCELL AO power domain"]
    VALID,
}
impl LCS_IS_VALIDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LCS_IS_VALIDR::INVALID => false,
            LCS_IS_VALIDR::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCS_IS_VALIDR {
        match value {
            false => LCS_IS_VALIDR::INVALID,
            true => LCS_IS_VALIDR::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline]
    pub fn is_invalid(&self) -> bool {
        *self == LCS_IS_VALIDR::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == LCS_IS_VALIDR::VALID
    }
}
#[doc = "Values that can be written to the field `LCS`"]
pub enum LCSW {
    #[doc = "CC310 operates in debug mode"]
    DEBUG,
    #[doc = "CC310 operates in secure mode"]
    SECURE,
}
impl LCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCSW::DEBUG => 0,
            LCSW::SECURE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCSW<'a> {
    w: &'a mut W,
}
impl<'a> _LCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CC310 operates in debug mode"]
    #[inline]
    pub fn debug(self) -> &'a mut W {
        self.variant(LCSW::DEBUG)
    }
    #[doc = "CC310 operates in secure mode"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(LCSW::SECURE)
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
#[doc = "Values that can be written to the field `LCS_IS_VALID`"]
pub enum LCS_IS_VALIDW {
    #[doc = "A valid LCS is not yet retained in the CRYPTOCELL AO power domain"]
    INVALID,
    #[doc = "A valid LCS is successfully retained in the CRYPTOCELL AO power domain"]
    VALID,
}
impl LCS_IS_VALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LCS_IS_VALIDW::INVALID => false,
            LCS_IS_VALIDW::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCS_IS_VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _LCS_IS_VALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCS_IS_VALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A valid LCS is not yet retained in the CRYPTOCELL AO power domain"]
    #[inline]
    pub fn invalid(self) -> &'a mut W {
        self.variant(LCS_IS_VALIDW::INVALID)
    }
    #[doc = "A valid LCS is successfully retained in the CRYPTOCELL AO power domain"]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(LCS_IS_VALIDW::VALID)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - Lifecycle state value. This field is write-once per reset."]
    #[inline]
    pub fn lcs(&self) -> LCSR {
        LCSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - This field is read-only and indicates if CRYPTOCELL LCS has been successfully configured since last reset"]
    #[inline]
    pub fn lcs_is_valid(&self) -> LCS_IS_VALIDR {
        LCS_IS_VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Lifecycle state value. This field is write-once per reset."]
    #[inline]
    pub fn lcs(&mut self) -> _LCSW {
        _LCSW { w: self }
    }
    #[doc = "Bit 8 - This field is read-only and indicates if CRYPTOCELL LCS has been successfully configured since last reset"]
    #[inline]
    pub fn lcs_is_valid(&mut self) -> _LCS_IS_VALIDW {
        _LCS_IS_VALIDW { w: self }
    }
}
