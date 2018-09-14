#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HOST_CRYPTOKEY_SEL {
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
#[doc = "Possible values of the field `HOST_CRYPTOKEY_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CRYPTOKEY_SELR {
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    K_DR,
    #[doc = "Use hard-coded RTL key K_PRTL"]
    K_PRTL,
    #[doc = "Use provided session key"]
    SESSION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HOST_CRYPTOKEY_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HOST_CRYPTOKEY_SELR::K_DR => 0,
            HOST_CRYPTOKEY_SELR::K_PRTL => 1,
            HOST_CRYPTOKEY_SELR::SESSION => 2,
            HOST_CRYPTOKEY_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HOST_CRYPTOKEY_SELR {
        match value {
            0 => HOST_CRYPTOKEY_SELR::K_DR,
            1 => HOST_CRYPTOKEY_SELR::K_PRTL,
            2 => HOST_CRYPTOKEY_SELR::SESSION,
            i => HOST_CRYPTOKEY_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `K_DR`"]
    #[inline]
    pub fn is_k_dr(&self) -> bool {
        *self == HOST_CRYPTOKEY_SELR::K_DR
    }
    #[doc = "Checks if the value of the field is `K_PRTL`"]
    #[inline]
    pub fn is_k_prtl(&self) -> bool {
        *self == HOST_CRYPTOKEY_SELR::K_PRTL
    }
    #[doc = "Checks if the value of the field is `SESSION`"]
    #[inline]
    pub fn is_session(&self) -> bool {
        *self == HOST_CRYPTOKEY_SELR::SESSION
    }
}
#[doc = "Values that can be written to the field `HOST_CRYPTOKEY_SEL`"]
pub enum HOST_CRYPTOKEY_SELW {
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    K_DR,
    #[doc = "Use hard-coded RTL key K_PRTL"]
    K_PRTL,
    #[doc = "Use provided session key"]
    SESSION,
}
impl HOST_CRYPTOKEY_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HOST_CRYPTOKEY_SELW::K_DR => 0,
            HOST_CRYPTOKEY_SELW::K_PRTL => 1,
            HOST_CRYPTOKEY_SELW::SESSION => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOST_CRYPTOKEY_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _HOST_CRYPTOKEY_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOST_CRYPTOKEY_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    #[inline]
    pub fn k_dr(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SELW::K_DR)
    }
    #[doc = "Use hard-coded RTL key K_PRTL"]
    #[inline]
    pub fn k_prtl(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SELW::K_PRTL)
    }
    #[doc = "Use provided session key"]
    #[inline]
    pub fn session(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SELW::SESSION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Select the source of the HW key that is used by the AES engine"]
    #[inline]
    pub fn host_cryptokey_sel(&self) -> HOST_CRYPTOKEY_SELR {
        HOST_CRYPTOKEY_SELR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Select the source of the HW key that is used by the AES engine"]
    #[inline]
    pub fn host_cryptokey_sel(&mut self) -> _HOST_CRYPTOKEY_SELW {
        _HOST_CRYPTOKEY_SELW { w: self }
    }
}
