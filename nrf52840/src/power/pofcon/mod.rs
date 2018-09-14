#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POFCON {
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
#[doc = "Possible values of the field `POF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl POFR {
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
            POFR::DISABLED => false,
            POFR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POFR {
        match value {
            false => POFR::DISABLED,
            true => POFR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == POFR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == POFR::ENABLED
    }
}
#[doc = "Possible values of the field `THRESHOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRESHOLDR {
    #[doc = "Set threshold to 1.7 V"]
    V17,
    #[doc = "Set threshold to 1.8 V"]
    V18,
    #[doc = "Set threshold to 1.9 V"]
    V19,
    #[doc = "Set threshold to 2.0 V"]
    V20,
    #[doc = "Set threshold to 2.1 V"]
    V21,
    #[doc = "Set threshold to 2.2 V"]
    V22,
    #[doc = "Set threshold to 2.3 V"]
    V23,
    #[doc = "Set threshold to 2.4 V"]
    V24,
    #[doc = "Set threshold to 2.5 V"]
    V25,
    #[doc = "Set threshold to 2.6 V"]
    V26,
    #[doc = "Set threshold to 2.7 V"]
    V27,
    #[doc = "Set threshold to 2.8 V"]
    V28,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl THRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            THRESHOLDR::V17 => 4,
            THRESHOLDR::V18 => 5,
            THRESHOLDR::V19 => 6,
            THRESHOLDR::V20 => 7,
            THRESHOLDR::V21 => 8,
            THRESHOLDR::V22 => 9,
            THRESHOLDR::V23 => 10,
            THRESHOLDR::V24 => 11,
            THRESHOLDR::V25 => 12,
            THRESHOLDR::V26 => 13,
            THRESHOLDR::V27 => 14,
            THRESHOLDR::V28 => 15,
            THRESHOLDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> THRESHOLDR {
        match value {
            4 => THRESHOLDR::V17,
            5 => THRESHOLDR::V18,
            6 => THRESHOLDR::V19,
            7 => THRESHOLDR::V20,
            8 => THRESHOLDR::V21,
            9 => THRESHOLDR::V22,
            10 => THRESHOLDR::V23,
            11 => THRESHOLDR::V24,
            12 => THRESHOLDR::V25,
            13 => THRESHOLDR::V26,
            14 => THRESHOLDR::V27,
            15 => THRESHOLDR::V28,
            i => THRESHOLDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `V17`"]
    #[inline]
    pub fn is_v17(&self) -> bool {
        *self == THRESHOLDR::V17
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline]
    pub fn is_v18(&self) -> bool {
        *self == THRESHOLDR::V18
    }
    #[doc = "Checks if the value of the field is `V19`"]
    #[inline]
    pub fn is_v19(&self) -> bool {
        *self == THRESHOLDR::V19
    }
    #[doc = "Checks if the value of the field is `V20`"]
    #[inline]
    pub fn is_v20(&self) -> bool {
        *self == THRESHOLDR::V20
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline]
    pub fn is_v21(&self) -> bool {
        *self == THRESHOLDR::V21
    }
    #[doc = "Checks if the value of the field is `V22`"]
    #[inline]
    pub fn is_v22(&self) -> bool {
        *self == THRESHOLDR::V22
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline]
    pub fn is_v23(&self) -> bool {
        *self == THRESHOLDR::V23
    }
    #[doc = "Checks if the value of the field is `V24`"]
    #[inline]
    pub fn is_v24(&self) -> bool {
        *self == THRESHOLDR::V24
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline]
    pub fn is_v25(&self) -> bool {
        *self == THRESHOLDR::V25
    }
    #[doc = "Checks if the value of the field is `V26`"]
    #[inline]
    pub fn is_v26(&self) -> bool {
        *self == THRESHOLDR::V26
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLDR::V27
    }
    #[doc = "Checks if the value of the field is `V28`"]
    #[inline]
    pub fn is_v28(&self) -> bool {
        *self == THRESHOLDR::V28
    }
}
#[doc = "Possible values of the field `THRESHOLDVDDH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRESHOLDVDDHR {
    #[doc = "Set threshold to 2.7 V"]
    V27,
    #[doc = "Set threshold to 2.8 V"]
    V28,
    #[doc = "Set threshold to 2.9 V"]
    V29,
    #[doc = "Set threshold to 3.0 V"]
    V30,
    #[doc = "Set threshold to 3.1 V"]
    V31,
    #[doc = "Set threshold to 3.2 V"]
    V32,
    #[doc = "Set threshold to 3.3 V"]
    V33,
    #[doc = "Set threshold to 3.4 V"]
    V34,
    #[doc = "Set threshold to 3.5 V"]
    V35,
    #[doc = "Set threshold to 3.6 V"]
    V36,
    #[doc = "Set threshold to 3.7 V"]
    V37,
    #[doc = "Set threshold to 3.8 V"]
    V38,
    #[doc = "Set threshold to 3.9 V"]
    V39,
    #[doc = "Set threshold to 4.0 V"]
    V40,
    #[doc = "Set threshold to 4.1 V"]
    V41,
    #[doc = "Set threshold to 4.2 V"]
    V42,
}
impl THRESHOLDVDDHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            THRESHOLDVDDHR::V27 => 0,
            THRESHOLDVDDHR::V28 => 1,
            THRESHOLDVDDHR::V29 => 2,
            THRESHOLDVDDHR::V30 => 3,
            THRESHOLDVDDHR::V31 => 4,
            THRESHOLDVDDHR::V32 => 5,
            THRESHOLDVDDHR::V33 => 6,
            THRESHOLDVDDHR::V34 => 7,
            THRESHOLDVDDHR::V35 => 8,
            THRESHOLDVDDHR::V36 => 9,
            THRESHOLDVDDHR::V37 => 10,
            THRESHOLDVDDHR::V38 => 11,
            THRESHOLDVDDHR::V39 => 12,
            THRESHOLDVDDHR::V40 => 13,
            THRESHOLDVDDHR::V41 => 14,
            THRESHOLDVDDHR::V42 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> THRESHOLDVDDHR {
        match value {
            0 => THRESHOLDVDDHR::V27,
            1 => THRESHOLDVDDHR::V28,
            2 => THRESHOLDVDDHR::V29,
            3 => THRESHOLDVDDHR::V30,
            4 => THRESHOLDVDDHR::V31,
            5 => THRESHOLDVDDHR::V32,
            6 => THRESHOLDVDDHR::V33,
            7 => THRESHOLDVDDHR::V34,
            8 => THRESHOLDVDDHR::V35,
            9 => THRESHOLDVDDHR::V36,
            10 => THRESHOLDVDDHR::V37,
            11 => THRESHOLDVDDHR::V38,
            12 => THRESHOLDVDDHR::V39,
            13 => THRESHOLDVDDHR::V40,
            14 => THRESHOLDVDDHR::V41,
            15 => THRESHOLDVDDHR::V42,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLDVDDHR::V27
    }
    #[doc = "Checks if the value of the field is `V28`"]
    #[inline]
    pub fn is_v28(&self) -> bool {
        *self == THRESHOLDVDDHR::V28
    }
    #[doc = "Checks if the value of the field is `V29`"]
    #[inline]
    pub fn is_v29(&self) -> bool {
        *self == THRESHOLDVDDHR::V29
    }
    #[doc = "Checks if the value of the field is `V30`"]
    #[inline]
    pub fn is_v30(&self) -> bool {
        *self == THRESHOLDVDDHR::V30
    }
    #[doc = "Checks if the value of the field is `V31`"]
    #[inline]
    pub fn is_v31(&self) -> bool {
        *self == THRESHOLDVDDHR::V31
    }
    #[doc = "Checks if the value of the field is `V32`"]
    #[inline]
    pub fn is_v32(&self) -> bool {
        *self == THRESHOLDVDDHR::V32
    }
    #[doc = "Checks if the value of the field is `V33`"]
    #[inline]
    pub fn is_v33(&self) -> bool {
        *self == THRESHOLDVDDHR::V33
    }
    #[doc = "Checks if the value of the field is `V34`"]
    #[inline]
    pub fn is_v34(&self) -> bool {
        *self == THRESHOLDVDDHR::V34
    }
    #[doc = "Checks if the value of the field is `V35`"]
    #[inline]
    pub fn is_v35(&self) -> bool {
        *self == THRESHOLDVDDHR::V35
    }
    #[doc = "Checks if the value of the field is `V36`"]
    #[inline]
    pub fn is_v36(&self) -> bool {
        *self == THRESHOLDVDDHR::V36
    }
    #[doc = "Checks if the value of the field is `V37`"]
    #[inline]
    pub fn is_v37(&self) -> bool {
        *self == THRESHOLDVDDHR::V37
    }
    #[doc = "Checks if the value of the field is `V38`"]
    #[inline]
    pub fn is_v38(&self) -> bool {
        *self == THRESHOLDVDDHR::V38
    }
    #[doc = "Checks if the value of the field is `V39`"]
    #[inline]
    pub fn is_v39(&self) -> bool {
        *self == THRESHOLDVDDHR::V39
    }
    #[doc = "Checks if the value of the field is `V40`"]
    #[inline]
    pub fn is_v40(&self) -> bool {
        *self == THRESHOLDVDDHR::V40
    }
    #[doc = "Checks if the value of the field is `V41`"]
    #[inline]
    pub fn is_v41(&self) -> bool {
        *self == THRESHOLDVDDHR::V41
    }
    #[doc = "Checks if the value of the field is `V42`"]
    #[inline]
    pub fn is_v42(&self) -> bool {
        *self == THRESHOLDVDDHR::V42
    }
}
#[doc = "Values that can be written to the field `POF`"]
pub enum POFW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl POFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POFW::DISABLED => false,
            POFW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POFW<'a> {
    w: &'a mut W,
}
impl<'a> _POFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POFW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POFW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `THRESHOLD`"]
pub enum THRESHOLDW {
    #[doc = "Set threshold to 1.7 V"]
    V17,
    #[doc = "Set threshold to 1.8 V"]
    V18,
    #[doc = "Set threshold to 1.9 V"]
    V19,
    #[doc = "Set threshold to 2.0 V"]
    V20,
    #[doc = "Set threshold to 2.1 V"]
    V21,
    #[doc = "Set threshold to 2.2 V"]
    V22,
    #[doc = "Set threshold to 2.3 V"]
    V23,
    #[doc = "Set threshold to 2.4 V"]
    V24,
    #[doc = "Set threshold to 2.5 V"]
    V25,
    #[doc = "Set threshold to 2.6 V"]
    V26,
    #[doc = "Set threshold to 2.7 V"]
    V27,
    #[doc = "Set threshold to 2.8 V"]
    V28,
}
impl THRESHOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            THRESHOLDW::V17 => 4,
            THRESHOLDW::V18 => 5,
            THRESHOLDW::V19 => 6,
            THRESHOLDW::V20 => 7,
            THRESHOLDW::V21 => 8,
            THRESHOLDW::V22 => 9,
            THRESHOLDW::V23 => 10,
            THRESHOLDW::V24 => 11,
            THRESHOLDW::V25 => 12,
            THRESHOLDW::V26 => 13,
            THRESHOLDW::V27 => 14,
            THRESHOLDW::V28 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _THRESHOLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THRESHOLDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set threshold to 1.7 V"]
    #[inline]
    pub fn v17(self) -> &'a mut W {
        self.variant(THRESHOLDW::V17)
    }
    #[doc = "Set threshold to 1.8 V"]
    #[inline]
    pub fn v18(self) -> &'a mut W {
        self.variant(THRESHOLDW::V18)
    }
    #[doc = "Set threshold to 1.9 V"]
    #[inline]
    pub fn v19(self) -> &'a mut W {
        self.variant(THRESHOLDW::V19)
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline]
    pub fn v20(self) -> &'a mut W {
        self.variant(THRESHOLDW::V20)
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline]
    pub fn v21(self) -> &'a mut W {
        self.variant(THRESHOLDW::V21)
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline]
    pub fn v22(self) -> &'a mut W {
        self.variant(THRESHOLDW::V22)
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline]
    pub fn v23(self) -> &'a mut W {
        self.variant(THRESHOLDW::V23)
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline]
    pub fn v24(self) -> &'a mut W {
        self.variant(THRESHOLDW::V24)
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline]
    pub fn v25(self) -> &'a mut W {
        self.variant(THRESHOLDW::V25)
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline]
    pub fn v26(self) -> &'a mut W {
        self.variant(THRESHOLDW::V26)
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLDW::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline]
    pub fn v28(self) -> &'a mut W {
        self.variant(THRESHOLDW::V28)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `THRESHOLDVDDH`"]
pub enum THRESHOLDVDDHW {
    #[doc = "Set threshold to 2.7 V"]
    V27,
    #[doc = "Set threshold to 2.8 V"]
    V28,
    #[doc = "Set threshold to 2.9 V"]
    V29,
    #[doc = "Set threshold to 3.0 V"]
    V30,
    #[doc = "Set threshold to 3.1 V"]
    V31,
    #[doc = "Set threshold to 3.2 V"]
    V32,
    #[doc = "Set threshold to 3.3 V"]
    V33,
    #[doc = "Set threshold to 3.4 V"]
    V34,
    #[doc = "Set threshold to 3.5 V"]
    V35,
    #[doc = "Set threshold to 3.6 V"]
    V36,
    #[doc = "Set threshold to 3.7 V"]
    V37,
    #[doc = "Set threshold to 3.8 V"]
    V38,
    #[doc = "Set threshold to 3.9 V"]
    V39,
    #[doc = "Set threshold to 4.0 V"]
    V40,
    #[doc = "Set threshold to 4.1 V"]
    V41,
    #[doc = "Set threshold to 4.2 V"]
    V42,
}
impl THRESHOLDVDDHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            THRESHOLDVDDHW::V27 => 0,
            THRESHOLDVDDHW::V28 => 1,
            THRESHOLDVDDHW::V29 => 2,
            THRESHOLDVDDHW::V30 => 3,
            THRESHOLDVDDHW::V31 => 4,
            THRESHOLDVDDHW::V32 => 5,
            THRESHOLDVDDHW::V33 => 6,
            THRESHOLDVDDHW::V34 => 7,
            THRESHOLDVDDHW::V35 => 8,
            THRESHOLDVDDHW::V36 => 9,
            THRESHOLDVDDHW::V37 => 10,
            THRESHOLDVDDHW::V38 => 11,
            THRESHOLDVDDHW::V39 => 12,
            THRESHOLDVDDHW::V40 => 13,
            THRESHOLDVDDHW::V41 => 14,
            THRESHOLDVDDHW::V42 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THRESHOLDVDDHW<'a> {
    w: &'a mut W,
}
impl<'a> _THRESHOLDVDDHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THRESHOLDVDDHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline]
    pub fn v28(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V28)
    }
    #[doc = "Set threshold to 2.9 V"]
    #[inline]
    pub fn v29(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V29)
    }
    #[doc = "Set threshold to 3.0 V"]
    #[inline]
    pub fn v30(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V30)
    }
    #[doc = "Set threshold to 3.1 V"]
    #[inline]
    pub fn v31(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V31)
    }
    #[doc = "Set threshold to 3.2 V"]
    #[inline]
    pub fn v32(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V32)
    }
    #[doc = "Set threshold to 3.3 V"]
    #[inline]
    pub fn v33(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V33)
    }
    #[doc = "Set threshold to 3.4 V"]
    #[inline]
    pub fn v34(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V34)
    }
    #[doc = "Set threshold to 3.5 V"]
    #[inline]
    pub fn v35(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V35)
    }
    #[doc = "Set threshold to 3.6 V"]
    #[inline]
    pub fn v36(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V36)
    }
    #[doc = "Set threshold to 3.7 V"]
    #[inline]
    pub fn v37(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V37)
    }
    #[doc = "Set threshold to 3.8 V"]
    #[inline]
    pub fn v38(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V38)
    }
    #[doc = "Set threshold to 3.9 V"]
    #[inline]
    pub fn v39(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V39)
    }
    #[doc = "Set threshold to 4.0 V"]
    #[inline]
    pub fn v40(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V40)
    }
    #[doc = "Set threshold to 4.1 V"]
    #[inline]
    pub fn v41(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V41)
    }
    #[doc = "Set threshold to 4.2 V"]
    #[inline]
    pub fn v42(self) -> &'a mut W {
        self.variant(THRESHOLDVDDHW::V42)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Enable or disable power failure warning"]
    #[inline]
    pub fn pof(&self) -> POFR {
        POFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:4 - Power-fail comparator threshold setting. This setting applies both for normal voltage mode (supply connected to both VDD and VDDH) and high voltage mode (supply connected to VDDH only). Values 0-3 set threshold below 1.7 V and should not be used as brown out detection will be activated before power failure warning on such low voltages."]
    #[inline]
    pub fn threshold(&self) -> THRESHOLDR {
        THRESHOLDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Power-fail comparator threshold setting for high voltage mode (supply connected to VDDH only). This setting does not apply for normal voltage mode (supply connected to both VDD and VDDH)."]
    #[inline]
    pub fn thresholdvddh(&self) -> THRESHOLDVDDHR {
        THRESHOLDVDDHR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable or disable power failure warning"]
    #[inline]
    pub fn pof(&mut self) -> _POFW {
        _POFW { w: self }
    }
    #[doc = "Bits 1:4 - Power-fail comparator threshold setting. This setting applies both for normal voltage mode (supply connected to both VDD and VDDH) and high voltage mode (supply connected to VDDH only). Values 0-3 set threshold below 1.7 V and should not be used as brown out detection will be activated before power failure warning on such low voltages."]
    #[inline]
    pub fn threshold(&mut self) -> _THRESHOLDW {
        _THRESHOLDW { w: self }
    }
    #[doc = "Bits 8:11 - Power-fail comparator threshold setting for high voltage mode (supply connected to VDDH only). This setting does not apply for normal voltage mode (supply connected to both VDD and VDDH)."]
    #[inline]
    pub fn thresholdvddh(&mut self) -> _THRESHOLDVDDHW {
        _THRESHOLDVDDHW { w: self }
    }
}
