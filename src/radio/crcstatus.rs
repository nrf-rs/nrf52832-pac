#[doc = "Reader of register CRCSTATUS"]
pub type R = crate::R<u32, super::CRCSTATUS>;
#[doc = "CRC status of packet received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCSTATUS_A {
    #[doc = "0: Packet received with CRC error"]
    CRCERROR = 0,
    #[doc = "1: Packet received with CRC ok"]
    CRCOK = 1,
}
impl From<CRCSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCSTATUS`"]
pub type CRCSTATUS_R = crate::R<bool, CRCSTATUS_A>;
impl CRCSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCSTATUS_A {
        match self.bits {
            false => CRCSTATUS_A::CRCERROR,
            true => CRCSTATUS_A::CRCOK,
        }
    }
    #[doc = "Checks if the value of the field is `CRCERROR`"]
    #[inline(always)]
    pub fn is_crcerror(&self) -> bool {
        *self == CRCSTATUS_A::CRCERROR
    }
    #[doc = "Checks if the value of the field is `CRCOK`"]
    #[inline(always)]
    pub fn is_crcok(&self) -> bool {
        *self == CRCSTATUS_A::CRCOK
    }
}
impl R {
    #[doc = "Bit 0 - CRC status of packet received"]
    #[inline(always)]
    pub fn crcstatus(&self) -> CRCSTATUS_R {
        CRCSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
