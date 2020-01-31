#[doc = "Reader of register RUNSTATUS"]
pub type R = crate::R<u32, super::RUNSTATUS>;
#[doc = "Indicates whether or not the watchdog is running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNSTATUS_A {
    #[doc = "0: Watchdog not running"]
    NOTRUNNING = 0,
    #[doc = "1: Watchdog is running"]
    RUNNING = 1,
}
impl From<RUNSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RUNSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RUNSTATUS`"]
pub type RUNSTATUS_R = crate::R<bool, RUNSTATUS_A>;
impl RUNSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUNSTATUS_A {
        match self.bits {
            false => RUNSTATUS_A::NOTRUNNING,
            true => RUNSTATUS_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == RUNSTATUS_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUNSTATUS_A::RUNNING
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether or not the watchdog is running"]
    #[inline(always)]
    pub fn runstatus(&self) -> RUNSTATUS_R {
        RUNSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
