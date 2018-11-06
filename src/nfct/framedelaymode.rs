#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRAMEDELAYMODE {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `FRAMEDELAYMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMEDELAYMODER {
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    FREERUN,
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOW,
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    EXACTVAL,
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOWGRID,
}
impl FRAMEDELAYMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRAMEDELAYMODER::FREERUN => 0,
            FRAMEDELAYMODER::WINDOW => 1,
            FRAMEDELAYMODER::EXACTVAL => 2,
            FRAMEDELAYMODER::WINDOWGRID => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRAMEDELAYMODER {
        match value {
            0 => FRAMEDELAYMODER::FREERUN,
            1 => FRAMEDELAYMODER::WINDOW,
            2 => FRAMEDELAYMODER::EXACTVAL,
            3 => FRAMEDELAYMODER::WINDOWGRID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREERUN`"]
    #[inline]
    pub fn is_free_run(&self) -> bool {
        *self == FRAMEDELAYMODER::FREERUN
    }
    #[doc = "Checks if the value of the field is `WINDOW`"]
    #[inline]
    pub fn is_window(&self) -> bool {
        *self == FRAMEDELAYMODER::WINDOW
    }
    #[doc = "Checks if the value of the field is `EXACTVAL`"]
    #[inline]
    pub fn is_exact_val(&self) -> bool {
        *self == FRAMEDELAYMODER::EXACTVAL
    }
    #[doc = "Checks if the value of the field is `WINDOWGRID`"]
    #[inline]
    pub fn is_window_grid(&self) -> bool {
        *self == FRAMEDELAYMODER::WINDOWGRID
    }
}
#[doc = "Values that can be written to the field `FRAMEDELAYMODE`"]
pub enum FRAMEDELAYMODEW {
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    FREERUN,
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOW,
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    EXACTVAL,
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOWGRID,
}
impl FRAMEDELAYMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRAMEDELAYMODEW::FREERUN => 0,
            FRAMEDELAYMODEW::WINDOW => 1,
            FRAMEDELAYMODEW::EXACTVAL => 2,
            FRAMEDELAYMODEW::WINDOWGRID => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAMEDELAYMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMEDELAYMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAMEDELAYMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    #[inline]
    pub fn free_run(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODEW::FREERUN)
    }
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline]
    pub fn window(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODEW::WINDOW)
    }
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    #[inline]
    pub fn exact_val(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODEW::EXACTVAL)
    }
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline]
    pub fn window_grid(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODEW::WINDOWGRID)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Configuration register for the Frame Delay Timer"]
    #[inline]
    pub fn framedelaymode(&self) -> FRAMEDELAYMODER {
        FRAMEDELAYMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Configuration register for the Frame Delay Timer"]
    #[inline]
    pub fn framedelaymode(&mut self) -> _FRAMEDELAYMODEW {
        _FRAMEDELAYMODEW { w: self }
    }
}
