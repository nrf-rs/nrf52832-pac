#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LFCLKSTAT {
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
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "LFCLK not running"]
    NOTRUNNING,
    #[doc = "LFCLK running"]
    RUNNING,
}
impl STATER {
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
            STATER::NOTRUNNING => false,
            STATER::RUNNING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATER {
        match value {
            false => STATER::NOTRUNNING,
            true => STATER::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline]
    pub fn is_not_running(&self) -> bool {
        *self == STATER::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline]
    pub fn is_running(&self) -> bool {
        *self == STATER::RUNNING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Source of LFCLK"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - LFCLK state"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
