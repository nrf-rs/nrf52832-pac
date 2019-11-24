#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - The IRK that was used last time an address was resolved"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x0f) as u8)
    }
}
