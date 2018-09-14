#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPSTALL {
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
#[doc = r" Proxy"]
pub struct _EPW<'a> {
    w: &'a mut W,
}
impl<'a> _EPW<'a> {
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
#[doc = "Values that can be written to the field `IO`"]
pub enum IOW {
    #[doc = "Selects OUT endpoint"]
    OUT,
    #[doc = "Selects IN endpoint"]
    IN,
}
impl IOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOW::OUT => false,
            IOW::IN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOW<'a> {
    w: &'a mut W,
}
impl<'a> _IOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects OUT endpoint"]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IOW::OUT)
    }
    #[doc = "Selects IN endpoint"]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IOW::IN)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STALL`"]
pub enum STALLW {
    #[doc = "Don't stall selected endpoint"]
    UNSTALL,
    #[doc = "Stall selected endpoint"]
    STALL,
}
impl STALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALLW::UNSTALL => false,
            STALLW::STALL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't stall selected endpoint"]
    #[inline]
    pub fn un_stall(self) -> &'a mut W {
        self.variant(STALLW::UNSTALL)
    }
    #[doc = "Stall selected endpoint"]
    #[inline]
    pub fn stall(self) -> &'a mut W {
        self.variant(STALLW::STALL)
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
    #[doc = "Bits 0:2 - Select endpoint number"]
    #[inline]
    pub fn ep(&mut self) -> _EPW {
        _EPW { w: self }
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline]
    pub fn io(&mut self) -> _IOW {
        _IOW { w: self }
    }
    #[doc = "Bit 8 - Stall selected endpoint"]
    #[inline]
    pub fn stall(&mut self) -> _STALLW {
        _STALLW { w: self }
    }
}
