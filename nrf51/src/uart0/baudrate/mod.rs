#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BAUDRATE {
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
#[doc = "Possible values of the field `BAUDRATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAUDRATER {
    #[doc = "1200 baud."]
    BAUD1200,
    #[doc = "2400 baud."]
    BAUD2400,
    #[doc = "4800 baud."]
    BAUD4800,
    #[doc = "9600 baud."]
    BAUD9600,
    #[doc = "14400 baud."]
    BAUD14400,
    #[doc = "19200 baud."]
    BAUD19200,
    #[doc = "28800 baud."]
    BAUD28800,
    #[doc = "31250 baud."]
    BAUD31250,
    #[doc = "38400 baud."]
    BAUD38400,
    #[doc = "56000 baud."]
    BAUD56000,
    #[doc = "57600 baud."]
    BAUD57600,
    #[doc = "76800 baud."]
    BAUD76800,
    #[doc = "115200 baud."]
    BAUD115200,
    #[doc = "230400 baud."]
    BAUD230400,
    #[doc = "250000 baud."]
    BAUD250000,
    #[doc = "460800 baud."]
    BAUD460800,
    #[doc = "921600 baud."]
    BAUD921600,
    #[doc = "1M baud."]
    BAUD1M,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl BAUDRATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            BAUDRATER::BAUD1200 => 323584,
            BAUDRATER::BAUD2400 => 643072,
            BAUDRATER::BAUD4800 => 1290240,
            BAUDRATER::BAUD9600 => 2576384,
            BAUDRATER::BAUD14400 => 3866624,
            BAUDRATER::BAUD19200 => 5152768,
            BAUDRATER::BAUD28800 => 7729152,
            BAUDRATER::BAUD31250 => 8388608,
            BAUDRATER::BAUD38400 => 10309632,
            BAUDRATER::BAUD56000 => 15007744,
            BAUDRATER::BAUD57600 => 15462400,
            BAUDRATER::BAUD76800 => 20615168,
            BAUDRATER::BAUD115200 => 30924800,
            BAUDRATER::BAUD230400 => 61845504,
            BAUDRATER::BAUD250000 => 67108864,
            BAUDRATER::BAUD460800 => 123695104,
            BAUDRATER::BAUD921600 => 247386112,
            BAUDRATER::BAUD1M => 268435456,
            BAUDRATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> BAUDRATER {
        match value {
            323584 => BAUDRATER::BAUD1200,
            643072 => BAUDRATER::BAUD2400,
            1290240 => BAUDRATER::BAUD4800,
            2576384 => BAUDRATER::BAUD9600,
            3866624 => BAUDRATER::BAUD14400,
            5152768 => BAUDRATER::BAUD19200,
            7729152 => BAUDRATER::BAUD28800,
            8388608 => BAUDRATER::BAUD31250,
            10309632 => BAUDRATER::BAUD38400,
            15007744 => BAUDRATER::BAUD56000,
            15462400 => BAUDRATER::BAUD57600,
            20615168 => BAUDRATER::BAUD76800,
            30924800 => BAUDRATER::BAUD115200,
            61845504 => BAUDRATER::BAUD230400,
            67108864 => BAUDRATER::BAUD250000,
            123695104 => BAUDRATER::BAUD460800,
            247386112 => BAUDRATER::BAUD921600,
            268435456 => BAUDRATER::BAUD1M,
            i => BAUDRATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BAUD1200`"]
    #[inline]
    pub fn is_baud1200(&self) -> bool {
        *self == BAUDRATER::BAUD1200
    }
    #[doc = "Checks if the value of the field is `BAUD2400`"]
    #[inline]
    pub fn is_baud2400(&self) -> bool {
        *self == BAUDRATER::BAUD2400
    }
    #[doc = "Checks if the value of the field is `BAUD4800`"]
    #[inline]
    pub fn is_baud4800(&self) -> bool {
        *self == BAUDRATER::BAUD4800
    }
    #[doc = "Checks if the value of the field is `BAUD9600`"]
    #[inline]
    pub fn is_baud9600(&self) -> bool {
        *self == BAUDRATER::BAUD9600
    }
    #[doc = "Checks if the value of the field is `BAUD14400`"]
    #[inline]
    pub fn is_baud14400(&self) -> bool {
        *self == BAUDRATER::BAUD14400
    }
    #[doc = "Checks if the value of the field is `BAUD19200`"]
    #[inline]
    pub fn is_baud19200(&self) -> bool {
        *self == BAUDRATER::BAUD19200
    }
    #[doc = "Checks if the value of the field is `BAUD28800`"]
    #[inline]
    pub fn is_baud28800(&self) -> bool {
        *self == BAUDRATER::BAUD28800
    }
    #[doc = "Checks if the value of the field is `BAUD31250`"]
    #[inline]
    pub fn is_baud31250(&self) -> bool {
        *self == BAUDRATER::BAUD31250
    }
    #[doc = "Checks if the value of the field is `BAUD38400`"]
    #[inline]
    pub fn is_baud38400(&self) -> bool {
        *self == BAUDRATER::BAUD38400
    }
    #[doc = "Checks if the value of the field is `BAUD56000`"]
    #[inline]
    pub fn is_baud56000(&self) -> bool {
        *self == BAUDRATER::BAUD56000
    }
    #[doc = "Checks if the value of the field is `BAUD57600`"]
    #[inline]
    pub fn is_baud57600(&self) -> bool {
        *self == BAUDRATER::BAUD57600
    }
    #[doc = "Checks if the value of the field is `BAUD76800`"]
    #[inline]
    pub fn is_baud76800(&self) -> bool {
        *self == BAUDRATER::BAUD76800
    }
    #[doc = "Checks if the value of the field is `BAUD115200`"]
    #[inline]
    pub fn is_baud115200(&self) -> bool {
        *self == BAUDRATER::BAUD115200
    }
    #[doc = "Checks if the value of the field is `BAUD230400`"]
    #[inline]
    pub fn is_baud230400(&self) -> bool {
        *self == BAUDRATER::BAUD230400
    }
    #[doc = "Checks if the value of the field is `BAUD250000`"]
    #[inline]
    pub fn is_baud250000(&self) -> bool {
        *self == BAUDRATER::BAUD250000
    }
    #[doc = "Checks if the value of the field is `BAUD460800`"]
    #[inline]
    pub fn is_baud460800(&self) -> bool {
        *self == BAUDRATER::BAUD460800
    }
    #[doc = "Checks if the value of the field is `BAUD921600`"]
    #[inline]
    pub fn is_baud921600(&self) -> bool {
        *self == BAUDRATER::BAUD921600
    }
    #[doc = "Checks if the value of the field is `BAUD1M`"]
    #[inline]
    pub fn is_baud1m(&self) -> bool {
        *self == BAUDRATER::BAUD1M
    }
}
#[doc = "Values that can be written to the field `BAUDRATE`"]
pub enum BAUDRATEW {
    #[doc = "1200 baud."]
    BAUD1200,
    #[doc = "2400 baud."]
    BAUD2400,
    #[doc = "4800 baud."]
    BAUD4800,
    #[doc = "9600 baud."]
    BAUD9600,
    #[doc = "14400 baud."]
    BAUD14400,
    #[doc = "19200 baud."]
    BAUD19200,
    #[doc = "28800 baud."]
    BAUD28800,
    #[doc = "31250 baud."]
    BAUD31250,
    #[doc = "38400 baud."]
    BAUD38400,
    #[doc = "56000 baud."]
    BAUD56000,
    #[doc = "57600 baud."]
    BAUD57600,
    #[doc = "76800 baud."]
    BAUD76800,
    #[doc = "115200 baud."]
    BAUD115200,
    #[doc = "230400 baud."]
    BAUD230400,
    #[doc = "250000 baud."]
    BAUD250000,
    #[doc = "460800 baud."]
    BAUD460800,
    #[doc = "921600 baud."]
    BAUD921600,
    #[doc = "1M baud."]
    BAUD1M,
}
impl BAUDRATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            BAUDRATEW::BAUD1200 => 323584,
            BAUDRATEW::BAUD2400 => 643072,
            BAUDRATEW::BAUD4800 => 1290240,
            BAUDRATEW::BAUD9600 => 2576384,
            BAUDRATEW::BAUD14400 => 3866624,
            BAUDRATEW::BAUD19200 => 5152768,
            BAUDRATEW::BAUD28800 => 7729152,
            BAUDRATEW::BAUD31250 => 8388608,
            BAUDRATEW::BAUD38400 => 10309632,
            BAUDRATEW::BAUD56000 => 15007744,
            BAUDRATEW::BAUD57600 => 15462400,
            BAUDRATEW::BAUD76800 => 20615168,
            BAUDRATEW::BAUD115200 => 30924800,
            BAUDRATEW::BAUD230400 => 61845504,
            BAUDRATEW::BAUD250000 => 67108864,
            BAUDRATEW::BAUD460800 => 123695104,
            BAUDRATEW::BAUD921600 => 247386112,
            BAUDRATEW::BAUD1M => 268435456,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BAUDRATEW<'a> {
    w: &'a mut W,
}
impl<'a> _BAUDRATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BAUDRATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1200 baud."]
    #[inline]
    pub fn baud1200(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD1200)
    }
    #[doc = "2400 baud."]
    #[inline]
    pub fn baud2400(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD2400)
    }
    #[doc = "4800 baud."]
    #[inline]
    pub fn baud4800(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD4800)
    }
    #[doc = "9600 baud."]
    #[inline]
    pub fn baud9600(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD9600)
    }
    #[doc = "14400 baud."]
    #[inline]
    pub fn baud14400(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD14400)
    }
    #[doc = "19200 baud."]
    #[inline]
    pub fn baud19200(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD19200)
    }
    #[doc = "28800 baud."]
    #[inline]
    pub fn baud28800(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD28800)
    }
    #[doc = "31250 baud."]
    #[inline]
    pub fn baud31250(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD31250)
    }
    #[doc = "38400 baud."]
    #[inline]
    pub fn baud38400(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD38400)
    }
    #[doc = "56000 baud."]
    #[inline]
    pub fn baud56000(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD56000)
    }
    #[doc = "57600 baud."]
    #[inline]
    pub fn baud57600(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD57600)
    }
    #[doc = "76800 baud."]
    #[inline]
    pub fn baud76800(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD76800)
    }
    #[doc = "115200 baud."]
    #[inline]
    pub fn baud115200(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD115200)
    }
    #[doc = "230400 baud."]
    #[inline]
    pub fn baud230400(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD230400)
    }
    #[doc = "250000 baud."]
    #[inline]
    pub fn baud250000(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD250000)
    }
    #[doc = "460800 baud."]
    #[inline]
    pub fn baud460800(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD460800)
    }
    #[doc = "921600 baud."]
    #[inline]
    pub fn baud921600(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD921600)
    }
    #[doc = "1M baud."]
    #[inline]
    pub fn baud1m(self) -> &'a mut W {
        self.variant(BAUDRATEW::BAUD1M)
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
    #[doc = "Bits 0:31 - UART baudrate."]
    #[inline]
    pub fn baudrate(&self) -> BAUDRATER {
        BAUDRATER::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - UART baudrate."]
    #[inline]
    pub fn baudrate(&mut self) -> _BAUDRATEW {
        _BAUDRATEW { w: self }
    }
}
