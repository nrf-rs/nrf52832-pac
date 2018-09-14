#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RR {
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
}
#[doc = "Values that can be written to the field `RR`"]
pub enum RRW {
    #[doc = "Value to request a reload of the watchdog timer"]
    RELOAD,
}
impl RRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            RRW::RELOAD => 1850885685,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRW<'a> {
    w: &'a mut W,
}
impl<'a> _RRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Value to request a reload of the watchdog timer"]
    #[inline]
    pub fn reload(self) -> &'a mut W {
        self.variant(RRW::RELOAD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bits 0:31 - Reload request register"]
    #[inline]
    pub fn rr(&mut self) -> _RRW {
        _RRW { w: self }
    }
}
