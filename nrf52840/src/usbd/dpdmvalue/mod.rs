#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DPDMVALUE {
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
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    RESUME,
    #[doc = "D+ forced high, D- forced low (J state)"]
    J,
    #[doc = "D+ forced low, D- forced high (K state)"]
    K,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::RESUME => 1,
            STATER::J => 2,
            STATER::K => 4,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            1 => STATER::RESUME,
            2 => STATER::J,
            4 => STATER::K,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline]
    pub fn is_resume(&self) -> bool {
        *self == STATER::RESUME
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline]
    pub fn is_j(&self) -> bool {
        *self == STATER::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline]
    pub fn is_k(&self) -> bool {
        *self == STATER::K
    }
}
#[doc = "Values that can be written to the field `STATE`"]
pub enum STATEW {
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    RESUME,
    #[doc = "D+ forced high, D- forced low (J state)"]
    J,
    #[doc = "D+ forced low, D- forced high (K state)"]
    K,
}
impl STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STATEW::RESUME => 1,
            STATEW::J => 2,
            STATEW::K => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _STATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    #[inline]
    pub fn resume(self) -> &'a mut W {
        self.variant(STATEW::RESUME)
    }
    #[doc = "D+ forced high, D- forced low (J state)"]
    #[inline]
    pub fn j(self) -> &'a mut W {
        self.variant(STATEW::J)
    }
    #[doc = "D+ forced low, D- forced high (K state)"]
    #[inline]
    pub fn k(self) -> &'a mut W {
        self.variant(STATEW::K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:4 - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 31;
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
    #[doc = "Bits 0:4 - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline]
    pub fn state(&mut self) -> _STATEW {
        _STATEW { w: self }
    }
}
