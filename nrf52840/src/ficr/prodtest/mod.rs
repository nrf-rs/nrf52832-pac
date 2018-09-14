#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRODTEST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PRODTEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRODTESTR {
    #[doc = "Production tests done"]
    DONE,
    #[doc = "Production tests not done"]
    NOTDONE,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PRODTESTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PRODTESTR::DONE => 3141677471,
            PRODTESTR::NOTDONE => 4294967295,
            PRODTESTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PRODTESTR {
        match value {
            3141677471 => PRODTESTR::DONE,
            4294967295 => PRODTESTR::NOTDONE,
            i => PRODTESTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline]
    pub fn is_done(&self) -> bool {
        *self == PRODTESTR::DONE
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline]
    pub fn is_not_done(&self) -> bool {
        *self == PRODTESTR::NOTDONE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Production test signature n"]
    #[inline]
    pub fn prodtest(&self) -> PRODTESTR {
        PRODTESTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
