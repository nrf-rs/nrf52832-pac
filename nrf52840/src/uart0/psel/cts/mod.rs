#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTS {
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
#[doc = r" Value of the field"]
pub struct PINR {
    bits: u8,
}
impl PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PORTR {
    bits: bool,
}
impl PORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `CONNECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONNECTR {
    #[doc = "Disconnect"]
    DISCONNECTED,
    #[doc = "Connect"]
    CONNECTED,
}
impl CONNECTR {
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
            CONNECTR::DISCONNECTED => true,
            CONNECTR::CONNECTED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONNECTR {
        match value {
            true => CONNECTR::DISCONNECTED,
            false => CONNECTR::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline]
    pub fn is_disconnected(&self) -> bool {
        *self == CONNECTR::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline]
    pub fn is_connected(&self) -> bool {
        *self == CONNECTR::CONNECTED
    }
}
#[doc = r" Proxy"]
pub struct _PINW<'a> {
    w: &'a mut W,
}
impl<'a> _PINW<'a> {
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
#[doc = r" Proxy"]
pub struct _PORTW<'a> {
    w: &'a mut W,
}
impl<'a> _PORTW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CONNECT`"]
pub enum CONNECTW {
    #[doc = "Disconnect"]
    DISCONNECTED,
    #[doc = "Connect"]
    CONNECTED,
}
impl CONNECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONNECTW::DISCONNECTED => true,
            CONNECTW::CONNECTED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONNECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONNECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONNECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disconnect"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(CONNECTW::DISCONNECTED)
    }
    #[doc = "Connect"]
    #[inline]
    pub fn connected(self) -> &'a mut W {
        self.variant(CONNECTW::CONNECTED)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:4 - Pin number"]
    #[inline]
    pub fn pin(&self) -> PINR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PINR { bits }
    }
    #[doc = "Bit 5 - Port number"]
    #[inline]
    pub fn port(&self) -> PORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PORTR { bits }
    }
    #[doc = "Bit 31 - Connection"]
    #[inline]
    pub fn connect(&self) -> CONNECTR {
        CONNECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:4 - Pin number"]
    #[inline]
    pub fn pin(&mut self) -> _PINW {
        _PINW { w: self }
    }
    #[doc = "Bit 5 - Port number"]
    #[inline]
    pub fn port(&mut self) -> _PORTW {
        _PORTW { w: self }
    }
    #[doc = "Bit 31 - Connection"]
    #[inline]
    pub fn connect(&mut self) -> _CONNECTW {
        _CONNECTW { w: self }
    }
}
