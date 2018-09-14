#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTOGGLE {
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
pub struct EPR {
    bits: u8,
}
impl EPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOR {
    #[doc = "Selects OUT endpoint"]
    OUT,
    #[doc = "Selects IN endpoint"]
    IN,
}
impl IOR {
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
            IOR::OUT => false,
            IOR::IN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOR {
        match value {
            false => IOR::OUT,
            true => IOR::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IOR::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IOR::IN
    }
}
#[doc = "Possible values of the field `VALUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALUER {
    #[doc = "No action on data toggle when writing the register with this value"]
    NOP,
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    DATA0,
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    DATA1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VALUER::NOP => 0,
            VALUER::DATA0 => 1,
            VALUER::DATA1 => 2,
            VALUER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VALUER {
        match value {
            0 => VALUER::NOP,
            1 => VALUER::DATA0,
            2 => VALUER::DATA1,
            i => VALUER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == VALUER::NOP
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == VALUER::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == VALUER::DATA1
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
#[doc = "Values that can be written to the field `VALUE`"]
pub enum VALUEW {
    #[doc = "No action on data toggle when writing the register with this value"]
    NOP,
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    DATA0,
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    DATA1,
}
impl VALUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VALUEW::NOP => 0,
            VALUEW::DATA0 => 1,
            VALUEW::DATA1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _VALUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALUEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No action on data toggle when writing the register with this value"]
    #[inline]
    pub fn nop(self) -> &'a mut W {
        self.variant(VALUEW::NOP)
    }
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    #[inline]
    pub fn data0(self) -> &'a mut W {
        self.variant(VALUEW::DATA0)
    }
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    #[inline]
    pub fn data1(self) -> &'a mut W {
        self.variant(VALUEW::DATA1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:2 - Select bulk endpoint number"]
    #[inline]
    pub fn ep(&self) -> EPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPR { bits }
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline]
    pub fn io(&self) -> IOR {
        IOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Data toggle value"]
    #[inline]
    pub fn value(&self) -> VALUER {
        VALUER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Select bulk endpoint number"]
    #[inline]
    pub fn ep(&mut self) -> _EPW {
        _EPW { w: self }
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline]
    pub fn io(&mut self) -> _IOW {
        _IOW { w: self }
    }
    #[doc = "Bits 8:9 - Data toggle value"]
    #[inline]
    pub fn value(&mut self) -> _VALUEW {
        _VALUEW { w: self }
    }
}
