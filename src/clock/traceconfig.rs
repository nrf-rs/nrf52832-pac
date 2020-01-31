#[doc = "Reader of register TRACECONFIG"]
pub type R = crate::R<u32, super::TRACECONFIG>;
#[doc = "Writer for register TRACECONFIG"]
pub type W = crate::W<u32, super::TRACECONFIG>;
#[doc = "Register TRACECONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::TRACECONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACEPORTSPEED_A {
    #[doc = "0: 32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    _32MHZ = 0,
    #[doc = "1: 16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    _16MHZ = 1,
    #[doc = "2: 8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    _8MHZ = 2,
    #[doc = "3: 4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    _4MHZ = 3,
}
impl From<TRACEPORTSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEPORTSPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRACEPORTSPEED`"]
pub type TRACEPORTSPEED_R = crate::R<u8, TRACEPORTSPEED_A>;
impl TRACEPORTSPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACEPORTSPEED_A {
        match self.bits {
            0 => TRACEPORTSPEED_A::_32MHZ,
            1 => TRACEPORTSPEED_A::_16MHZ,
            2 => TRACEPORTSPEED_A::_8MHZ,
            3 => TRACEPORTSPEED_A::_4MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_32MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_8MHZ
    }
    #[doc = "Checks if the value of the field is `_4MHZ`"]
    #[inline(always)]
    pub fn is_4mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_4MHZ
    }
}
#[doc = "Write proxy for field `TRACEPORTSPEED`"]
pub struct TRACEPORTSPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEPORTSPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEPORTSPEED_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_32MHZ)
    }
    #[doc = "16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_16MHZ)
    }
    #[doc = "8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_8MHZ)
    }
    #[doc = "4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    #[inline(always)]
    pub fn _4mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_4MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Pin multiplexing of trace signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACEMUX_A {
    #[doc = "0: GPIOs multiplexed onto all trace-pins"]
    GPIO = 0,
    #[doc = "1: SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins"]
    SERIAL = 1,
    #[doc = "2: TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14."]
    PARALLEL = 2,
}
impl From<TRACEMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRACEMUX`"]
pub type TRACEMUX_R = crate::R<u8, TRACEMUX_A>;
impl TRACEMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRACEMUX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRACEMUX_A::GPIO),
            1 => Val(TRACEMUX_A::SERIAL),
            2 => Val(TRACEMUX_A::PARALLEL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TRACEMUX_A::GPIO
    }
    #[doc = "Checks if the value of the field is `SERIAL`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == TRACEMUX_A::SERIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        *self == TRACEMUX_A::PARALLEL
    }
}
#[doc = "Write proxy for field `TRACEMUX`"]
pub struct TRACEMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEMUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIOs multiplexed onto all trace-pins"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TRACEMUX_A::GPIO)
    }
    #[doc = "SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut W {
        self.variant(TRACEMUX_A::SERIAL)
    }
    #[doc = "TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14."]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(TRACEMUX_A::PARALLEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TRACEPORTSPEED_R {
        TRACEPORTSPEED_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals."]
    #[inline(always)]
    pub fn tracemux(&self) -> TRACEMUX_R {
        TRACEMUX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&mut self) -> TRACEPORTSPEED_W {
        TRACEPORTSPEED_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals."]
    #[inline(always)]
    pub fn tracemux(&mut self) -> TRACEMUX_W {
        TRACEMUX_W { w: self }
    }
}
