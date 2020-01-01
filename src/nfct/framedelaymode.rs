#[doc = "Reader of register FRAMEDELAYMODE"]
pub type R = crate::R<u32, super::FRAMEDELAYMODE>;
#[doc = "Writer for register FRAMEDELAYMODE"]
pub type W = crate::W<u32, super::FRAMEDELAYMODE>;
#[doc = "Register FRAMEDELAYMODE `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FRAMEDELAYMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Configuration register for the Frame Delay Timer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRAMEDELAYMODE_A {
    #[doc = "0: Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    FREERUN = 0,
    #[doc = "1: Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOW = 1,
    #[doc = "2: Frame is transmitted exactly at FRAMEDELAYMAX"]
    EXACTVAL = 2,
    #[doc = "3: Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOWGRID = 3,
}
impl From<FRAMEDELAYMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAMEDELAYMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRAMEDELAYMODE`"]
pub type FRAMEDELAYMODE_R = crate::R<u8, FRAMEDELAYMODE_A>;
impl FRAMEDELAYMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMEDELAYMODE_A {
        match self.bits {
            0 => FRAMEDELAYMODE_A::FREERUN,
            1 => FRAMEDELAYMODE_A::WINDOW,
            2 => FRAMEDELAYMODE_A::EXACTVAL,
            3 => FRAMEDELAYMODE_A::WINDOWGRID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREERUN`"]
    #[inline(always)]
    pub fn is_free_run(&self) -> bool {
        *self == FRAMEDELAYMODE_A::FREERUN
    }
    #[doc = "Checks if the value of the field is `WINDOW`"]
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        *self == FRAMEDELAYMODE_A::WINDOW
    }
    #[doc = "Checks if the value of the field is `EXACTVAL`"]
    #[inline(always)]
    pub fn is_exact_val(&self) -> bool {
        *self == FRAMEDELAYMODE_A::EXACTVAL
    }
    #[doc = "Checks if the value of the field is `WINDOWGRID`"]
    #[inline(always)]
    pub fn is_window_grid(&self) -> bool {
        *self == FRAMEDELAYMODE_A::WINDOWGRID
    }
}
#[doc = "Write proxy for field `FRAMEDELAYMODE`"]
pub struct FRAMEDELAYMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDELAYMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAMEDELAYMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    #[inline(always)]
    pub fn free_run(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODE_A::FREERUN)
    }
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn window(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODE_A::WINDOW)
    }
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn exact_val(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODE_A::EXACTVAL)
    }
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn window_grid(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODE_A::WINDOWGRID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub fn framedelaymode(&self) -> FRAMEDELAYMODE_R {
        FRAMEDELAYMODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub fn framedelaymode(&mut self) -> FRAMEDELAYMODE_W {
        FRAMEDELAYMODE_W { w: self }
    }
}
