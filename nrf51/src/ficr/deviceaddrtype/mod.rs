#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVICEADDRTYPE {
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
#[doc = "Possible values of the field `DEVICEADDRTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVICEADDRTYPER {
    #[doc = "Public address."]
    PUBLIC,
    #[doc = "Random address."]
    RANDOM,
}
impl DEVICEADDRTYPER {
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
            DEVICEADDRTYPER::PUBLIC => false,
            DEVICEADDRTYPER::RANDOM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEVICEADDRTYPER {
        match value {
            false => DEVICEADDRTYPER::PUBLIC,
            true => DEVICEADDRTYPER::RANDOM,
        }
    }
    #[doc = "Checks if the value of the field is `PUBLIC`"]
    #[inline]
    pub fn is_public(&self) -> bool {
        *self == DEVICEADDRTYPER::PUBLIC
    }
    #[doc = "Checks if the value of the field is `RANDOM`"]
    #[inline]
    pub fn is_random(&self) -> bool {
        *self == DEVICEADDRTYPER::RANDOM
    }
}
#[doc = "Values that can be written to the field `DEVICEADDRTYPE`"]
pub enum DEVICEADDRTYPEW {
    #[doc = "Public address."]
    PUBLIC,
    #[doc = "Random address."]
    RANDOM,
}
impl DEVICEADDRTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEVICEADDRTYPEW::PUBLIC => false,
            DEVICEADDRTYPEW::RANDOM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVICEADDRTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVICEADDRTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVICEADDRTYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Public address."]
    #[inline]
    pub fn public(self) -> &'a mut W {
        self.variant(DEVICEADDRTYPEW::PUBLIC)
    }
    #[doc = "Random address."]
    #[inline]
    pub fn random(self) -> &'a mut W {
        self.variant(DEVICEADDRTYPEW::RANDOM)
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
    #[doc = "Bit 0 - Device address type."]
    #[inline]
    pub fn deviceaddrtype(&self) -> DEVICEADDRTYPER {
        DEVICEADDRTYPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Device address type."]
    #[inline]
    pub fn deviceaddrtype(&mut self) -> _DEVICEADDRTYPEW {
        _DEVICEADDRTYPEW { w: self }
    }
}
