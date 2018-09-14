#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LFCLKSRCCOPY {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "32.768 kHz RC oscillator"]
    RC,
    #[doc = "32.768 kHz crystal oscillator"]
    XTAL,
    #[doc = "32.768 kHz synthesized from HFCLK"]
    SYNTH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::RC => 0,
            SRCR::XTAL => 1,
            SRCR::SYNTH => 2,
            SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            0 => SRCR::RC,
            1 => SRCR::XTAL,
            2 => SRCR::SYNTH,
            i => SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline]
    pub fn is_rc(&self) -> bool {
        *self == SRCR::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == SRCR::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline]
    pub fn is_synth(&self) -> bool {
        *self == SRCR::SYNTH
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Clock source"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
