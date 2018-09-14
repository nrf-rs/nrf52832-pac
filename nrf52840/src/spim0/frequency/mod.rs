#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FREQUENCY {
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
#[doc = "Possible values of the field `FREQUENCY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQUENCYR {
    #[doc = "125 kbps"]
    K125,
    #[doc = "250 kbps"]
    K250,
    #[doc = "500 kbps"]
    K500,
    #[doc = "1 Mbps"]
    M1,
    #[doc = "2 Mbps"]
    M2,
    #[doc = "4 Mbps"]
    M4,
    #[doc = "8 Mbps"]
    M8,
    #[doc = "16 Mbps"]
    M16,
    #[doc = "32 Mbps"]
    M32,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl FREQUENCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            FREQUENCYR::K125 => 33554432,
            FREQUENCYR::K250 => 67108864,
            FREQUENCYR::K500 => 134217728,
            FREQUENCYR::M1 => 268435456,
            FREQUENCYR::M2 => 536870912,
            FREQUENCYR::M4 => 1073741824,
            FREQUENCYR::M8 => 2147483648,
            FREQUENCYR::M16 => 167772160,
            FREQUENCYR::M32 => 335544320,
            FREQUENCYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> FREQUENCYR {
        match value {
            33554432 => FREQUENCYR::K125,
            67108864 => FREQUENCYR::K250,
            134217728 => FREQUENCYR::K500,
            268435456 => FREQUENCYR::M1,
            536870912 => FREQUENCYR::M2,
            1073741824 => FREQUENCYR::M4,
            2147483648 => FREQUENCYR::M8,
            167772160 => FREQUENCYR::M16,
            335544320 => FREQUENCYR::M32,
            i => FREQUENCYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `K125`"]
    #[inline]
    pub fn is_k125(&self) -> bool {
        *self == FREQUENCYR::K125
    }
    #[doc = "Checks if the value of the field is `K250`"]
    #[inline]
    pub fn is_k250(&self) -> bool {
        *self == FREQUENCYR::K250
    }
    #[doc = "Checks if the value of the field is `K500`"]
    #[inline]
    pub fn is_k500(&self) -> bool {
        *self == FREQUENCYR::K500
    }
    #[doc = "Checks if the value of the field is `M1`"]
    #[inline]
    pub fn is_m1(&self) -> bool {
        *self == FREQUENCYR::M1
    }
    #[doc = "Checks if the value of the field is `M2`"]
    #[inline]
    pub fn is_m2(&self) -> bool {
        *self == FREQUENCYR::M2
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline]
    pub fn is_m4(&self) -> bool {
        *self == FREQUENCYR::M4
    }
    #[doc = "Checks if the value of the field is `M8`"]
    #[inline]
    pub fn is_m8(&self) -> bool {
        *self == FREQUENCYR::M8
    }
    #[doc = "Checks if the value of the field is `M16`"]
    #[inline]
    pub fn is_m16(&self) -> bool {
        *self == FREQUENCYR::M16
    }
    #[doc = "Checks if the value of the field is `M32`"]
    #[inline]
    pub fn is_m32(&self) -> bool {
        *self == FREQUENCYR::M32
    }
}
#[doc = "Values that can be written to the field `FREQUENCY`"]
pub enum FREQUENCYW {
    #[doc = "125 kbps"]
    K125,
    #[doc = "250 kbps"]
    K250,
    #[doc = "500 kbps"]
    K500,
    #[doc = "1 Mbps"]
    M1,
    #[doc = "2 Mbps"]
    M2,
    #[doc = "4 Mbps"]
    M4,
    #[doc = "8 Mbps"]
    M8,
    #[doc = "16 Mbps"]
    M16,
    #[doc = "32 Mbps"]
    M32,
}
impl FREQUENCYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            FREQUENCYW::K125 => 33554432,
            FREQUENCYW::K250 => 67108864,
            FREQUENCYW::K500 => 134217728,
            FREQUENCYW::M1 => 268435456,
            FREQUENCYW::M2 => 536870912,
            FREQUENCYW::M4 => 1073741824,
            FREQUENCYW::M8 => 2147483648,
            FREQUENCYW::M16 => 167772160,
            FREQUENCYW::M32 => 335544320,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQUENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQUENCYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQUENCYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "125 kbps"]
    #[inline]
    pub fn k125(self) -> &'a mut W {
        self.variant(FREQUENCYW::K125)
    }
    #[doc = "250 kbps"]
    #[inline]
    pub fn k250(self) -> &'a mut W {
        self.variant(FREQUENCYW::K250)
    }
    #[doc = "500 kbps"]
    #[inline]
    pub fn k500(self) -> &'a mut W {
        self.variant(FREQUENCYW::K500)
    }
    #[doc = "1 Mbps"]
    #[inline]
    pub fn m1(self) -> &'a mut W {
        self.variant(FREQUENCYW::M1)
    }
    #[doc = "2 Mbps"]
    #[inline]
    pub fn m2(self) -> &'a mut W {
        self.variant(FREQUENCYW::M2)
    }
    #[doc = "4 Mbps"]
    #[inline]
    pub fn m4(self) -> &'a mut W {
        self.variant(FREQUENCYW::M4)
    }
    #[doc = "8 Mbps"]
    #[inline]
    pub fn m8(self) -> &'a mut W {
        self.variant(FREQUENCYW::M8)
    }
    #[doc = "16 Mbps"]
    #[inline]
    pub fn m16(self) -> &'a mut W {
        self.variant(FREQUENCYW::M16)
    }
    #[doc = "32 Mbps"]
    #[inline]
    pub fn m32(self) -> &'a mut W {
        self.variant(FREQUENCYW::M32)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - SPI master data rate"]
    #[inline]
    pub fn frequency(&self) -> FREQUENCYR {
        FREQUENCYR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 67108864 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - SPI master data rate"]
    #[inline]
    pub fn frequency(&mut self) -> _FREQUENCYW {
        _FREQUENCYW { w: self }
    }
}
