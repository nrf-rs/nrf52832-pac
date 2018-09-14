#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::NFCTAGSTATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `NFCTAGSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFCTAGSTATER {
    #[doc = "Disabled or sense"]
    DISABLED,
    #[doc = "RampUp"]
    RAMPUP,
    #[doc = "Idle"]
    IDLE,
    #[doc = "Receive"]
    RECEIVE,
    #[doc = "FrameDelay"]
    FRAMEDELAY,
    #[doc = "Transmit"]
    TRANSMIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NFCTAGSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NFCTAGSTATER::DISABLED => 0,
            NFCTAGSTATER::RAMPUP => 2,
            NFCTAGSTATER::IDLE => 3,
            NFCTAGSTATER::RECEIVE => 4,
            NFCTAGSTATER::FRAMEDELAY => 5,
            NFCTAGSTATER::TRANSMIT => 6,
            NFCTAGSTATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NFCTAGSTATER {
        match value {
            0 => NFCTAGSTATER::DISABLED,
            2 => NFCTAGSTATER::RAMPUP,
            3 => NFCTAGSTATER::IDLE,
            4 => NFCTAGSTATER::RECEIVE,
            5 => NFCTAGSTATER::FRAMEDELAY,
            6 => NFCTAGSTATER::TRANSMIT,
            i => NFCTAGSTATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NFCTAGSTATER::DISABLED
    }
    #[doc = "Checks if the value of the field is `RAMPUP`"]
    #[inline]
    pub fn is_ramp_up(&self) -> bool {
        *self == NFCTAGSTATER::RAMPUP
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == NFCTAGSTATER::IDLE
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline]
    pub fn is_receive(&self) -> bool {
        *self == NFCTAGSTATER::RECEIVE
    }
    #[doc = "Checks if the value of the field is `FRAMEDELAY`"]
    #[inline]
    pub fn is_frame_delay(&self) -> bool {
        *self == NFCTAGSTATER::FRAMEDELAY
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline]
    pub fn is_transmit(&self) -> bool {
        *self == NFCTAGSTATER::TRANSMIT
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - NfcTag state"]
    #[inline]
    pub fn nfctagstate(&self) -> NFCTAGSTATER {
        NFCTAGSTATER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
