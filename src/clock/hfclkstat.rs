#[doc = "Reader of register HFCLKSTAT"]
pub type R = crate::R<u32, super::HFCLKSTAT>;
#[doc = "Source of HFCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: 64 MHz internal oscillator (HFINT)"]
    RC = 0,
    #[doc = "1: 64 MHz crystal oscillator (HFXO)"]
    XTAL = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::RC,
            true => SRC_A::XTAL,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == SRC_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == SRC_A::XTAL
    }
}
#[doc = "HFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: HFCLK not running"]
    NOTRUNNING = 0,
    #[doc = "1: HFCLK running"]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<bool, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::NOTRUNNING,
            true => STATE_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == STATE_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STATE_A::RUNNING
    }
}
impl R {
    #[doc = "Bit 0 - Source of HFCLK"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - HFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
