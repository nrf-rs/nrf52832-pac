#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SELRES {
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
pub struct RFU10R {
    bits: u8,
}
impl RFU10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CASCADE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASCADER {
    #[doc = "NFCID1 complete"]
    COMPLETE,
    #[doc = "NFCID1 not complete"]
    NOTCOMPLETE,
}
impl CASCADER {
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
            CASCADER::COMPLETE => false,
            CASCADER::NOTCOMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CASCADER {
        match value {
            false => CASCADER::COMPLETE,
            true => CASCADER::NOTCOMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == CASCADER::COMPLETE
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == CASCADER::NOTCOMPLETE
    }
}
#[doc = r" Value of the field"]
pub struct RFU43R {
    bits: u8,
}
impl RFU43R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PROTOCOLR {
    bits: u8,
}
impl PROTOCOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFU7R {
    bits: bool,
}
impl RFU7R {
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
#[doc = r" Proxy"]
pub struct _RFU10W<'a> {
    w: &'a mut W,
}
impl<'a> _RFU10W<'a> {
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
#[doc = "Values that can be written to the field `CASCADE`"]
pub enum CASCADEW {
    #[doc = "NFCID1 complete"]
    COMPLETE,
    #[doc = "NFCID1 not complete"]
    NOTCOMPLETE,
}
impl CASCADEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CASCADEW::COMPLETE => false,
            CASCADEW::NOTCOMPLETE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CASCADEW<'a> {
    w: &'a mut W,
}
impl<'a> _CASCADEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CASCADEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NFCID1 complete"]
    #[inline]
    pub fn complete(self) -> &'a mut W {
        self.variant(CASCADEW::COMPLETE)
    }
    #[doc = "NFCID1 not complete"]
    #[inline]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(CASCADEW::NOTCOMPLETE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFU43W<'a> {
    w: &'a mut W,
}
impl<'a> _RFU43W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PROTOCOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTOCOLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFU7W<'a> {
    w: &'a mut W,
}
impl<'a> _RFU7W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu10(&self) -> RFU10R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFU10R { bits }
    }
    #[doc = "Bit 2 - Cascade bit (controlled by hardware, write has no effect)"]
    #[inline]
    pub fn cascade(&self) -> CASCADER {
        CASCADER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu43(&self) -> RFU43R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFU43R { bits }
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline]
    pub fn protocol(&self) -> PROTOCOLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROTOCOLR { bits }
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu7(&self) -> RFU7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFU7R { bits }
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
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu10(&mut self) -> _RFU10W {
        _RFU10W { w: self }
    }
    #[doc = "Bit 2 - Cascade bit (controlled by hardware, write has no effect)"]
    #[inline]
    pub fn cascade(&mut self) -> _CASCADEW {
        _CASCADEW { w: self }
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu43(&mut self) -> _RFU43W {
        _RFU43W { w: self }
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline]
    pub fn protocol(&mut self) -> _PROTOCOLW {
        _PROTOCOLW { w: self }
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline]
    pub fn rfu7(&mut self) -> _RFU7W {
        _RFU7W { w: self }
    }
}
