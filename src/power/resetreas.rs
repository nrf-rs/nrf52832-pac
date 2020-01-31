#[doc = "Reader of register RESETREAS"]
pub type R = crate::R<u32, super::RESETREAS>;
#[doc = "Writer for register RESETREAS"]
pub type W = crate::W<u32, super::RESETREAS>;
#[doc = "Register RESETREAS `reset()`'s with value 0"]
impl crate::ResetValue for super::RESETREAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset from pin-reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETPIN_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<RESETPIN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETPIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESETPIN`"]
pub type RESETPIN_R = crate::R<bool, RESETPIN_A>;
impl RESETPIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETPIN_A {
        match self.bits {
            false => RESETPIN_A::NOTDETECTED,
            true => RESETPIN_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == RESETPIN_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == RESETPIN_A::DETECTED
    }
}
#[doc = "Write proxy for field `RESETPIN`"]
pub struct RESETPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETPIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETPIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reset from watchdog detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOG_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DOG_A> for bool {
    #[inline(always)]
    fn from(variant: DOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOG`"]
pub type DOG_R = crate::R<bool, DOG_A>;
impl DOG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOG_A {
        match self.bits {
            false => DOG_A::NOTDETECTED,
            true => DOG_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DOG_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DOG_A::DETECTED
    }
}
#[doc = "Write proxy for field `DOG`"]
pub struct DOG_W<'a> {
    w: &'a mut W,
}
impl<'a> DOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOG_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOG_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reset from soft reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SREQ_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<SREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SREQ`"]
pub type SREQ_R = crate::R<bool, SREQ_A>;
impl SREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREQ_A {
        match self.bits {
            false => SREQ_A::NOTDETECTED,
            true => SREQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == SREQ_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SREQ_A::DETECTED
    }
}
#[doc = "Write proxy for field `SREQ`"]
pub struct SREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SREQ_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SREQ_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reset from CPU lock-up detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKUP`"]
pub type LOCKUP_R = crate::R<bool, LOCKUP_A>;
impl LOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::NOTDETECTED,
            true => LOCKUP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LOCKUP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LOCKUP_A::DETECTED
    }
}
#[doc = "Write proxy for field `LOCKUP`"]
pub struct LOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFF_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<OFF_A> for bool {
    #[inline(always)]
    fn from(variant: OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFF`"]
pub type OFF_R = crate::R<bool, OFF_A>;
impl OFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFF_A {
        match self.bits {
            false => OFF_A::NOTDETECTED,
            true => OFF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == OFF_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == OFF_A::DETECTED
    }
}
#[doc = "Write proxy for field `OFF`"]
pub struct OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OFF_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OFF_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCOMP_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<LPCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: LPCOMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPCOMP`"]
pub type LPCOMP_R = crate::R<bool, LPCOMP_A>;
impl LPCOMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCOMP_A {
        match self.bits {
            false => LPCOMP_A::NOTDETECTED,
            true => LPCOMP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LPCOMP_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LPCOMP_A::DETECTED
    }
}
#[doc = "Write proxy for field `LPCOMP`"]
pub struct LPCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCOMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCOMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIF_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DIF_A> for bool {
    #[inline(always)]
    fn from(variant: DIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIF`"]
pub type DIF_R = crate::R<bool, DIF_A>;
impl DIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIF_A {
        match self.bits {
            false => DIF_A::NOTDETECTED,
            true => DIF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DIF_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DIF_A::DETECTED
    }
}
#[doc = "Write proxy for field `DIF`"]
pub struct DIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DIF_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DIF_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reset due to wake up from System OFF mode by NFC field detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFC_A {
    #[doc = "0: Not detected"]
    NOTDETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<NFC_A> for bool {
    #[inline(always)]
    fn from(variant: NFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NFC`"]
pub type NFC_R = crate::R<bool, NFC_A>;
impl NFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFC_A {
        match self.bits {
            false => NFC_A::NOTDETECTED,
            true => NFC_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == NFC_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == NFC_A::DETECTED
    }
}
#[doc = "Write proxy for field `NFC`"]
pub struct NFC_W<'a> {
    w: &'a mut W,
}
impl<'a> NFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(NFC_A::NOTDETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(NFC_A::DETECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reset from pin-reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> RESETPIN_R {
        RESETPIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset from watchdog detected"]
    #[inline(always)]
    pub fn dog(&self) -> DOG_R {
        DOG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset from soft reset detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SREQ_R {
        SREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset from CPU lock-up detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Reset due to wake up from System OFF mode by NFC field detect"]
    #[inline(always)]
    pub fn nfc(&self) -> NFC_R {
        NFC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin-reset detected"]
    #[inline(always)]
    pub fn resetpin(&mut self) -> RESETPIN_W {
        RESETPIN_W { w: self }
    }
    #[doc = "Bit 1 - Reset from watchdog detected"]
    #[inline(always)]
    pub fn dog(&mut self) -> DOG_W {
        DOG_W { w: self }
    }
    #[doc = "Bit 2 - Reset from soft reset detected"]
    #[inline(always)]
    pub fn sreq(&mut self) -> SREQ_W {
        SREQ_W { w: self }
    }
    #[doc = "Bit 3 - Reset from CPU lock-up detected"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LOCKUP_W {
        LOCKUP_W { w: self }
    }
    #[doc = "Bit 16 - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W {
        OFF_W { w: self }
    }
    #[doc = "Bit 17 - Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&mut self) -> LPCOMP_W {
        LPCOMP_W { w: self }
    }
    #[doc = "Bit 18 - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
    #[inline(always)]
    pub fn dif(&mut self) -> DIF_W {
        DIF_W { w: self }
    }
    #[doc = "Bit 19 - Reset due to wake up from System OFF mode by NFC field detect"]
    #[inline(always)]
    pub fn nfc(&mut self) -> NFC_W {
        NFC_W { w: self }
    }
}
